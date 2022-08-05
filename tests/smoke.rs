use std::collections::HashMap;

use reqwest::{Method, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::test;
use tracing::info;
use tracing_http_layer::{HttpConfig, HttpLayer, IntoHttpConfig};
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug, Serialize, Deserialize)]
struct SmokeJson {
    foo: String,
    meaning: u8,
}

impl IntoHttpConfig for SmokeJson {
    fn into_http_config(self) -> HttpConfig {
        HttpConfig {
            method: Method::POST,
            url: Url::parse("http://localhost:3000").unwrap(),
            headers: HashMap::default(),
            json: Some(json!({
                "foo": self.foo,
                "meaning": self.meaning,
            })),
        }
    }

    fn add_message(mut http_message: HttpConfig, message: &str) -> HttpConfig {
        let mut json = http_message.json.take().unwrap_or_default();

        json["message"] = Value::String(message.to_string());

        http_message.json = Some(json);

        http_message
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

    info!(
        http_message = data.into_http_config().into_trace(),
        "Hello!"
    );

    messenger.stop().await;
}
