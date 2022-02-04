
use rocket::{Request, Data};
use rocket::route::{Outcome, Handler, Route};
use rocket::http::{Method};
use rocket::data::Limits;
use rocket::get;
use std::sync::mpsc::{SyncSender};
use crate::types::{Message, TimeoutMessage, DirectiveMessage};

#[derive(Clone)]
pub struct MessageHandler {
    out: SyncSender<Message>
}

impl MessageHandler {
    pub fn new(out: SyncSender<Message>) -> MessageHandler {
        MessageHandler {
            out: out,
        }
    }

    async fn from_data<'r>(req: &'r Request<'_>, data: Data<'r>) -> Result<Message, String> {
        let limit = req.limits().get("json").unwrap_or(Limits::JSON);
        let string = match data.open(limit).into_string().await {
            Ok(s) if s.is_complete() => s.into_inner(),
            Ok(_) => {
                return Err(String::from("data limit exceeded"));
            },
            Err(e) => return Err(String::from(e.to_string())),
        };
        Message::from_str(&string)
    }
}

#[rocket::async_trait]
impl Handler for MessageHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        
        if let Ok(message) = MessageHandler::from_data(req, data).await {
            self.out.send(message).unwrap()
        }

        Outcome::from(req, "Ok")
    }
}

impl Into<Vec<Route>> for MessageHandler {
    fn into(self) -> Vec<Route> {
        vec![Route::new(Method::Post, "/", self)]
    }
}

#[derive(Clone)]
pub struct TimeoutHandler {
    out: SyncSender<TimeoutMessage>
}

impl TimeoutHandler {
    pub fn new(out: SyncSender<TimeoutMessage>) -> TimeoutHandler {
        TimeoutHandler {
            out: out,
        }
    }

    async fn from_data<'r>(req: &'r Request<'_>, data: Data<'r>) -> Result<TimeoutMessage, String> {
        let limit = req.limits().get("json").unwrap_or(Limits::JSON);
        let string = match data.open(limit).into_string().await {
            Ok(s) if s.is_complete() => s.into_inner(),
            Ok(_) => {
                return Err(String::from("data limit exceeded"));
            },
            Err(e) => return Err(String::from(e.to_string())),
        };
        TimeoutMessage::from_str(&string)
    }
}

#[rocket::async_trait]
impl Handler for TimeoutHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        
        if let Ok(message) = TimeoutHandler::from_data(req, data).await {
            self.out.send(message).unwrap()
        }

        Outcome::from(req, "Ok")
    }
}

impl Into<Vec<Route>> for TimeoutHandler {
    fn into(self) -> Vec<Route> {
        vec![Route::new(Method::Post, "/", self)]
    }
}

#[derive(Clone)]
pub struct DirectiveHandler {
    out: SyncSender<DirectiveMessage>
}

impl DirectiveHandler {
    pub fn new(out: SyncSender<DirectiveMessage>) -> DirectiveHandler {
        DirectiveHandler {
            out: out,
        }
    }

    async fn from_data<'r>(req: &'r Request<'_>, data: Data<'r>) -> Result<DirectiveMessage, String> {
        let limit = req.limits().get("json").unwrap_or(Limits::JSON);
        let string = match data.open(limit).into_string().await {
            Ok(s) if s.is_complete() => s.into_inner(),
            Ok(_) => {
                return Err(String::from("data limit exceeded"));
            },
            Err(e) => return Err(String::from(e.to_string())),
        };
        DirectiveMessage::from_str(&string)
    }
}

#[rocket::async_trait]
impl Handler for DirectiveHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        
        if let Ok(message) = DirectiveHandler::from_data(req, data).await {
            self.out.send(message).unwrap()
        }

        Outcome::from(req, "Ok")
    }
}

impl Into<Vec<Route>> for DirectiveHandler {
    fn into(self) -> Vec<Route> {
        vec![Route::new(Method::Post, "/", self)]
    }
}

#[get("/health")]
pub fn health() -> String {
    String::from("Ok!")
}