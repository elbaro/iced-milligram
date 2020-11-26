///
/// 3 button styles
/// 3 button sizes
/// 2 button colors
///
/// => 3*3*2 = 18 combinations

pub const BUTTON_HEIGHT: u16 = 38;
pub const BUTTON_TEXT_SIZE: u16 = 16;

use crate::style::button::*;

pub fn create_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
    text_color: iced::Color,
    style: ButtonStyle,
    scale: f32,
) -> iced::Button<'a, Message> {
    let button = iced::Button::new(
        state,
        iced::Text::new(label)
            .font(crate::style::ROBOTO_BOLD)
            .color(text_color)
            .horizontal_alignment(iced::HorizontalAlignment::Center)
            .vertical_alignment(iced::VerticalAlignment::Center)
            .size((BUTTON_TEXT_SIZE as f32 * scale) as u16),
    )
    .padding((10.0 * scale) as u16)
    .style(style)
    .height(iced::Length::Units((BUTTON_HEIGHT as f32 * scale) as u16))
    .width(iced::Length::Shrink);

    button
}

pub fn button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(state, label, iced::Color::WHITE, ButtonStyle::Default, 1.0)
}

pub fn outlined_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(
        state,
        label,
        crate::style::PURPLE,
        ButtonStyle::Outlined,
        1.0,
    )
}

pub fn clear_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(state, label, crate::style::PURPLE, ButtonStyle::Clear, 1.0)
}

pub fn large_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(state, label, iced::Color::WHITE, ButtonStyle::Default, 2.0)
}

pub fn large_outlined_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(
        state,
        label,
        crate::style::PURPLE,
        ButtonStyle::Outlined,
        2.0,
    )
}

pub fn large_clear_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(state, label, crate::style::PURPLE, ButtonStyle::Clear, 2.0)
}

pub fn small_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(state, label, iced::Color::WHITE, ButtonStyle::Default, 0.5)
}

pub fn small_outlined_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(
        state,
        label,
        crate::style::PURPLE,
        ButtonStyle::Outlined,
        0.5,
    )
}

pub fn small_clear_button<'a, Message: Clone>(
    state: &'a mut iced::button::State,
    label: &str,
) -> iced::Button<'a, Message> {
    create_button(state, label, crate::style::PURPLE, ButtonStyle::Clear, 0.5)
}
