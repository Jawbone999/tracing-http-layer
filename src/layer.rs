use std::marker::PhantomData;

use crate::{
    config::HttpLayerBuilder,
    message::Message,
    messenger::{messenger, Messenger},
    HttpConfig, IntoHttpConfig,
};
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{Event, Subscriber};
use tracing_bunyan_formatter::JsonStorage;
use tracing_subscriber::{layer::Context, Layer};

pub static HTTP_MESSAGE_FIELD_NAME: &str = "http_message";
pub static MESSAGE_FIELD_NAME: &'static str = "message";

pub struct HttpLayer<T: IntoHttpConfig> {
    sender: UnboundedSender<Message>,
    _type: PhantomData<T>,
}

impl<T: IntoHttpConfig> HttpLayer<T> {
    pub fn builder() -> HttpLayerBuilder<T> {
        HttpLayerBuilder::default()
    }

    pub fn new(config: HttpLayerBuilder<T>) -> (Self, Messenger) {
        let (sender, receiver) = mpsc::unbounded_channel();

        (
            Self {
                sender: sender.clone(),
                _type: PhantomData,
            },
            Messenger {
                sender,
                handler: tokio::spawn(messenger(receiver, config.client.unwrap_or_default())),
            },
        )
    }
}

impl<S, T> Layer<S> for HttpLayer<T>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    T: IntoHttpConfig + 'static,
{
    fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
        let mut event_visitor = JsonStorage::default();
        event.record(&mut event_visitor);

        let http_message = match event_visitor
            .values()
            .get(HTTP_MESSAGE_FIELD_NAME)
            .map(|v| v.as_str().map(|s| serde_json::from_str::<HttpConfig>(s)))
        {
            Some(Some(Ok(v))) => v,
            _ => return,
        };

        let message = match event_visitor
            .values()
            .get(MESSAGE_FIELD_NAME)
            .map(|v| v.as_str())
        {
            Some(Some(s)) => s,
            _ => return,
        };

        let http = dbg!(T::add_message(http_message, message));

        let _ = self.sender.send(Message::Http(http));
    }
}
