pub enum ButtonStyle {
    Default,
    Outlined,
    Clear,
}

impl iced::button::StyleSheet for ButtonStyle {
    fn active(&self) -> iced::button::Style {
        match self {
            ButtonStyle::Default => iced::button::Style {
                background: Some(iced::Background::Color(crate::style::PURPLE)),
                border_radius: 4,
                border_width: 0,
                text_color: iced::Color::WHITE,
                ..iced::button::Style::default()
            },
            ButtonStyle::Outlined => iced::button::Style {
                background: None,
                border_radius: 4,
                border_width: 1,
                border_color: crate::style::PURPLE,
                text_color: crate::style::PURPLE,
                ..iced::button::Style::default()
            },
            ButtonStyle::Clear => iced::button::Style {
                background: None,
                border_radius: 4,
                border_width: 0,
                text_color: crate::style::PURPLE,
                ..iced::button::Style::default()
            },
        }
    }

    /// bug: text_color does not affect the text color inside the button
    fn hovered(&self) -> iced::button::Style {
        match self {
            ButtonStyle::Default => iced::button::Style {
                background: Some(iced::Background::Color(crate::style::DARK_GREY)),
                border_radius: 4,
                border_width: 0,
                text_color: iced::Color::WHITE,
                ..iced::button::Style::default()
            },
            ButtonStyle::Outlined => iced::button::Style {
                background: Some(iced::Background::Color(crate::style::DIMMED_PURPLE)),
                border_radius: 4,
                border_width: 1,
                border_color: crate::style::DARK_GREY,
                // text_color: crate::style::DARK_GREY,
                ..iced::button::Style::default()
            },
            ButtonStyle::Clear => iced::button::Style {
                background: Some(iced::Background::Color(crate::style::DIMMED_PURPLE)),
                border_radius: 4,
                border_width: 0,
                // text_color: crate::style::DARK_GREY,
                ..iced::button::Style::default()
            },
        }
    }
}
