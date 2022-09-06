use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::test;
use tracing::{info, warn};
use tracing_http_layer::{HttpLayer, IntoHttpTrace};
use tracing_subscriber::{prelude::*, EnvFilter};
use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorCode {
    code: u64,
    message: String,
}

pub async fn start_logging_server() {
    let logger = warp::post()
        .and(warp::path("log"))
        .and(warp::body::json())
        .map(|body: ErrorCode| {
            println!("[{}]: {}", body.code, body.message);
            warp::reply()
        });

    warp::serve(logger).run(([127, 0, 0, 1], 3000)).await
}

impl IntoHttpTrace for ErrorCode {
    fn handle_event(
        &self,
        client: &Client,
        message: &str,
        metadata: &HashMap<&str, Value>,
    ) -> Option<RequestBuilder> {
        let is_quiet = metadata
            .get("quiet")
            .and_then(|v| v.as_bool())
            .unwrap_or_default();

        let code = metadata.get("code").and_then(|v| v.as_u64()).unwrap_or(500);

        if is_quiet {
            None
        } else {
            let log = ErrorCode {
                code,
                message: message.to_owned(),
            };

            Some(client.post("http://localhost:3000/log").json(&log))
        }
    }
}

// RUST_LOG=tracing_http_layer=debug,logging_server=debug cargo test -- --nocapture
#[test]
async fn smoke() {
    let handle = tokio::spawn(start_logging_server());

    let (http_layer, messenger): (HttpLayer<ErrorCode>, _) = HttpLayer::new(None);
    let fmt_layer = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(http_layer)
        .init();

    let data = ErrorCode {
        code: 301,
        message: "Moved permanently".into(),
    };
    warn!(
        http_trace = data.trace(),
        "Something went wrong, we need to tell someone!"
    );

    let data = ErrorCode {
        code: 418,
        message: "I am a teapot".into(),
    };
    info!(
        http_trace = data.trace(),
        quiet = true,
        "Everything is fine, no need to tell anyone!"
    );

    messenger.stop().await;
    handle.abort();
}
