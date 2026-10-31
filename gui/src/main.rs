/// Main entry point of the app.
// Iced imports
use iced;

#[macro_use]
mod assets;
mod app;
mod style;

pub mod components;
pub mod pages;

pub fn main() -> iced::Result {
    let app = iced::application(
        app::EtherBenchApp::new,
        app::EtherBenchApp::update,
        app::EtherBenchApp::view,
    )
    .title("EtherBench - GUI Application");

    load_custom_fonts!(app).run()
}
