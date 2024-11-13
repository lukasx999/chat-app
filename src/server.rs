use std:: {
    io,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod server_db;
use server_db::{DB};

mod model;
use model::{ChatHistory, Message};



fn route_get(route: &str) -> String {
    format!("GET {route} HTTP/1.1")
}

fn get_json_response(json: &String) -> String {

    let header        = "HTTP/1.1 200 OK\n";
    let content_type  = "Content-Type: application/json\n";
    let length        = format!("Content-Length: {}\n\n", json.len());
    let body          = format!("{}", json);
    format!("{}{}{}{}", header, content_type, length, body)

}

fn handle_connection(db: &DB, mut stream: TcpStream) -> io::Result<()> {

    let buf = BufReader::new(&mut stream);
    let request: String = buf
        .lines()
        .next()
        .unwrap()?;

    // db.add_message("mike", "hello");

    let response = if request == route_get("/chat_history") {

        let history: ChatHistory = db.get_history();
        let json: String = history.serialize()?;
        get_json_response(&json)

    }
    else {
        "HTTP/1.1 404 NOT FOUND".to_owned()
    };

    stream.write_all(response.as_bytes())?;
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
