use crate::HttpConfig;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait IntoHttpConfig: Debug + Serialize + DeserializeOwned {
    fn into_http_config(&self) -> HttpConfig;

    fn add_message(http_message: HttpConfig, message: &str) -> HttpConfig;
}
