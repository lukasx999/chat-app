use eframe::egui;

mod client_chat;
use client_chat as chat;

mod model;
use model::{ChatHistory, Message};


struct ChatClient {
    username: String,
    current_message: String,
    server_connection: chat::ChatApplicaton,
}

impl ChatClient {
    fn new(cc: &eframe::CreationContext<'_>, name: &str) -> Self {
        Self {
            username: name.to_owned(),
            current_message: "".to_owned(),
            server_connection: chat::ChatApplicaton::new()
        }
    }
}


impl eframe::App for ChatClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ctx.set_pixels_per_point(3.0);

            ui.heading("Chat Application");

            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });



            let mut s = String::new();

            // TODO: better layout
            for message in self.server_connection.get_messages() {
                s.push_str(format!("[{}]: {}\n", message.sender, message.message).as_str());
            }

            ui.label(s);

            ctx.input(|input| {
                if input.key_pressed(egui::Key::Enter) {
                    println!("you pressed enter");
                }
            });

            let textedit: egui::Response = ui.text_edit_singleline(&mut self.current_message);
            if ui.button("send").clicked() {
                self.server_connection.send_message(
                    chat::ChatMessage::new(self.username.as_str(), self.current_message.as_str())
                );
            }





            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

        });

    }
}


const WINDOW_WIDTH:  f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;


fn get_chat_history() -> ChatHistory {
    let response: reqwest::blocking::Response = reqwest::blocking
        ::get("http://127.0.0.1:7878/chat_history")
        .unwrap();

    response.json::<ChatHistory>().unwrap()
    // let json: String = response.text().unwrap();
    // ChatHistory::deserialize(&json)

}


fn main() -> eframe::Result {

    let chat: ChatHistory = get_chat_history();
    dbg!(chat);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Chat App")
            .with_active(true)
            .with_resizable(true)
            .with_position(egui::Pos2::new(1500.0, 500.0))
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // Ok(Box::<ChatClient>::default())
            Ok(Box::new(ChatClient::new(cc, "mike")))
        }),
    )
}
