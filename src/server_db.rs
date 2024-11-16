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

    pub async fn add_message(&self, msg: Message) -> Result<(), sqlx::Error> {

        sqlx::query!("INSERT INTO chat (sender, message) VALUES (?1, ?2)",
            msg.sender, msg.message)
            .execute(&self.conn).await?;

        Ok(())

    }

    pub async fn get_history(&self) -> sqlx::Result<ChatHistory> {

        let messages: Vec<Message> =
        sqlx::query_as!(Message, "SELECT id, sender, message FROM chat")
            .fetch_all(&self.conn).await?;

        Ok(ChatHistory::from(messages))

    }

}
