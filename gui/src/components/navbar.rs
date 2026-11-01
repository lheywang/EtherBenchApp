/// Define the lateral navbar, in the VSCode style.
/// Used to change between the different operating modes.

use iced::widget::{column, container, Space};
use iced::{ Element, Length, alignment};

use crate::app::{Message, Page};
use crate::assets::{Icon};
use crate::style;
use crate::components::elements::buttons::{icon_btn};

pub fn view(_active_page: &Page) -> Element<'_, Message> {

    // Naviguation column.
    // Elements are on top, and settings / help on the bottom.
    let icon_size = iced::Length::Fixed(24.0);
    let nav = column![
        icon_btn(&Icon::PageProgrammer, Some(Message::NaviguateTo(Page::Programmer)), icon_size),
        icon_btn(&Icon::PageDebugger, Some(Message::NaviguateTo(Page::Debugger)), icon_size),
        icon_btn(&Icon::PageIo, Some(Message::NaviguateTo(Page::Io)), icon_size),
        icon_btn(&Icon::PageRouting, Some(Message::NaviguateTo(Page::Routing)), icon_size),
        icon_btn(&Icon::PageMemory, Some(Message::NaviguateTo(Page::Memory)), icon_size),
        icon_btn(&Icon::PagePlot, Some(Message::NaviguateTo(Page::Plot)), icon_size),
        icon_btn(&Icon::PageFiles, Some(Message::NaviguateTo(Page::Files)), icon_size),
        icon_btn(&Icon::PageSequences, Some(Message::NaviguateTo(Page::Sequences)), icon_size),
        Space::new().height(Length::Fill),
        icon_btn(&Icon::PageHelp, Some(Message::NaviguateTo(Page::Help)), icon_size),
        icon_btn(&Icon::PageSettings, Some(Message::NaviguateTo(Page::Settings)), icon_size),
    ]
    .spacing(12)
    .align_x(alignment::Horizontal::Center);

    container(nav)
        .width(Length::Shrink)
        .height(Length::Fill)
        .style(style::navbar_style) 
        .into()

}

