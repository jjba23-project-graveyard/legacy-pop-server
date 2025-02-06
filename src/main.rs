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
use paris::{error, info};
use tokio::net::TcpListener;

mod model;

static mut BEHAVIORS: Vec<RouteBehavior> = vec![];

fn get_header(headers: &hyper::HeaderMap, key: &str) -> Option<String> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

async fn register_behaviour(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    info!("Starting registration of behavior");

    // Extract headers safely
    let wanted_method = get_header(request.headers(), "x-pop-server-method").unwrap_or_default();
    let wanted_path = get_header(request.headers(), "x-pop-server-path").unwrap_or_default();
    let wanted_content_type = get_header(request.headers(), "x-pop-server-content-type")
        .unwrap_or_else(|| "application/json".to_string());
    let wanted_status: u16 = get_header(request.headers(), "x-pop-server-status")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    // Read the request body safely
    let body_bytes = request
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();

    let wanted_body = String::from_utf8_lossy(&body_bytes).into_owned();

    // Store the behavior (assuming BEHAVIORS is a global mutable variable)
    unsafe {
        BEHAVIORS.push(RouteBehavior {
            method: wanted_method,
            status: wanted_status,
            path: wanted_path,
            content_type: wanted_content_type,
            body: wanted_body,
        });
    }

    Ok(Response::new(Full::new(Bytes::from(
        "Registered behavior successfully!",
    ))))
}

async fn fetch_behaviour(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    info!("Fetching behavior");

    let behavior = unsafe {
        BEHAVIORS
            .iter()
            .find(|b| b.method == request.method().as_str() && b.path == request.uri().path())
    };

    match behavior {
        Some(behavior) => {
            info!("Found behavior: {:?}", behavior);

            Response::builder()
                .status(behavior.status)
                .header("content-type", &behavior.content_type)
                .body(Full::new(Bytes::from(behavior.body.clone())))
        }
        None => {
            info!("No match found!");

            Response::builder().status(404).body(Full::new(Bytes::from(
                "No match found, using default behavior!",
            )))
        }
    }
}

async fn pop_server(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
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
