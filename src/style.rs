/// list of components:
/// - [ ] typography
/// - [ ] heading
/// - [ ] blockquotes
/// - [ ] buttons
/// - [ ] lists
/// - [ ] forms
/// - [ ] table
/// - [ ] grid
/// - [ ] code

fn purple() -> iced::Color {
    iced::Color::from_rgb(0.608, 0.302, 0.792)
}

enum ButtonKind {
    Default,  // background=color, fore-text=white
    Outlined, // border=colored, background=transparent, fore-text=color
    Clear,    // no border, background=transparent, fore-text=color
}

enum ButtonSize {
    Normal, // font 11px font-weight 700 height 38px spacing 1px border 1px radius 4px
    Small,  // font 8px height 28px padding-lr 15px
    Large,  // font 14px height 45px padding-lrud 2px
}

enum ButtonColor {
    Default,
    Black,
}

pub struct ButtonStyle {
    kind: ButtonKind,
    size: ButtonSize,
    color: ButtonColor,
}

impl iced::button::StyleSheet for ButtonStyle {
    fn active(&self) -> iced::button::Style {
        let color = if let ButtonColor::Default = self.color {
            purple()
        } else {
            iced::Color::BLACK
        };

        match self.kind {
            ButtonKind::Default => iced::button::Style {
                background: Some(iced::Background::Color(color)),
                border_radius: 4,
                border_width: 0,
                text_color: iced::Color::WHITE,
                ..iced::button::Style::default()
            },
            ButtonKind::Outlined => iced::button::Style {
                background: None,
                border_radius: 4,
                border_width: 1,
                border_color: color,
                text_color: color,
                ..iced::button::Style::default()
            },
            ButtonKind::Clear => iced::button::Style {
                background: None,
                border_radius: 4,
                border_width: 0,
                text_color: color,
                ..iced::button::Style::default()
            },
        }
    }
    fn hovered(&self) -> iced::button::Style {
        self.active()
    }
}

pub fn button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Default,
        size: ButtonSize::Normal,
        color: ButtonColor::Default,
    }
}
pub fn large_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Default,
        size: ButtonSize::Large,
        color: ButtonColor::Default,
    }
}
pub fn small_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Default,
        size: ButtonSize::Small,
        color: ButtonColor::Default,
    }
}
pub fn outlined_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Outlined,
        size: ButtonSize::Normal,
        color: ButtonColor::Default,
    }
}
pub fn outlined_large_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Outlined,
        size: ButtonSize::Large,
        color: ButtonColor::Default,
    }
}
pub fn outlined_small_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Outlined,
        size: ButtonSize::Small,
        color: ButtonColor::Default,
    }
}
pub fn clear_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Clear,
        size: ButtonSize::Normal,
        color: ButtonColor::Default,
    }
}
pub fn clear_large_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Clear,
        size: ButtonSize::Large,
        color: ButtonColor::Default,
    }
}
pub fn clear_small_button() -> ButtonStyle {
    ButtonStyle {
        kind: ButtonKind::Clear,
        size: ButtonSize::Small,
        color: ButtonColor::Default,
    }
}
