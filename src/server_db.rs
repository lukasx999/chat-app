use crate::model::{ChatHistory, Message};
use sqlx::{SqliteConnection, SqlitePool, SqliteExecutor, Connection, Executor};



#[derive(Debug)]
pub struct DB {
    conn: SqlitePool,
    db_path: String,
}

impl DB {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        Ok(Self {
            conn: SqlitePool::connect(db_path).await?,
            db_path: db_path.to_owned(),
        })
    }

    pub async fn setup(&mut self) -> Result<(), sqlx::Error> {
        self.conn.execute("CREATE TABLE chat (
                           id      INTEGER PRIMARY KEY,
                           sender  TEXT NOT NULL,
                           message TEXT NOT NULL)"
        ).await?;
        Ok(())
        // TODO: timestamp, ...

    }

    /*
    // TODO: add proper error handling with results
    pub fn add_message(&self, msg: Message) {

        self.conn.execute(
            "INSERT INTO chat (sender, message) VALUES (?1, ?2)",
            (msg.sender, msg.message)
        ).unwrap();

    }
    */


    // pub async fn get_history(&mut self) -> sqlx::Result<ChatHistory> {
    //
    //     let query = "SELECT id, sender, message FROM chat";
    //
    //     let msgs: Vec<(i32, String, String)> =
    //     sqlx::query_as(query)
    //         .fetch_all(&self.conn).await?;
    //
    //     let h: Vec<Message> = msgs.iter().map(|message| -> Message {
    //         Message::new(Some(message.0 as u32), message.1.as_str(), message.2.as_str())
    //     }).collect();
    //
    //     Ok(ChatHistory::from(h))
    //
    // }


    pub async fn get_history(&mut self) -> sqlx::Result<ChatHistory> {

        let msgs: Vec<Message> =
        sqlx::query_as!(Message, "SELECT id, sender, message FROM chat")
            .fetch_all(&self.conn).await?;

        Ok(ChatHistory::from(msgs))

    }


}
