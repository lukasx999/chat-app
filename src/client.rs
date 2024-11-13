use eframe::egui;
use std::io;
use std::error::Error;

mod model;
use model::{ChatHistory, Message};




// Can hold any type of error
type E<T> = Result<T, Box<dyn Error>>;

// Stores the state of the client
struct ChatClient {
    username:        String,
    current_message: String,
    chat_history:    ChatHistory,
}

impl ChatClient {
    fn new(_cc: &eframe::CreationContext<'_>, name: &str) -> E<Self> {

        let mut s = Self {
            username:        name.to_owned(),
            current_message: "".to_owned(),
            chat_history:    ChatHistory::new(),
        };

        s.fetch_history()?;

        Ok(s)

    }

    fn fetch_history(&mut self) -> E<()> {
        self.chat_history = get_chat_history()?;
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


            // let textedit: egui::Response = ui.text_edit_singleline(&mut self.current_message);
            // if ui.button("send").clicked() {
            //     self.server_connection.send_message(
            //         chat::ChatMessage::new(self.username.as_str(), self.current_message.as_str())
            //     );
            // }


            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

        });

    }
}


const WINDOW_WIDTH:   f32  = 800.0;
const WINDOW_HEIGHT:  f32  = 600.0;
const SERVER_ADDRESS: &str = "http://127.0.0.1:7878";


fn get_chat_history() -> E<ChatHistory> {
    let response: reqwest::blocking::Response = reqwest::blocking
        ::get(format!("{SERVER_ADDRESS}/chat_history"))?;

    let json: String = response.text()?;
    Ok(ChatHistory::deserialize(&json)?)

}



fn main() -> E<()> {

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
            Ok(Box::new(ChatClient::new(cc, "mike").unwrap()))
        })
    )?)

}
