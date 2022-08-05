use std::marker::PhantomData;

use crate::{layer::HttpLayer, messenger::Messenger, IntoHttpMessage};
use reqwest::Client;

pub struct HttpLayerBuilder<T: IntoHttpMessage> {
    pub client: Option<Client>,
    _type: PhantomData<T>,
}

impl<T: IntoHttpMessage> Default for HttpLayerBuilder<T> {
    fn default() -> Self {
        Self {
            client: None,
            _type: PhantomData,
        }
    }
}

impl<T: IntoHttpMessage> HttpLayerBuilder<T> {
    pub fn client(mut self) -> Self {
        self.client = Some(Client::new());
        self
    }

    pub fn build(self) -> (HttpLayer<T>, Messenger) {
        HttpLayer::new(self)
    }
}
