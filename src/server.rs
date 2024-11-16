use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
    response::Json,
    extract::State,
    http::StatusCode,
};

mod server_db;
use server_db::DB;

mod model;
use model::{ChatHistory, Message};


const ADDRESS: &str = "127.0.0.1:7878";



type AnyError<T> = Result<T, Box<dyn std::error::Error>>;



fn get_json_response(json: &String) -> String {

    let header        = "HTTP/1.1 200 OK\n";
    let content_type  = "Content-Type: application/json\n";
    let length        = format!("Content-Length: {}\n\n", json.len());
    let body          = format!("{}", json);
    format!("{}{}{}{}", header, content_type, length, body)

}

/*
fn handle_connection(db: &DB, mut stream: TcpStream) -> io::Result<()> {

    let buf = BufReader::new(&mut stream);
    let mut buf_lines = buf.lines();

    let request: String = buf_lines
        .next()
        .unwrap()?; // TODO: this

    let body: String = buf_lines.nth(2).unwrap()?;
    dbg!(&request);

    let response: Option<String> =
    if request == "GET /chat_history HTTP/1.1" {
        let history: ChatHistory = db.get_history();
        let json: String = history.serialize()?;
        Some(get_json_response(&json))
    }
    else if request == "POST /send_message HTTP/1.1" {
        // db.add_message("mike", "hello");
        // dbg!(body);
        None
    }
    else {
        Some("HTTP/1.1 404 NOT FOUND".to_owned())
    };

    if let Some(json) = response {
        stream.write_all(json.as_bytes())?;
    }
    Ok(())

}
*/









async fn chat_history(state: State<Arc<DB>>)
-> Result<Json<ChatHistory>, StatusCode> {

    println!("connection found at /chat_history");

    match state.get_history().await {
        Ok(history) => Ok(Json(history)),
        Err(_)      => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }

}


// TODO: make post request to this route from client
async fn add_message(state: State<Arc<DB>>)
-> Result<(), StatusCode> {

    println!("connection found at /add_message");

    let msg = Message::new(None, "gouber", "whats sup");

    match state.add_message(msg).await {
        Ok(_)  => Ok(()),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }

}





#[tokio::main]
async fn main() -> AnyError<()> {

    let state = Arc::new(DB::new("src/chat.db").await?);

    let app = Router::new()
        .route("/chat_history", get(chat_history))
        .route("/add_message", post(add_message))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    axum::serve(listener, app).await.unwrap();



    // let response: Option<String> =
    // if request == "GET /chat_history HTTP/1.1" {

            // let history: ChatHistory = db.get_history();
            // let json: String = history.serialize()?;
            // Some(get_json_response(&json))

    // }
    // else if request == "POST /send_message HTTP/1.1" {
    //     // db.add_message("mike", "hello");
    //     // dbg!(body);
    //     None
    // }
    // else {
    //     Some("HTTP/1.1 404 NOT FOUND".to_owned())
    // };
    //
    // if let Some(json) = response {
    //     stream.write_all(json.as_bytes())?;
    // }
    // Ok(())



    Ok(())

}
