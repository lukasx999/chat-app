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

    // pub fn get_history(&self) -> ChatHistory {
    //
    //     let mut stmt = self.conn.prepare("SELECT * FROM chat").unwrap();
    //
    //     ChatHistory::from(
    //         stmt.query_map([], |row| {
    //             let (id, sender, message): (u32, String, String) = (
    //                 row.get(0).unwrap(),
    //                 row.get(1).unwrap(),
    //                 row.get(2).unwrap()
    //             );
    //             Ok(Message::new(Some(id), sender.as_str(), message.as_str()))
    //         })
    //             .unwrap()
    //             .map(|item| item.unwrap())
    //             .collect::<Vec<Message>>())
    //
    // }


    pub async fn get_history(&mut self) -> Result<ChatHistory, sqlx::Error> {

        let msgs: Vec<(i32, String, String)> =
        sqlx::query_as("SELECT id, sender, message FROM chat")
            .fetch_all(&self.conn).await?;

        dbg!(msgs);


        // let messages: Vec<dyn sqlx::Row> =
        // self.conn.fetch_all("SELECT * FROM chat").await.unwrap();

        Ok(ChatHistory::new())

    }

}
