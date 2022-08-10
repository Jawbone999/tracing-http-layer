use crate::{
    message::Message,
    messenger::{messenger, Messenger},
    IntoHttpTrace,
};
use reqwest::Client;
use std::marker::PhantomData;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{Event, Subscriber};
use tracing_bunyan_formatter::JsonStorage;
use tracing_subscriber::{layer::Context, Layer};

pub static HTTP_TRACE_FIELD_NAME: &str = "http_trace";
pub static MESSAGE_FIELD_NAME: &'static str = "message";

pub struct HttpLayer<T: IntoHttpTrace> {
    sender: UnboundedSender<Message>,
    client: Client,
    _type: PhantomData<T>,
}

impl<T: IntoHttpTrace> HttpLayer<T> {
    pub fn new(client: Option<Client>) -> (Self, Messenger) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let client = client.unwrap_or_default();

        (
            Self {
                sender: sender.clone(),
                client: client.clone(),
                _type: PhantomData,
            },
            Messenger {
                sender,
                handler: tokio::spawn(messenger(receiver)),
            },
        )
    }
}

impl<S, T> Layer<S> for HttpLayer<T>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    T: IntoHttpTrace + 'static,
{
    fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
        let mut event_visitor = JsonStorage::default();
        event.record(&mut event_visitor);

        let http_trace = match event_visitor
            .values()
            .get(HTTP_TRACE_FIELD_NAME)
            .map(|v| v.as_str().map(|s| serde_json::from_str::<T>(s)))
        {
            Some(Some(Ok(v))) => dbg!(v),
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

        if let Some(req) = http_trace.handle_event(&self.client, message, event_visitor.values()) {
            let _ = self.sender.send(Message::Http(req));
        }
    }
}
