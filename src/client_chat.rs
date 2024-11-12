#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender:  String,
    pub message: String,
    // extra information (timestampt, ...)
}

impl ChatMessage {
    pub fn new(sender: &str, message: &str) -> Self {
        Self {
            sender:  sender.to_owned(),
            message: message.to_owned()
        }
    }
}

pub type ChatHistory = Vec<ChatMessage>;

// Dummy representation for server
#[derive(Debug, Clone)]
pub struct ChatApplicaton {
    messages: ChatHistory,
}

impl ChatApplicaton {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn send_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &ChatHistory {
        &self.messages
    }

}
