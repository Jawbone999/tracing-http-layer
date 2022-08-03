use crate::message::{HttpMessage, Message};
use reqwest::Client;
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

pub async fn messenger(mut receiver: UnboundedReceiver<Message>, client: Client) {
    while let Some(Message::Http(HttpMessage {
        method,
        url,
        headers,
        json,
    })) = receiver.recv().await
    {
        let mut req = client.request(method, url);

        for (key, value) in headers.into_iter() {
            req = req.header(key, value);
        }

        if let Some(data) = json {
            req = req.json(&data);
        }

        match req.send().await {
            Ok(res) => debug!(?res),
            Err(err) => error!(?err),
        }
    }
}
