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
use std::sync::{Arc, Mutex};

use http_body_util::{BodyExt, Full};

use hyper::body::Bytes;
use hyper::Request;
use hyper::Response;
use log::{error, info};

use crate::model::RouteBehavior;

fn get_header(headers: &hyper::HeaderMap, key: &str) -> Option<String> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

fn make_behavior_key(behavior: RouteBehavior) -> String {
    format!(
        "{}::{}::{}",
        behavior.request_method,
        behavior.request_path,
        behavior.request_content_type.unwrap_or("nil".to_string())
    )
}

fn make_behavior_key_from_request(request: &Request<hyper::body::Incoming>) -> String {
    format!(
        "{}::{}::{}",
        request.method(),
        request.uri().path(),
        request
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(String::from)
            .ok_or("nil")
            .unwrap_or("nil".to_string())
    )
}

pub async fn register_behaviour(
    request: Request<hyper::body::Incoming>,
    behaviors: &Arc<Mutex<HashMap<String, RouteBehavior>>>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    info!("starting registration of behavior");

    let request_method =
        get_header(request.headers(), "x-pop-server-request-method").unwrap_or("GET".to_string());
    let request_path =
        get_header(request.headers(), "x-pop-server-request-path").unwrap_or("/".to_string());
    let request_content_type = get_header(request.headers(), "x-pop-server-request-content-type");
    let response_content_type = get_header(request.headers(), "x-pop-server-response-content-type")
        .unwrap_or("application/json".to_string());
    let response_status: u16 = get_header(request.headers(), "x-pop-server-response-status")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    let body_bytes = request
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();

    let wanted_body = String::from_utf8_lossy(&body_bytes).into_owned();

    let route_behavior = RouteBehavior {
        request_method,
        response_status,
        request_path,
        request_content_type,
        response_content_type,
        response_body: wanted_body,
    };
    let k = make_behavior_key(route_behavior.clone());
    let behaviors = behaviors.lock();

    match behaviors {
        Ok(mut b) => {
            b.insert(k.clone(), route_behavior.clone());
            ()
        }
        Err(_) => error!("could not acquire mutex lock"),
    }

    info!("registered behavior successfully: {}", k.clone());

    Ok(Response::new(Full::new(Bytes::from(
        "registered behavior successfully!",
    ))))
}

pub async fn fetch_behaviour(
    request: Request<hyper::body::Incoming>,
    behaviors: &Arc<Mutex<HashMap<String, RouteBehavior>>>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    let behaviors = Arc::clone(behaviors);
    let behaviors_map = behaviors.lock();
    let k = make_behavior_key_from_request(&request).clone();
    info!("fetching behavior, looking for key: `{}`", k);

    let found_behavior = behaviors_map.ok().and_then(|b| b.get(k.as_str()).cloned());

    match found_behavior {
        Some(behavior) => {
            info!("found behavior: {:?}", behavior);

            Response::builder()
                .status(behavior.response_status)
                .header("content-type", behavior.response_content_type.clone())
                .body(Full::new(Bytes::from(behavior.response_body.clone())))
        }
        None => {
            error!("no match found, using default behavior");

            Response::builder().status(404).body(Full::new(Bytes::from(
                "no match found, using default behavior",
            )))
        }
    }
}
