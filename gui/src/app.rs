/// Define the central app for the GUI.
/// Does nothing except routing the call to the different elements that need to be shown.
// Iced imports
use iced::widget::{row, column,  container, text};
use iced::{Element, Length, Color};

use crate::components::menubar::MenubarMessage;
// Local importers
use crate::components::{navbar, menubar};
use crate::pages::programmer::{ProgrammerState, ProgrammerMessage};
use crate::style;

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
    MenuBarAction(MenubarMessage),
    Programmer(ProgrammerMessage),
    Close,
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

            // Page change required
            Message::NaviguateTo(page) => {
                self.current_page = page;
                iced::Task::none()
            }

            // Menubar action
            Message::MenuBarAction(action) => {
                menubar::update(action).map(Message::MenuBarAction)
            }

            // Views updates
            Message::Programmer(ws_msg) => self
                .programmer_state
                .update(ws_msg)
                .map(Message::Programmer),

            // Close the app
            Message::Close => {
                iced::Task::none()
            }
        }
    }

    /// Building the view
    pub fn view(&self) -> Element<'_, Message> {
        let navbar = navbar::view(&self.current_page);

        // let menubar = ...
        let menubar = menubar::view("Connected", Some(Color::WHITE), Some("192.168.1.50"), Some("COM12"));

        // let bottombar = ...

        // Let's build the current page
        let content: Element<Message> = match self.current_page {
            Page::Programmer => self.programmer_state.view().map(Message::Programmer),
            Page::Debugger => container(text("Debugger in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Io => container(text("IO in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Memory => container(text("Memory in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Routing => container(text("Routing in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Plot => container(text("Plot in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Files => container(text("Files in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Sequences => container(text("Sequences in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Help => container(text("Help in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            Page::Settings => container(text("Settings in building ..."))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        };

        // Assemble the central area
        let page = row![navbar, content];

        // Global assembly
        let screen = column![
            menubar,
            page
        ];

        // Apply some styles to everything
        container(screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::navbar_style)
            .into()
    }
}
