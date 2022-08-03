use crate::{
    config::HttpLayerBuilder,
    message::Message,
    messenger::{messenger, Messenger},
    HttpMessage,
};
use serde_json::Value::String;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{Event, Subscriber};
use tracing_bunyan_formatter::JsonStorage;
use tracing_subscriber::{layer::Context, Layer};

pub static HTTP_MESSAGE_FIELD_NAME: &str = "http_message";
pub static MESSAGE_FIELD_NAME: &'static str = "message";

pub struct HttpLayer {
    sender: UnboundedSender<Message>,
}

impl HttpLayer {
    pub fn builder() -> HttpLayerBuilder {
        HttpLayerBuilder::default()
    }

    pub fn new(config: HttpLayerBuilder) -> (Self, Messenger) {
        let (sender, receiver) = mpsc::unbounded_channel();

        (
            Self {
                sender: sender.clone(),
            },
            Messenger {
                sender,
                handler: tokio::spawn(messenger(receiver, config.client.unwrap_or_default())),
            },
        )
    }
}

impl<S> Layer<S> for HttpLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let current_span = ctx.lookup_current();
        let mut event_visitor = JsonStorage::default();
        event.record(&mut event_visitor);

        // Only continue if the field is set to true.
        match event_visitor
            .values()
            .get(HTTP_MESSAGE_FIELD_NAME)
            .map(|v| v.as_bool())
        {
            Some(Some(true)) => {}
            _ => return,
        }

        // Get the message, only continue if it is a valid HttpMessage.
        let http_message = event_visitor
            .values()
            .get(MESSAGE_FIELD_NAME)
            .map(|v| match v {
                String(s) => Some(s.as_str()),
                _ => None,
            });

        if let Some(Some(s)) = http_message {
            let msg: Result<HttpMessage, _> = serde_json::from_str(s);

            if let Ok(http) = msg {
                self.sender.send(Message::Http(http));
            }
        }
    }
}
