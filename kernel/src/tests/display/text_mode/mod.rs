pub use self::display_facade::DisplayFacade;
pub use self::dummy_display::DummyDisplay;

pub mod dummy_display;
pub mod display_facade;
mod buffer;
mod scroll_printer;