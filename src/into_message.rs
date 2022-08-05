use crate::HttpMessage;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait IntoHttpMessage: Debug + Serialize + DeserializeOwned {
    fn into_http_message(self, message: &str) -> HttpMessage;

    fn into_trace(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
