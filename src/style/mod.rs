pub mod button;
pub use button::*;

pub mod color;
pub use color::*;

pub mod form;
pub use form::*;

pub const ROBOTO_BOLD: iced::Font = iced::Font::External {
    name: "Roboto Bold",
    bytes: include_bytes!("../Roboto-Bold.ttf"),
};

pub const ROBOTO_LIGHT: iced::Font = iced::Font::External {
    name: "Roboto Light",
    bytes: include_bytes!("../Roboto-Light.ttf"),
};
