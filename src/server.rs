use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
    response::{Json, Response},
    extract::{
        State,
        ws::{
            self,
            WebSocket,
            WebSocketUpgrade,
        },
    },
    http::StatusCode,
};

mod server_db;
use server_db::DB;

mod model;
use model::{ChatHistory, Message};


const ADDRESS: &str = "127.0.0.1:7878";



type AnyError<T> = Result<T, Box<dyn std::error::Error>>;




async fn chat_history(state: State<Arc<DB>>)
-> Result<Json<ChatHistory>, StatusCode> {

    let db: Arc<DB> = state.0;
    println!("connection found at /chat_history");

    match db.get_history().await {
        Ok(history) => Ok(Json(history)),
        Err(_)      => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }

}


async fn add_message(state: State<Arc<DB>>, payload: Json<Message>)
-> Result<(), StatusCode> {

    let db: Arc<DB> = state.0;
    println!("connection found at /add_message");

    let message: Message = payload.0;

    match db.add_message(message).await {
        Ok(_)  => Ok(()),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }

}


async fn ws_upgrade_handler(ws: WebSocketUpgrade, state: State<Arc<DB>>)
-> Response {

    println!("connection found at /ws");

    ws.on_upgrade(|socket| ws_handler(socket, state))

}


// TODO: send new message via websocket when new message gets added
async fn ws_handler(mut socket: WebSocket, state: State<Arc<DB>>) {
    let t = ws::Message::Text("greetings".to_owned());
    socket.send(t).await.unwrap();
}





#[tokio::main]
async fn main() -> AnyError<()> {

    let state = Arc::new(DB::new("src/chat.db").await?);

    let app = Router::new()
        .route("/chat_history", get (chat_history))
        .route("/add_message",  post(add_message))
        .route("/ws",           get (ws_upgrade_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())

}
