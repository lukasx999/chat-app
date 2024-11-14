use std:: {
    io,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod server_db;
use server_db::{DB};

mod model;
use model::{ChatHistory, Message};





fn get_json_response(json: &String) -> String {

    let header        = "HTTP/1.1 200 OK\n";
    let content_type  = "Content-Type: application/json\n";
    let length        = format!("Content-Length: {}\n\n", json.len());
    let body          = format!("{}", json);
    format!("{}{}{}{}", header, content_type, length, body)

}

fn handle_connection(db: &DB, mut stream: TcpStream) -> io::Result<()> {

    let buf = BufReader::new(&mut stream);
    let mut buf_lines = buf.lines();

    let request: String = buf_lines
        .next()
        .unwrap()?; // TODO: this

    let body: String = buf_lines.nth(2).unwrap()?;


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






const ADDRESS: &str = "127.0.0.1:7878";

fn main() -> io::Result<()> {

    let args: Vec<String> = std::env::args().collect();
    let db = DB::new("chat.db");

    if args.len() == 2 && args[1] == "setup" {
        println!("setting up db...");
        db.setup();
        return Ok(());
    }

    let listener = TcpListener::bind(ADDRESS)?;
    println!("listening...");

    // TODO: multithreading
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(&db, stream)?;
        println!("connection found!");
    }

    Ok(())

}
