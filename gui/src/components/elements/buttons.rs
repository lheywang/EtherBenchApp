use iced::widget::svg;
/// Define the standard buttons for the GUI.
/// This ensure a uniformity across all elements.
// Iced imports
use iced::widget::{Space, button, row, text};
use iced::{Color, Element, alignment};

use crate::assets::Icon;
use crate::style;

/// Factory for primary buttons
pub fn text_btn<'a, Message>(label: &'a str, on_press: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut btn = button(text(label).size(14))
        .padding([8, 16])
        .style(style::primary_button_style);

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    btn.into()
}

pub fn icon_text_btn<'a, Message>(
    icon: &'a Icon,
    label: &'a str,
    on_press: Option<Message>,
    icon_size: iced::Length,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = row![
        icon.widget().width(icon_size).height(icon_size),
        Space::new().width(20),
        text(label).size(14)
    ]
    .align_y(alignment::Vertical::Center);

    let mut btn = button(content)
        .padding([8, 16])
        .style(style::primary_button_style);

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    btn.into()
}

pub fn icon_btn<'a, Message>(
    icon: &'a Icon,
    on_press: Option<Message>,
    icon_size: iced::Length,
) -> Element<'a, Message>
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

    let mut btn = button(content)
        .padding([8, 16])
        .style(style::primary_icon_button_style);

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    btn.into()
}
