/// Define the standard buttons for the GUI.
/// This ensure a uniformity across all elements.
// Iced imports
use iced::widget::{Space, button, row, text};
use iced::{Element, alignment};

use crate::assets::Icon;

/// Factory for primary buttons
pub fn primary<'a, Message>(label: &'a str, on_press: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut btn = button(text(label).size(14))
        .padding([8, 16])
        .style(button::primary);

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    btn.into()
}

pub fn secondary<'a, Message>(label: &'a str, on_press: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut btn = button(text(label).size(14))
        .padding([8, 16])
        .style(button::secondary);

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    btn.into()
}

pub fn danger<'a, Message>(label: &'a str, on_press: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut btn = button(text(label).size(14))
        .padding([8, 16])
        .style(button::danger);

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    btn.into()
}

pub fn icon_btn<'a, Message>(
    icon: &'a Icon,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = row![
        icon.widget().width(16),
        Space::new().width(20),
        text(label).size(14)
    ]
    .align_y(alignment::Vertical::Center);

    let mut btn = button(content).padding([8, 16]).style(button::primary);

    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }

    btn.into()
}
