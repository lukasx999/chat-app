use std:: {
    io,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

mod server_db;
use server_db::{DB};

mod model;
use model::{ChatHistory, Message};


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





use axum::{
    routing::get,
    Router,
    response::Json,
    extract::State,
};


const ADDRESS: &str = "127.0.0.1:7878";



async fn chat_history(state: State<Arc<DB>>) -> Json<Message> {
    // state.get_history().await.unwrap();
    Json(Message::new(None, "foo", "bar"))
}





#[tokio::main]
async fn main() -> AnyError<()> {

    // let args: Vec<String> = std::env::args().collect();
    // let db = DB::new("chat.db");
    //
    // if args.len() == 2 && args[1] == "setup" {
    //     println!("setting up db...");
    //     db.setup();
    //     return Ok(());
    // }



    let mut db = DB::new("chat.db").await?;
    let history = db.get_history().await?;
    dbg!(history);

    let state = Arc::new(db);


    let app = Router::new()
        .route("/chat_history", get(chat_history))
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






    /*
    let listener = TcpListener::bind(ADDRESS)?;
    println!("listening...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(&db, stream)?;
        println!("connection found!");
    }
    */

    Ok(())



}
