/// Define the central app for the GUI.
/// Does nothing except routing the call to the different elements that need to be shown.
// Iced imports
use iced::widget::{column, container, text};
use iced::{Element, Length};

// Local importers
use crate::components;
use crate::pages::programmer::{self, ProgrammerState};

/// Enumerate the different pages to be available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Programmer,
    Debugger,
    Io,
    Routing,
    Memory,
    Plot,
    Files,
    Sequences,
    Help,
    Settings,
}

/// The central memory struct
pub struct EtherBenchApp {
    current_page: Page,

    programmer_state: ProgrammerState,
}

/// Global enumerator for all the submodules
#[derive(Debug, Clone)]
pub enum Message {
    NaviguateTo(Page),
    Programmer(programmer::ProgrammerMessage),
}

/// Implemting the router
impl EtherBenchApp {
    /// Creating a new instance of the app.
    pub fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                current_page: Page::Programmer,
                programmer_state: ProgrammerState::new(),
            },
            iced::Task::none(),
        )
    }

    /// Updating the view.
    /// This is basically a router to the matching area while adding the required elements all around !
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::NaviguateTo(page) => {
                self.current_page = page;
                iced::Task::none()
            }

            Message::Programmer(ws_msg) => self
                .programmer_state
                .update(ws_msg)
                .map(Message::Programmer),
        }
    }

    /// Building the view
    pub fn view(&self) -> Element<'_, Message> {
        // let navbar = ...

        // let menubar = ...

        // let bottombar = ...

        // Let's build the current page
        let content: Element<Message> = match self.current_page {
            Page::Programmer => self.programmer_state.view().map(Message::Programmer),
            Page::Debugger => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Io => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Memory => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Routing => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Plot => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Files => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Sequences => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Help => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Settings => container(text("In building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        };

        // Final assembly
        column![content].into()
    }
}
