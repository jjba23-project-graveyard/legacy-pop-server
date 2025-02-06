// pop-server - a puppet server, at your command

// Copyright (C) 2025 Josep Bigorra (jjba23) <jjbigorra@gmail.com>

// pop-server is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// pop-server is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with pop-server.  If not, see <https://www.gnu.org/licenses/>.

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::{BodyExt, Full};

use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use model::RouteBehavior;
use paris::info;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod model;

static mut BEHAVIORS: Vec<RouteBehavior> = vec![];

async fn register_behaviour(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    info!("starting registration of behavior");

    let wanted_method = request.method().as_str().to_string();
    let wanted_path = request.uri().path().to_string();
    let request_body_bytes = request
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();

    let wanted_body: String = String::from_utf8(request_body_bytes.into_iter().collect()).unwrap();

    unsafe {
        BEHAVIORS.push(RouteBehavior {
            method: wanted_method,
            path: wanted_path,
            body: wanted_body,
        });
    };

    Ok(Response::new(Full::new(Bytes::from(
        "registered behavior successfully!",
    ))))
}

async fn fetch_behaviour(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    info!("fetching behavior");

    Ok(Response::new(Full::new(Bytes::from(
        "fetched behavior successfully!",
    ))))
}

async fn pop_server(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    info!("incoming request: {:?}", request);

    match request.method().as_str() {
        "POST" => match request.uri().path() {
            "/pop-server/register-behaviour" => register_behaviour(request).await,
            _ => fetch_behaviour(request).await,
        },
        _ => fetch_behaviour(request).await,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("\nstarting `pop-server`\n");

    let addr = SocketAddr::from(([127, 0, 0, 1], 50088));
    info!("socket address: {}", addr);

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(|request| pop_server(request)))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
