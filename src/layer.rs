use crate::{
    messenger::{messenger, Message, Messenger},
    IntoHttpTrace,
};
use reqwest::Client;
use std::marker::PhantomData;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::{Event, Subscriber};
use tracing_bunyan_formatter::JsonStorage;
use tracing_subscriber::{layer::Context, Layer};

/// The name of the tracing field which contains an HttpTrace.
pub static HTTP_TRACE_FIELD_NAME: &str = "http_trace";

/// The name of the tracing field which contains the event message.
pub static MESSAGE_FIELD_NAME: &'static str = "message";

pub struct HttpLayer<T: IntoHttpTrace> {
    /// The channel to which all Messages are sent.
    sender: UnboundedSender<Message>,

    /// The `reqwest::Client` used to perform HTTP requests.
    client: Client,

    /// The type of data this HttpLayer will handle.
    _type: PhantomData<T>,
}

impl<T: IntoHttpTrace> HttpLayer<T> {
    /// Create a new HttpLayer which handles the given type of data.
    ///
    /// A `reqwest::Client` can optionally be provided, if a specific configuration is required.
    pub fn new(client: Option<Client>) -> (Self, Messenger) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let client = client.unwrap_or_default();

        (
            Self {
                sender: sender.clone(),
                client,
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

        // If the `http_trace` field is correctly present, then we should handle the data.
        let http_trace = match event_visitor
            .values()
            .get(HTTP_TRACE_FIELD_NAME)
            .map(|v| v.as_str().map(|s| serde_json::from_str::<T>(s)))
            .flatten()
        {
            Some(Ok(v)) => v,
            _ => return,
        };

        // We also need a message to send.
        let message = match event_visitor
            .values()
            .get(MESSAGE_FIELD_NAME)
            .map(|v| v.as_str())
            .flatten()
        {
            Some(s) => s,
            _ => return,
        };

        // If the type T returns a `reqwest::RequestBuilder`, then we need to send it to our sender.
        if let Some(req) = http_trace.handle_event(&self.client, message, event_visitor.values()) {
            _ = self.sender.send(Message::Http(req));
        }
    }
}
