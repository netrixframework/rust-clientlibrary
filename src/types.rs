use rocket::serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

pub type ReplicaID = String;
pub type Error = String;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub from: ReplicaID,
    pub to: ReplicaID,
    pub data: Vec<u8>,
    pub id: String,
    pub intercept: bool
}

impl Message {
    pub fn from_str(s: &str) -> Result<Message, Error> {
        let res: serde_json::Result<Message> = serde_json::from_str(s);
        match res {
            Ok(message) => Ok(message),
            Err(_) => Err(String::from("failed to parse message from data"))
        }
    }

    pub fn to_str(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| format!("error marshalling timeout message: {}", e))
    }
}

pub struct TimeoutInfo {
    pub t: String,
    pub d: Duration,
}

impl TimeoutInfo {
    pub fn new(t: String, d: Duration) -> TimeoutInfo {
        TimeoutInfo {
            t: t,
            d: d,
        }
    }

    pub fn duration(&self) -> String {
        format!("{}ms", self.d.as_millis())
    }

    pub fn to_message(&self, replica_id: ReplicaID) -> TimeoutMessage {
        TimeoutMessage {
            t: self.t.clone(),
            d: self.duration(),
            replica: replica_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TimeoutMessage {
    pub t: String,
    pub d: String,
    pub replica: ReplicaID,
}

impl TimeoutMessage {
    pub fn from_str(s: &str) -> Result<TimeoutMessage, Error> {
        let res: serde_json::Result<TimeoutMessage> = serde_json::from_str(s);
        match res {
            Ok(message) => Ok(message),
            Err(_) => Err(String::from("failed to parse message from data"))
        }
    }

    pub fn to_str(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| format!("error marshalling timeout message: {}", e))
    }
}

pub enum Event {
    MessageSend(String),
    MessageReceive(String),
    TimeoutStart(TimeoutInfo),
    TimeoutEnd(TimeoutInfo),
    Other(String, HashMap<String,String>)
}

#[derive(Serialize, Deserialize)]
pub struct EventMessage {
    t: String,
    params: HashMap<String, String>,
    replica: ReplicaID,
    timestamp: SystemTime,
}

impl EventMessage {
    pub fn from_str(s: &str) -> Result<EventMessage, Error> {
        let res: serde_json::Result<EventMessage> = serde_json::from_str(s);
        match res {
            Ok(e) => Ok(e),
            Err(_) =>  Err(String::from("failed to parse event from data"))
        }
    }
    
    pub fn to_str(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| format!("error marshalling timeout message: {}", e))
    }
}

impl Event {
    pub fn to_message(&self, replica_id: ReplicaID) -> EventMessage {
        match self {
            Event::MessageSend(m_id) => {
                EventMessage {
                    t: String::from("MessageSend"),
                    params: HashMap::from([
                        (String::from("message_id"), String::from(m_id)),
                    ]),
                    replica: replica_id,
                    timestamp: SystemTime::now(),
                }
            },
            Event::MessageReceive(m_id) => {
                EventMessage {
                    t: String::from("MessageReceive"),
                    params: HashMap::from([
                        (String::from("message_id"), String::from(m_id)),
                    ]),
                    replica: replica_id,
                    timestamp: SystemTime::now(),
                }
            },
            Event::TimeoutStart(timeout) => {
                EventMessage {
                    t: String::from("TimeoutStart"),
                    params: HashMap::from([
                        (String::from("type"), timeout.t.clone()),
                        (String::from("duration"), timeout.duration()),
                    ]),
                    replica: replica_id,
                    timestamp: SystemTime::now(),
                }
            },
            Event::TimeoutEnd(timeout) => {
                EventMessage {
                    t: String::from("TimeoutEnd"),
                    params: HashMap::from([
                        (String::from("type"), timeout.t.clone()),
                        (String::from("duration"), timeout.duration()),
                    ]),
                    replica: replica_id,
                    timestamp: SystemTime::now(),
                }
            },
            Event::Other(t, params) => {
                EventMessage {
                    t: String::from(t),
                    params: params.clone(),
                    replica: replica_id,
                    timestamp: SystemTime::now(),
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DirectiveMessage {
    pub action: String
}

impl DirectiveMessage {
    pub fn from_str(s: &str) -> Result<DirectiveMessage, Error> {
        let res: serde_json::Result<DirectiveMessage> = serde_json::from_str(s);
        match res {
            Ok(directive) => Ok(directive),
            Err(_) => Err(String::from("failed to parse message from data"))
        }
    }
}