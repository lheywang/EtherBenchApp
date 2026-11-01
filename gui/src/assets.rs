//! Fast access for static ressources within the gui app.
//! Loaded into RAM at start to ensure the executable remains fully portable.
//!
//! Accessed through a shared enum access.
//!

use iced::Font;
use iced::font::{Family, Stretch, Style, Weight};
use iced::widget::svg;

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
    Logo, 
    PageDebugger,
    PageHelp,
    PageIo,
    PageMemory,
    PagePlot,
    PageProgrammer,
    PageRouting,
    PageSequences,
    PageSettings,
    PageFiles,
    CircleFull, 
    CircleEmpty,
    Player,
}

impl Icon {
    pub fn handle(&self) -> svg::Handle {
        match self {
            // Common items
            Icon::Logo => svg::Handle::from_memory(include_bytes!("../assets/icons/logo/logo.svg").as_slice()),

            // Pages
            Icon::PageDebugger => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/debugger.svg").as_slice(),
            ),
            Icon::PageHelp => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/help.svg").as_slice(),
            ),
            Icon::PageIo => {
                svg::Handle::from_memory(include_bytes!("../assets/icons/navbar/io.svg").as_slice())
            }
            Icon::PageMemory => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/memory.svg").as_slice(),
            ),
            Icon::PagePlot => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/plots.svg").as_slice(),
            ),
            Icon::PageProgrammer => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/programmer.svg").as_slice(),
            ),
            Icon::PageRouting => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/router.svg").as_slice(),
            ),
            Icon::PageSequences => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/sequences.svg").as_slice(),
            ),
            Icon::PageSettings => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/settings.svg").as_slice(),
            ),
            Icon::PageFiles => svg::Handle::from_memory(
                include_bytes!("../assets/icons/navbar/files.svg").as_slice(),
            ),

            // Menubar status
            Icon::CircleEmpty => svg::Handle::from_memory(
                include_bytes!("../assets/icons/menubar/empty.svg").as_slice(),
            ),
            Icon::CircleFull => svg::Handle::from_memory(
                include_bytes!("../assets/icons/menubar/full.svg").as_slice(),
            ),

            // StatusBar elements
            Icon::Player => svg::Handle::from_memory(
                include_bytes!("../assets/icons/statusbar/play.svg").as_slice(),
            ),
        }
    }

    pub fn widget<'a>(&self) -> svg::Svg<'_> {
        svg::Svg::new(self.handle())
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
    const FAMILY: Family = Family::Name("Cinzel");

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
