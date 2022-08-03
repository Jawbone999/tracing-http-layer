use reqwest::Client;

use crate::{layer::HttpLayer, messenger::Messenger};

#[derive(Default)]
pub struct HttpLayerBuilder {
    pub client: Option<Client>,
}

impl HttpLayerBuilder {
    pub fn client(mut self) -> Self {
        self.client = Some(Client::new());
        self
    }

    pub fn build(self) -> (HttpLayer, Messenger) {
        HttpLayer::new(self)
    }
}
