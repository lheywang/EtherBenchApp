/// Define the central styling control.
/// This define all the colors on the UI.
use iced::widget::container;
use iced::{Border, Color, Theme, border};

/// Standard palette for all the elements
pub const BACKGROUND_MAIN: Color = Color::from_rgb(0.12, 0.12, 0.12);
pub const BACKGROUND_PANEL: Color = Color::from_rgb(0.16, 0.16, 0.16);
pub const BACKGROUND_BORDER: Color = Color::from_rgb(0.2, 0.2, 0.2);
pub const ACCENT: Color = Color::from_rgb(0.0, 0.47, 0.84);
pub const TEXT_MUTED: Color = Color::from_rgb(0.6, 0.6, 0.6);
pub const TEXT_ALERT: Color = Color::from_rgb(0.9, 0.3, 0.3);

/// Style for the navbar
pub fn navbar_style(theme: &Theme) -> container::Style {
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

pub fn statusbar_style(theme: &Theme) -> container::Style {
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

pub fn menubar_style(theme: &Theme) -> container::Style {
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
