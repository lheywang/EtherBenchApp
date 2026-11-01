/// Generate the top menubar used within the App
// Iced
use iced::widget::{Space, button, container, row, text};
use iced::{Color, Element, Length, alignment};
use iced_aw::{Menu, MenuBar, menu_items};

// Local
use crate::app::Message;
use crate::assets::Icon;
use crate::components::elements::texts::{self, icon_text_text};
use crate::style;

/// Define the local MenuBar messages. Usefull to handle them locally !
#[derive(Debug, Clone)]
pub enum MenubarMessage {
    FileOpenScript,
    FileCloseScript,
    FileLoadSlot,
    FileSaveSlot,
    FileTransferSlot,
    FileExportSession,
    EditCopy,
    EditCut,
    EditPaste,
    EditFind,
    EditReplace,
    ViewScreenshot,
    ViewExportTo,
}

fn menubar_button<'a>(
    name: impl Into<String>,
    message: MenubarMessage,
) -> button::Button<'a, MenubarMessage> {
    button(text(name.into()).width(Length::Fill))
        .on_press(message)
        .width(Length::Fill)
        .style(button::text)
}

/// Top bar for the App with a menu on the left, some space and finally the statuses.
pub fn view<'a>(
    status: &'a str,
    color: Option<Color>,
    connected_ip: Option<&'a str>,
    connected_serial: Option<&'a str>,
) -> Element<'a, Message> {
    // Logo
    let logo = texts::icon_text(&Icon::Logo, iced::Length::Fixed(16.0), Color::WHITE);

    const MENU_WIDTH: f32 = 220.0;

    // Let's define the menubar items
    let file = Menu::new(menu_items![
        (menubar_button("New", MenubarMessage::FileOpenScript)),
        (menubar_button("Close", MenubarMessage::FileCloseScript)),
        (menubar_button("Load", MenubarMessage::FileLoadSlot)),
        (menubar_button("Save", MenubarMessage::FileSaveSlot)),
        (menubar_button("Transfer to", MenubarMessage::FileTransferSlot)),
        (menubar_button("Export serial session", MenubarMessage::FileExportSession)),
    ])
    .width(MENU_WIDTH);

    let edit = Menu::new(menu_items![
        (menubar_button("Cut", MenubarMessage::EditCut)),
        (menubar_button("Copy", MenubarMessage::EditCopy)),
        (menubar_button("Paste", MenubarMessage::EditPaste)),
        (menubar_button("Find", MenubarMessage::EditFind)),
        (menubar_button("Replace", MenubarMessage::EditReplace)),
    ])
    .width(MENU_WIDTH);

    let view = Menu::new(menu_items![
        (menubar_button("Screenshot", MenubarMessage::ViewScreenshot)),
        (menubar_button("Export to image", MenubarMessage::ViewExportTo)),
    ])
    .width(MENU_WIDTH);

    // Define the main menubar item
    let bar = MenuBar::new(menu_items![
        (text("File").size(13), file),
        (text("Edit").size(13), edit),
        (text("View").size(13), view),
    ])
    .spacing(12)
    .style(style::menubar_style);

    // Define the status size
    let status_size = iced::Length::Fixed(16.0);

    // Add the status elements
    let serial_status = match connected_serial {
        Some(serial) => icon_text_text(
            &Icon::CircleFull,
            &format!("Using {serial}"),
            status_size,
            style::COLOR_ACTIVE,
        ),
        None => icon_text_text(
            &Icon::CircleFull,
            format!("Serial unconnected"),
            status_size,
            style::COLOR_INACTIVE,
        ),
    };

    let ip_status = match connected_ip {
        Some(ip) => icon_text_text(
            &Icon::CircleFull,
            &format!("Connected to {ip}"),
            status_size,
            style::COLOR_ACTIVE,
        ),
        None => icon_text_text(
            &Icon::CircleFull,
            format!("Network unconnected"),
            status_size,
            style::COLOR_INACTIVE,
        ),
    };

    let device_status = match color {
        Some(color) => icon_text_text(
            &Icon::CircleFull,
            format!("Status : {status}"),
            status_size,
            color,
        ),
        None => icon_text_text(
            &Icon::CircleFull,
            format!("Status is not available"),
            status_size,
            style::COLOR_INACTIVE,
        ),
    };

    // Combine the contents
    let content = row![
        logo,
        bar,
        Space::new().width(Length::Fill),
        device_status,
        ip_status,
        serial_status
    ]
    .spacing(12)
    .padding([2, 12])
    .align_y(alignment::Vertical::Center);

    // Build the final container
    let element: Element<'a, MenubarMessage> = container(content)
        .width(Length::Fill)
        .style(style::navbar_style)
        .into();

    element.map(Message::MenuBarAction)
}

pub fn update(message: MenubarMessage) -> iced::Task<MenubarMessage> {
    match message {
        MenubarMessage::FileOpenScript => {
            println!("Openning file");
            iced::Task::none()
        }

        MenubarMessage::FileCloseScript => {
            println!("Closing file");
            iced::Task::none()
        }

        _ => {
            println!("Unhandled");
            iced::Task::none()
        }
    }
}
