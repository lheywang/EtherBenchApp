/// Define the central styling control.
/// This define all the colors on the UI.
use iced::widget::{button, container};
use iced::{Border, Color, Theme};

/// Standard palette for all the elements
pub const BACKGROUND_MAIN: Color = Color::from_rgb(0.12, 0.12, 0.12);
pub const BACKGROUND_PANEL: Color = Color::from_rgb(0.16, 0.16, 0.16);
pub const BACKGROUND_BORDER: Color = Color::from_rgb(0.2, 0.2, 0.2);
pub const ACCENT: Color = Color::from_rgb(0.0, 0.47, 0.84);
pub const TEXT_MUTED: Color = Color::from_rgb(0.6, 0.6, 0.6);
pub const TEXT_ALERT: Color = Color::from_rgb(0.9, 0.3, 0.3);

pub const BUTTON_HOVER: Color = Color::from_rgb(0.12, 0.55, 0.95);
pub const BUTTON_PRESS: Color = Color::from_rgb(0.0, 0.38, 0.70);

/// Style for the navbar
pub fn navbar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BACKGROUND_PANEL.into()),
        border: Border {
            color: BACKGROUND_BORDER.into(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn statusbar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BACKGROUND_PANEL.into()),
        border: Border {
            color: BACKGROUND_BORDER.into(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn menubar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BACKGROUND_PANEL.into()),
        border: Border {
            color: BACKGROUND_BORDER.into(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

/// Resolve primary button style, depending on it's current status.
pub fn primary_button_style(
    _theme: &Theme, status: button::Status
) -> button::Style {

    match status {

        button::Status::Active => button::Style {
            background: Some(ACCENT.into()),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(), 
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(BUTTON_HOVER.into()),
            text_color: Color::WHITE,
            border: Border {
                color: Color::WHITE.scale_alpha(0.25),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(BUTTON_PRESS.into()),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(), 
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(Color::WHITE.scale_alpha(0.4).into()),
            text_color: TEXT_MUTED,
            border: Border {
                radius: 6.0.into(), 
                ..Default::default()
            },
            ..Default::default()
        },
    }

}

pub fn primary_icon_button_style(
    _theme: &Theme, status: button::Status
) -> button::Style {

    match status {

        button::Status::Active => button::Style {
            background: None,
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(), 
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Color::WHITE.scale_alpha(0.08).into()),
            text_color: Color::WHITE,
            border: Border {
                color: Color::WHITE.scale_alpha(0.15),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(BUTTON_PRESS.into()),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(), 
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: TEXT_MUTED,
            border: Border {
                radius: 6.0.into(), 
                ..Default::default()
            },
            ..Default::default()
        },
    }

}
