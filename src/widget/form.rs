struct TextInputStyle;

const ACTIVE_BORDER_COLOR: iced::Color = iced::Color::from_rgb(0.82, 0.82, 0.82);
const PLACEHOLDER_COLOR: iced::Color = iced::Color::from_rgb(0.541, 0.541, 0.541);
const VALUE_COLOR: iced::Color = iced::Color::from_rgb(0.376, 0.424, 0.463);
const SELECTION_COLOR: iced::Color = iced::Color::from_rgb(0.69, 0.69, 0.69);

impl iced::text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> iced::text_input::Style {
        iced::widget::text_input::Style {
            border_color: ACTIVE_BORDER_COLOR,
            border_width: 1,
            border_radius: 4,
            ..Default::default()
        }
    }

    fn focused(&self) -> iced::text_input::Style {
        iced::widget::text_input::Style {
            border_color: crate::style::color::PURPLE,
            border_width: 1,
            border_radius: 4,
            ..Default::default()
        }
    }

    fn placeholder_color(&self) -> iced::Color {
        PLACEHOLDER_COLOR
    }
    fn value_color(&self) -> iced::Color {
        VALUE_COLOR
    }
    fn selection_color(&self) -> iced::Color {
        SELECTION_COLOR
    }
}

pub fn text_input<'a, Message, F>(
    state: &'a mut iced::text_input::State,
    placeholder: &'a str,
    value: &'a str,
    on_change: F,
) -> iced::TextInput<'a, Message>
where
    F: 'static + Fn(String) -> Message,
{
    iced::TextInput::new(state, placeholder, value, on_change)
        .padding(10)
        .size(16)
        .style(TextInputStyle)
}
