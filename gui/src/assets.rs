//! Fast access for static ressources within the gui app.
//! Loaded into RAM at start to ensure the executable remains fully portable.
//!
//! Accessed through a shared enum access.
//!

use iced::Font;
use iced::font::{Family, Stretch, Style, Weight};
use iced::widget::image;

// Loading FiraCode fonts
pub const FIRA_CODE_BOLD_BYTES: &[u8] =
    include_bytes!("../assets/fonts/firacode/FiraCode-Bold.ttf");
pub const FIRA_CODE_LIGHT_BYTES: &[u8] =
    include_bytes!("../assets/fonts/firacode/FiraCode-Light.ttf");
pub const FIRA_CODE_MEDIUM_BYTES: &[u8] =
    include_bytes!("../assets/fonts/firacode/FiraCode-Medium.ttf");
pub const FIRA_CODE_REGULAR_BYTES: &[u8] =
    include_bytes!("../assets/fonts/firacode/FiraCode-Regular.ttf");
pub const FIRA_CODE_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../assets/fonts/firacode/FiraCode-SemiBold.ttf");

// Loading Cinzel fonts
pub const CINZEL_BLACK_BYTES: &[u8] = include_bytes!("../assets/fonts/cinzel/Cinzel-Black.ttf");
pub const CINZEL_BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/cinzel/Cinzel-Bold.ttf");
pub const CINZEL_EXTRABOLD_BYTES: &[u8] =
    include_bytes!("../assets/fonts/cinzel/Cinzel-ExtraBold.ttf");
pub const CINZEL_MEDIUM_BYTES: &[u8] = include_bytes!("../assets/fonts/cinzel/Cinzel-Medium.ttf");
pub const CINZEL_REGULAR_BYTES: &[u8] = include_bytes!("../assets/fonts/cinzel/Cinzel-Regular.ttf");
pub const CINZEL_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../assets/fonts/cinzel/Cinzel-SemiBold.ttf");

/// Define the enum used to access the different icons
pub enum Icon {
    Debugger,
    Help,
    Home,
    Io,
    Memory,
    Plot,
    Programmer,
    Routing,
    Sequences,
    Settings,
}

impl Icon {
    pub fn handle(&self) -> image::Handle {
        match self {
            Icon::Debugger => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/debugger.png").as_slice(),
            ),
            Icon::Help => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/help.png").as_slice(),
            ),
            Icon::Home => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/home.png").as_slice(),
            ),
            Icon::Io => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/io.png").as_slice(),
            ),
            Icon::Memory => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/memory.png").as_slice(),
            ),
            Icon::Plot => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/plot.png").as_slice(),
            ),
            Icon::Programmer => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/programmer.png").as_slice(),
            ),
            Icon::Routing => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/routing.png").as_slice(),
            ),
            Icon::Sequences => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/sequences.png").as_slice(),
            ),
            Icon::Settings => image::Handle::from_bytes(
                include_bytes!("../assets/icons/navbar/settings.png").as_slice(),
            ),
        }
    }

    pub fn widget(&self) -> image::Image {
        image::Image::new(self.handle())
    }
}

/// Define the different fonts
pub struct FiraCode;

impl FiraCode {
    const FAMILY: Family = Family::Name("Fira Code");

    pub const REGULAR: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: Style::Normal,
    };

    pub const BOLD: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Bold,
        ..Self::REGULAR
    };

    pub const LIGHT: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Light,
        ..Self::REGULAR
    };

    pub const MEDIUM: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Medium,
        ..Self::REGULAR
    };

    pub const EXTRABOLD: Font = Font {
        family: Self::FAMILY,
        weight: Weight::ExtraBold,
        ..Self::REGULAR
    };

}

pub struct Cinzel;

impl Cinzel {
    const FAMILY: Family =  Family::Name("Cinzel");

    pub const REGULAR: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: Style::Normal,
    };

    pub const BOLD: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Bold,
        ..Self::REGULAR
    };

    pub const LIGHT: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Light,
        ..Self::REGULAR
    };

    pub const MEDIUM: Font = Font {
        family: Self::FAMILY,
        weight: Weight::Medium,
        ..Self::REGULAR
    };

    pub const EXTRABOLD: Font = Font {
        family: Self::FAMILY,
        weight: Weight::ExtraBold,
        ..Self::REGULAR
    };
}

/// Add an extension to the application class to act as a Loader
macro_rules! load_custom_fonts {
    ($app:expr) => {
        $app.font($crate::assets::FIRA_CODE_BOLD_BYTES)
            .font($crate::assets::FIRA_CODE_LIGHT_BYTES)
            .font($crate::assets::FIRA_CODE_MEDIUM_BYTES)
            .font($crate::assets::FIRA_CODE_REGULAR_BYTES)
            .font($crate::assets::FIRA_CODE_SEMIBOLD_BYTES)
            .font($crate::assets::CINZEL_BLACK_BYTES)
            .font($crate::assets::CINZEL_BOLD_BYTES)
            .font($crate::assets::CINZEL_EXTRABOLD_BYTES)
            .font($crate::assets::CINZEL_MEDIUM_BYTES)
            .font($crate::assets::CINZEL_REGULAR_BYTES)
            .font($crate::assets::CINZEL_SEMIBOLD_BYTES)
    };
}