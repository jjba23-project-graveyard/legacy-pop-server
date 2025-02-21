// pop-server - a puppet server, at your command

// Copyright (C) 2025 Josep Bigorra (jjba23) <jjbigorra@gmail.com>

// pop-server is free software: you can redistribute it and/or modify
// it under the terms of the Lesser GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// pop-server is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the Lesser GNU General Public License
// along with pop-server.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use http_body_util::Full;

use behavior::{fetch_behaviour, register_behaviour};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use log::{error, info};
use model::RouteBehavior;
use tokio::net::TcpListener;

mod behavior;
mod model;

//
//
//
struct PopServer {}
impl PopServer {
    pub const DEFAULT_PORT: u16 = 50088;
    pub const DEFAULT_REGISTER_ROUTE: &str = "/api/v2/pop-server/register-behavior";
}

async fn pop_server(
    request: Request<hyper::body::Incoming>,
    bs: &Arc<Mutex<HashMap<String, RouteBehavior>>>,
    register_route: String,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    info!("incoming request: {:?}", request);

    match (request.method(), request.uri().path()) {
        (&Method::POST, route) if route == register_route => register_behaviour(request, bs).await,
        _ => fetch_behaviour(request, bs).await,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    info!("\npop-server starting up\n");

    let port: u16 = env::var("POP_SERVER_LISTEN_PORT")
        .map_err(|e| e.to_string())
        .and_then(|v| v.parse::<u16>().map_err(|e| e.to_string()))
        .unwrap_or(PopServer::DEFAULT_PORT);

    let register_route: String = env::var("POP_SERVER_REGISTER_ROUTE")
        .map_err(|e| e.to_string())
        .unwrap_or(PopServer::DEFAULT_REGISTER_ROUTE.to_string());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("socket address: {}", addr);

    let listener = TcpListener::bind(addr).await?;

    let behaviors: Arc<Mutex<HashMap<String, RouteBehavior>>> =
        Arc::new(Mutex::new(HashMap::new()));

    loop {
        let reg_route: String = register_route.clone();
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let behaviors = Arc::clone(&behaviors);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|request| pop_server(request, &behaviors, reg_route.clone())),
                )
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
