use crate::message::Message;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tracing::{debug, error};

pub struct Messenger {
    pub sender: UnboundedSender<Message>,
    pub handler: JoinHandle<()>,
}

impl Messenger {
    pub async fn stop(self) {
        let _ = self.sender.send(Message::Stop);
        self.handler.await.unwrap();
    }
}

pub async fn messenger(mut receiver: UnboundedReceiver<Message>) {
    while let Some(Message::Http(req)) = receiver.recv().await {
        match req.send().await {
            Ok(res) => debug!(?res),
            Err(err) => error!(?err),
        }
    }
}
