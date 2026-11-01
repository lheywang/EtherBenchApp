/// Define the standard buttons for the GUI.
/// This ensure a uniformity across all elements.
// Iced imports
use iced::widget::{Space, row, text, container};
use iced::{Color, Element, alignment};
use iced::widget::svg;

use crate::assets::Icon;

/// Factory for primary buttons
pub fn text_text<'a, Message>(label: impl Into<String>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let msg = text(label.into()).size(14)
        .style(text::primary);

    msg.into()
}

pub fn icon_text_text<'a, Message>(
    icon: &'a Icon,
    label: impl Into<String>,
    icon_size: iced::Length,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = row![
        icon.widget().width(icon_size).height(icon_size),
        Space::new().width(20),
        text(label.into()).size(14)
    ]
    .align_y(alignment::Vertical::Center);

    let msg = container(content)
        .padding([8, 16]);

    msg.into()
}

pub fn icon_text<'a, Message>(icon: &'a Icon, icon_size: iced::Length) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = icon
        .widget()
        .width(icon_size)
        .height(icon_size)
        .style(|_theme, _status| svg::Style {
            color: Some(Color::WHITE),
        });

    let msg = container(content)
        .padding([8, 16]);

    msg.into()
}
