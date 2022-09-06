mod layer;
mod messenger;

use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

// Public API
pub use layer::HttpLayer;
pub use messenger::Messenger;

/// A trait for converting a value into an HttpTrace message.
///
/// The `trace` method should be left as default.
pub trait IntoHttpTrace: Debug + Serialize + DeserializeOwned {
    /// Tracing requires that all fields are strings.
    fn trace(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Construct a request based on the content of the tracing event, if desired.
    fn handle_event(
        &self,
        client: &Client,
        message: &str,
        metadata: &HashMap<&str, Value>,
    ) -> Option<RequestBuilder>;
}
