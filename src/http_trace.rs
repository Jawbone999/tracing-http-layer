use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

pub trait IntoHttpTrace: Debug + Serialize + DeserializeOwned {
    fn trace(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn handle_event(
        &self,
        client: &Client,
        message: &str,
        metadata: &HashMap<&str, Value>,
    ) -> Option<RequestBuilder>;
}
