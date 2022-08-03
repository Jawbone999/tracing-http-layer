use std::{collections::HashMap, fmt::Display};

use reqwest::{Method, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};

pub enum Message {
    Http(HttpMessage),
    Stop,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpMessage {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) method: Method,

    #[serde_as(as = "DisplayFromStr")]
    pub(crate) url: Url,

    pub(crate) headers: HashMap<String, String>,

    pub(crate) json: Option<Value>,
}

impl HttpMessage {
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: HashMap::default(),
            json: None,
        }
    }

    pub fn header(mut self, k: String, v: String) -> Self {
        self.headers.insert(k, v);
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn json<T: Serialize>(mut self, data: &T) -> Self {
        self.json = Some(serde_json::to_value(data).unwrap());
        self
    }
}

impl Display for HttpMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
