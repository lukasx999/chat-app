use std:: {
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod db;
use db::{DB};

mod model;
use model::{ChatHistory, Message};



fn handle_connection(mut stream: TcpStream) {

    let buf = BufReader::new(&mut stream);
    let request: String = buf.lines().next().unwrap().unwrap();

    let mut response = String::new();

    response = match request.as_str() {
        "GET /chat_history HTTP/1.1" => {
            "HTTP/1.1 200 OK".to_owned()
            // TODO: return json
        }
        _ => {
            "HTTP/1.1 404 NOT FOUND".to_owned()
        }
    };


    // stream.write_all(response.as_bytes()).unwrap();


}





fn main() -> std::io::Result<()> {

    let args: Vec<String> = std::env::args().collect();
    let db = DB::new("chat.db");

    if args.len() == 2 && args[1] == "init" {
        println!("init!");
        db.setup();
        return Ok(());
    }

    // db.add_message("mike", "hello");
    let history: ChatHistory = db.get_history();


    /*
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    */



    Ok(())

}
