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
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use model::RouteBehavior;
use paris::info;
use tokio::net::TcpListener;

mod model;

static mut BEHAVIORS: Vec<RouteBehavior> = vec![];

async fn register_behaviour(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    info!("starting registration of behavior");

    let wanted_method = request
        .headers()
        .get("x-pop-server-method")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let wanted_path = request
        .headers()
        .get("x-pop-server-path")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let wanted_content_type = request
        .headers()
        .get("x-pop-server-content-type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let wanted_status = request
        .headers()
        .get("x-pop-server-status")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

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
            status: wanted_status.parse().unwrap_or(200),
            path: wanted_path,
            content_type: wanted_content_type,
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

    let b = unsafe {
        BEHAVIORS.iter().find(|b| {
            b.method == request.method().as_str().to_string()
                && b.path == request.uri().path().to_string()
        })
    };

    match b {
        Some(behavior) => {
            info!("found behavior: {:?}", behavior);

            Ok(Response::builder()
                .status(behavior.status)
                .header("content-type", behavior.content_type.to_string())
                .body(Full::new(Bytes::from(behavior.body.clone())))
                .unwrap())
        }
        _ => {
            info!("no match found!");
            Ok(Response::new(Full::new(Bytes::from(
                "no match found, using default behavior!",
            ))))
        }
    }
}

async fn pop_server(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    info!("incoming request: {:?}", request);

    match (request.method(), request.uri().path()) {
        (&Method::POST, "/pop-server/register-behavior") => register_behaviour(request).await,
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
