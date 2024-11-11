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

pub type ChatHistory = Vec<Message>;

