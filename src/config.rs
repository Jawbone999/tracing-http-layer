use crate::{layer::HttpLayer, messenger::Messenger, IntoHttpConfig};
use reqwest::Client;
use std::marker::PhantomData;

pub struct HttpLayerBuilder<T: IntoHttpConfig> {
    pub client: Option<Client>,
    _type: PhantomData<T>,
}

impl<T: IntoHttpConfig> Default for HttpLayerBuilder<T> {
    fn default() -> Self {
        Self {
            client: None,
            _type: PhantomData,
        }
    }
}

impl<T: IntoHttpConfig> HttpLayerBuilder<T> {
    pub fn client(mut self) -> Self {
        self.client = Some(Client::new());
        self
    }

    pub fn build(self) -> (HttpLayer<T>, Messenger) {
        HttpLayer::new(self)
    }
}
