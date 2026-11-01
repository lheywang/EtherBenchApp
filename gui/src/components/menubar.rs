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

/// Top bar for the App with a menu on the left, some space and finally the statuses.
pub fn view<'a>(
    status: &'a str,
    color: Option<Color>,
    connected_ip: Option<&'a str>,
    connected_serial: Option<&'a str>,
) -> Element<'a, Message> {
    // Logo
    let logo = texts::icon_text(&Icon::Logo, iced::Length::Fixed(16.0));

    // Let's define the menubar items
    let file = Menu::new(menu_items![
        (button("New").on_press(MenubarMessage::FileCloseScript)),
        (button("Close").on_press(MenubarMessage::FileCloseScript)),
        (button("Load").on_press(MenubarMessage::FileLoadSlot)),
        (button("Save").on_press(MenubarMessage::FileSaveSlot)),
        (button("Transfer to").on_press(MenubarMessage::FileTransferSlot)),
        (button("Export serial session").on_press(MenubarMessage::FileExportSession))
    ])
    .width(Length::Fill);

    let edit = Menu::new(menu_items![
        (button("Cut").on_press(MenubarMessage::EditCut)),
        (button("Copy").on_press(MenubarMessage::EditCopy)),
        (button("Paste").on_press(MenubarMessage::EditPaste)),
        (button("Find").on_press(MenubarMessage::EditFind)),
        (button("Replace").on_press(MenubarMessage::EditReplace)),
    ])
    .width(Length::Fill);

    let view = Menu::new(menu_items![
        (button("Screenshot").on_press(MenubarMessage::ViewScreenshot)),
        (button("Export to image").on_press(MenubarMessage::ViewExportTo))
    ])
    .width(Length::Fill);

    // Define the main menubar item
    let bar = MenuBar::new(menu_items![
        (text("File").size(13), file),
        (text("Edit").size(13), edit),
        (text("View").size(13), view),
    ]);

    // Define the status size
    let status_size = iced::Length::Fixed(16.0);

    // Add the status elements
    let serial_status = match connected_serial {
        Some(serial) => icon_text_text(&Icon::CircleFull, &format!("Using {serial}"), status_size),
        None => icon_text_text(
            &Icon::CircleFull,
            format!("Serial unconnected"),
            status_size,
        ),
    };

    let ip_status = match connected_ip {
        Some(ip) => icon_text_text(
            &Icon::CircleFull,
            &format!("Connected to {ip}"),
            status_size,
        ),
        None => icon_text_text(
            &Icon::CircleFull,
            format!("Network unconnected"),
            status_size,
        ),
    };

    let device_status = icon_text_text(&Icon::CircleFull, format!("Status : {status}"), status_size);

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
