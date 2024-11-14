use serde::{Deserialize, Serialize};
use std::io;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id:      Option<u32>,
    pub sender:  String,
    pub message: String,
    // extra information (timestampt, ...)
}

impl Message {
    pub fn new(id: Option<u32>, sender: &str, message: &str) -> Self {
        Self {
            id,
            sender:  sender.to_owned(),
            message: message.to_owned()
        }
    }

    pub fn serialize(&self) -> io::Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn deserialize(json: String) -> io::Result<Self> {
        Ok(serde_json::from_str(&json)?)
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistory(Vec<Message>);

impl ChatHistory {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn messages(&self) -> Vec<Message> {
        self.0.clone()
    }
}

impl From<Vec<Message>> for ChatHistory {
    fn from(v: Vec<Message>) -> Self {
        Self(v.clone())
    }
}

impl ChatHistory {

    // pub fn new() -> Self {
    //     Self(Vec::new())
    // }

    // pub fn add(&mut self, message: &Message) {
    //     self.0.push(message.clone());
    // }

    pub fn serialize(&self) -> io::Result<String> {
        Ok(serde_json::to_string(&self.0)?)
    }

    pub fn deserialize(json: &String) -> io::Result<Self> {
        Ok(serde_json::from_str(json.as_str())?)
    }

}
