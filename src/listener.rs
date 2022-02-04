use tokio::runtime::Runtime;
use std::io::Error;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use rocket::routes;
use crate::handlers::{MessageHandler, TimeoutHandler, DirectiveHandler, health};
use crate::types::{Message, TimeoutMessage, DirectiveMessage};

pub struct NetrixListener {
    message_send: SyncSender<Message>,
    message_receive: Receiver<Message>,
    timeout_send: SyncSender<TimeoutMessage>,
    timeout_receive: Receiver<TimeoutMessage>,
    directive_send: SyncSender<DirectiveMessage>,
    directive_receive: Receiver<DirectiveMessage>
}

impl NetrixListener {
    pub fn new(chan_size: usize) -> NetrixListener {
        let (message_send, message_receive) = sync_channel(chan_size);
        let (timeout_send, timeout_receive) = sync_channel(chan_size);
        let (directive_send, directive_receive) = sync_channel(chan_size);
        NetrixListener {
            message_receive: message_receive,
            message_send: message_send,
            timeout_receive: timeout_receive,
            timeout_send: timeout_send,
            directive_receive: directive_receive,
            directive_send: directive_send,
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        let server = rocket::build().
            mount("/messages", MessageHandler::new(self.message_send.clone())).
            mount("/timeout", TimeoutHandler::new(self.timeout_send.clone())).
            mount("/directive", DirectiveHandler::new(self.directive_send.clone())).
            mount("/health", routes![health]);

        if let Err(_) = server.launch().await {
            return Err(String::from("server failed"))
        }
        Ok(())
    }

    pub fn message_receiver(self) -> Receiver<Message> {
        self.message_receive
    }

    pub fn directive_receiver(self) -> Receiver<DirectiveMessage> {
        self.directive_receive
    }
}