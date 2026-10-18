use iced::widget::{button, column, container, text};
use iced::{alignment, Element, Length};

// On importe notre librairie locale
use memory::get_dummy_buffer_size;

// Mod
#[macro_use]
mod assets;

pub fn main() -> iced::Result {
    let app = iced::application(DemoApp::default, DemoApp::update, DemoApp::view)
        // CORRECTION : On type explicitement '_state: &DemoApp' 
        // pour que le compilateur déduise correctement la lifetime
        .title(|_state: &DemoApp| String::from("EtherBench - Démo Architecturale"));

    load_custom_fonts!(app).run()
}

#[derive(Default)]
struct DemoApp {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
}

impl DemoApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mem_size = get_dummy_buffer_size();

        let content = column![
            text(format!("Taille du buffer central : {} octets", mem_size)).size(20),
            text(format!("Compteur UI : {}", self.value)).size(30),
            button("Tester la réactivité").on_press(Message::IncrementPressed)
        ]
        .padding(20)
        .spacing(20)
        .align_x(alignment::Horizontal::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}