use std::collections::HashMap;

use reqwest::{Method, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::test;
use tracing::info;
use tracing_http_layer::{HttpLayer, HttpMessage, IntoHttpMessage};
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug, Serialize, Deserialize)]
struct SmokeJson {
    foo: String,
    meaning: u8,
}

impl IntoHttpMessage for SmokeJson {
    fn into_http_message(self, message: &str) -> HttpMessage {
        HttpMessage {
            method: Method::POST,
            url: Url::parse("http://localhost:3000").unwrap(),
            headers: HashMap::default(),
            json: Some(json!({
                "message": message,
                "foo": self.foo,
                "meaning": self.meaning,
            })),
        }
    }
}

#[test]
async fn smoke() {
    let (http_layer, messenger): (HttpLayer<SmokeJson>, _) = HttpLayer::builder().build();
    let fmt_layer = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(http_layer)
        .init();

    let data = SmokeJson {
        foo: "bar".to_string(),
        meaning: 42,
    };

    info!(http_message = data.into_trace(), "Hello!");

    messenger.stop().await;
}
