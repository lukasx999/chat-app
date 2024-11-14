use rusqlite::{Connection, Result};

use crate::model::{ChatHistory, Message};



#[derive(Debug)]
pub struct DB {
    conn: Connection,
    db_path: String,
}

impl DB {
    pub fn new(db_path: &str) -> Self {
        Self {
            conn: Connection::open(db_path).unwrap(),
            db_path: db_path.to_owned(),
        }
    }

    pub fn setup(&self) {
        self.conn.execute(
            "CREATE TABLE chat (
            id      INTEGER PRIMARY KEY,
            sender  TEXT NOT NULL,
            message TEXT NOT NULL
            )
            ", (),
        ).unwrap();
        // TODO: timestamp, ...

    }

    // TODO: add proper error handling with results
    pub fn add_message(&self, msg: Message) {

        self.conn.execute(
            "INSERT INTO chat (sender, message) VALUES (?1, ?2)",
            (msg.sender, msg.message)
        ).unwrap();

    }

    pub fn get_history(&self) -> ChatHistory {

        let mut stmt = self.conn.prepare("SELECT * FROM chat").unwrap();

        ChatHistory::from(
            stmt.query_map([], |row| {
                let (id, sender, message): (u32, String, String) = (
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap()
                );
                Ok(Message::new(Some(id), sender.as_str(), message.as_str()))
            })
                .unwrap()
                .map(|item| item.unwrap())
                .collect::<Vec<Message>>())

    }

}
