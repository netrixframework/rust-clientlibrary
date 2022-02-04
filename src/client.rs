use std::net::{SocketAddr};
use reqwest::{Client as ReqwestClient};
use crate::listener::NetrixListener;
use crate::types::ReplicaID;

pub type DirectiveResult = Result<(), String>;

pub trait DirectiveHandler {
    fn stop(&self) -> DirectiveResult;
    fn start(&self) -> DirectiveResult;
    fn reset(&self) -> DirectiveResult;
}

pub struct ClientConfig {
    pub replica_id: ReplicaID,
    pub netrix_addr: String,
    pub adv_addr: String,
    pub listen_addr: SocketAddr
}

pub struct Client<T: DirectiveHandler> {
    config: ClientConfig,
    netrix_listener: NetrixListener,
    directive_handler: T,
    client: ReqwestClient,
}

impl<T: DirectiveHandler> Client<T> {
    pub fn new(config: ClientConfig, directive_handler: T) -> Client<T> {
        let rclient = ReqwestClient::builder().pool_max_idle_per_host(1).build().unwrap();

        Client {
            config: config,
            netrix_listener: NetrixListener::new(20),
            directive_handler: directive_handler,
            client: rclient,
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        self.netrix_listener.run().await
    }
}