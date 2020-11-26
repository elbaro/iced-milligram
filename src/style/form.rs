pub struct RadioStyle;

impl iced::radio::StyleSheet for RadioStyle {
    fn active(&self) -> iced::radio::Style {
        iced::radio::Style {
            background: iced::Background::Color(iced::Color::WHITE),
            dot_color: crate::style::PURPLE,
            border_width: 2.0,
            border_color: crate::style::PURPLE,
        }
    }
    fn hovered(&self) -> iced::radio::Style {
        iced::radio::Style {
            background: iced::Background::Color(iced::Color::WHITE),
            dot_color: crate::style::PURPLE,
            border_width: 1.0,
            border_color: crate::style::PURPLE,
        }
    }
}
