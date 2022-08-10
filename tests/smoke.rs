use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::test;
use tracing::info;
use tracing_http_layer::{HttpLayer, IntoHttpTrace};
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug, Serialize, Deserialize)]
struct SmokeJson {
    foo: String,
    meaning: u8,
}

impl IntoHttpTrace for SmokeJson {
    fn handle_event(
        &self,
        client: &Client,
        message: &str,
        metadata: &HashMap<&str, Value>,
    ) -> Option<RequestBuilder> {
        dbg!(metadata);
        let x = client
            .post("http://localhost:3000/smoke")
            .header("message", message)
            .json(self);

        Some(x)
    }
}

#[test]
async fn smoke() {
    let (http_layer, messenger): (HttpLayer<SmokeJson>, _) = HttpLayer::new(None);
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

    info!(http_trace = data.trace(), checked = true, "Hello!");

    messenger.stop().await;
}
