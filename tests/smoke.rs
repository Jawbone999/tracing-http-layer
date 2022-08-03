use reqwest::{Method, Url};
use serde::Serialize;
use tokio::test;
use tracing::debug;
use tracing_http_layer::{HttpLayer, HttpMessage};
use tracing_subscriber::prelude::*;

#[derive(Serialize)]
struct SmokeJson {
    foo: String,
    meaning: u8,
}

#[test]
async fn smoke() {
    let (layer, messenger) = HttpLayer::builder().build();

    tracing_subscriber::registry().with(layer).init();

    let data = SmokeJson {
        foo: "bar".to_string(),
        meaning: 42,
    };

    let message = HttpMessage::new(
        Method::POST,
        Url::parse("http://localhost:3000/test").unwrap(),
    )
    .json(&data);

    debug!(http_message = true, "{message}");

    messenger.stop().await;
}
