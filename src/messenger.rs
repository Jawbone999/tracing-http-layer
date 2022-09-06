use reqwest::RequestBuilder;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tracing::{debug, error};

pub enum Message {
    Http(RequestBuilder),
    Stop,
}

/// The Messenger allows us to stop all HttpLayer background processes whenever we want.
pub struct Messenger {
    pub sender: UnboundedSender<Message>,
    pub handler: JoinHandle<()>,
}

impl Messenger {
    pub async fn stop(self) {
        _ = self.sender.send(Message::Stop);
        self.handler.await.unwrap();
    }
}

/// A worker function which sends the received HTTP requests.
pub async fn messenger(mut receiver: UnboundedReceiver<Message>) {
    // Loop so long as we keep receiving HTTP requests.
    while let Some(Message::Http(req)) = receiver.recv().await {
        match req.send().await {
            Ok(res) => debug!(?res),
            Err(err) => error!(?err),
        }
    }
}
