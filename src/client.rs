use eframe::egui::{self, TextEdit};

mod model;
use model::{ChatHistory, Message};

use reqwest::blocking::Client as ReqwestClient;

use tokio_tungstenite::{WebSocketStream};
// tungstenite::protocol::Message

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use futures::StreamExt;


const WINDOW_WIDTH:   f32  = 1200.0;
const WINDOW_HEIGHT:  f32  = 1000.0;
const SERVER_ADDRESS: &str = "http://127.0.0.1:7878";




// Can hold any type of error
type AnyError<T> = Result<T, Box<dyn std::error::Error>>;

// Stores the state of the client
struct ChatClient {
    username: String,

    current_message: String,
    chat_history:    ChatHistory,

    request_client:  ReqwestClient,
}

impl ChatClient {
    fn new(_cc: &eframe::CreationContext<'_>, username: &str) -> AnyError<Self> {

        let mut s = Self {
            username:        username.to_owned(),
            current_message: "".to_owned(),
            chat_history:    ChatHistory::new(),
            request_client:  ReqwestClient::new(),
        };

        s.fetch_history()?;

        Ok(s)

    }


    async fn connect_websocket(&self) -> Result<(), tokio_tungstenite::tungstenite::Error> {

        let (ws_stream, _): (WebSocketStream<_>, _) =
        tokio_tungstenite::connect_async(format!("{SERVER_ADDRESS}/ws")).await?;

        let (write, read) = ws_stream.split();

        Ok(())

    }

    fn fetch_history(&mut self) -> AnyError<()> {
        let response = self.request_client
            .get(format!("{SERVER_ADDRESS}/chat_history"))
            .send()?;

        let json: String = response.text()?;
        self.chat_history = ChatHistory::deserialize(&json)?;
        Ok(())
    }

    fn send_message(&self, msg: Message) -> AnyError<()> {

        let ser = msg.serialize()?;

        self.request_client.post(format!("{SERVER_ADDRESS}/add_message"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, ser.len())
            .body(ser)
            .send()?;

        Ok(())
    }

}




impl eframe::App for ChatClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ctx.set_pixels_per_point(3.0);
            ui.heading("Chat Application");

            let mut s = String::new();

            for message in self.chat_history.messages() {
                s.push_str(format!("[{}]: {}\n", message.sender, message.message).as_str());
            }

            ui.label(s);

            // ctx.input(|input| {
            //     if input.key_pressed(egui::Key::Enter) {
            //         println!("you pressed enter");
            //     }
            // });





            if ui.button("update History").clicked() {
                self.fetch_history().unwrap(); // TODO: handle loss of connection
            }

            let edit_currentmsg: egui::Response =
            ui.text_edit_singleline(&mut self.current_message);


            if ui.button("send").clicked() {
                let msg = Message::new(None,
                                       self.username.as_str(),
                                       self.current_message.as_str());
                self.send_message(msg).unwrap(); // TODO: handle error
                self.current_message.clear();
            }


            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

        });

    }
}







fn main() -> AnyError<()> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Chat App")
            .with_active(true)
            .with_resizable(true)
            .with_position(egui::Pos2::new(1500.0, 500.0))
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    Ok(eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            Ok(Box::new(ChatClient::new(cc, "foobar johnson").unwrap()))
        })
    )?)

}
