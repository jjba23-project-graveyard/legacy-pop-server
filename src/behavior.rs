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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use http_body_util::{BodyExt, Full};

use hyper::body::Bytes;
use hyper::Request;
use hyper::Response;
use paris::{error, info, success};

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
        behavior.method, behavior.path, behavior.content_type
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
            .ok_or("application/json")
            .unwrap_or("application/json".to_string())
    )
}

pub async fn register_behaviour(
    request: Request<hyper::body::Incoming>,
    behaviors: &Arc<Mutex<HashMap<String, RouteBehavior>>>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    info!("starting registration of behavior");

    let wanted_method = get_header(request.headers(), "x-pop-server-method").unwrap_or_default();
    let wanted_path = get_header(request.headers(), "x-pop-server-path").unwrap_or_default();
    let wanted_content_type = get_header(request.headers(), "x-pop-server-content-type")
        .unwrap_or_else(|| "application/json".to_string());
    let wanted_status: u16 = get_header(request.headers(), "x-pop-server-status")
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

    let mut behaviors = behaviors.lock().unwrap();
    let b = RouteBehavior {
        method: wanted_method,
        status: wanted_status,
        path: wanted_path,
        content_type: wanted_content_type,
        body: wanted_body,
    };
    let k = make_behavior_key(b.clone());

    behaviors.insert(k.clone(), b.clone());
    success!("registered behavior successfully: {}", k.clone());

    Ok(Response::new(Full::new(Bytes::from(
        "registered behavior successfully!",
    ))))
}

pub async fn fetch_behaviour(
    request: Request<hyper::body::Incoming>,
    behaviors: &Arc<Mutex<HashMap<String, RouteBehavior>>>,
) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    let behaviors = Arc::clone(behaviors);
    let behaviors_map = behaviors.lock().unwrap();
    let k = make_behavior_key_from_request(&request).clone();
    info!("fetching behavior, looking for key: `{}`", k);
    let behavior = behaviors_map.get(k.as_str());

    match behavior {
        Some(behavior) => {
            success!("found behavior: {:?}", behavior);

            Response::builder()
                .status(behavior.status)
                .header("content-type", &behavior.content_type)
                .body(Full::new(Bytes::from(behavior.body.clone())))
        }
        None => {
            error!("no match found, using default behavior");

            Response::builder().status(404).body(Full::new(Bytes::from(
                "no match found, using default behavior",
            )))
        }
    }
}
