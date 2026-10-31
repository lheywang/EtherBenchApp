/// Define the programmer base page.
/// Handle basic programming operations, especially to flash memories and so on !
// Iced
use iced::widget::{container, row, text};
use iced::{Element, Length};

// Internal structure
pub struct ProgrammerState {
    pub is_executing: bool,
}

#[derive(Debug, Clone)]
pub enum ProgrammerMessage {
    RunTest,
}

impl ProgrammerState {
    pub fn new() -> Self {
        Self {
            is_executing: false,
        }
    }

    pub fn update(&mut self, message: ProgrammerMessage) -> iced::Task<ProgrammerMessage> {
        match message {
            ProgrammerMessage::RunTest => {
                println!("Executing the test !");
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, ProgrammerMessage> {
        let main_editor = container(text("Programmer in building"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        row![main_editor]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
