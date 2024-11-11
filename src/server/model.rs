use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id:      u32,
    pub sender:  String,
    pub message: String,
    // extra information (timestampt, ...)
}

impl Message {
    pub fn new(id: u32, sender: &str, message: &str) -> Self {
        Self {
            id,
            sender:  sender.to_owned(),
            message: message.to_owned()
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &String) -> Self {
        serde_json::from_str(json).unwrap()
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistory(Vec<Message>);

impl From<Vec<Message>> for ChatHistory {
    fn from(v: Vec<Message>) -> Self {
        Self(v.clone())
    }
}

impl ChatHistory {

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, message: &Message) {
        self.0.push(message.clone());
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    pub fn deserialize(json: &String) -> Self {
        serde_json::from_str(json).unwrap()
    }

}
