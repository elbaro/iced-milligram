struct TextInputStyle;

const ACTIVE_BORDER_COLOR: iced::Color = iced::Color::from_rgb(0.82, 0.82, 0.82);
const PLACEHOLDER_COLOR: iced::Color = iced::Color::from_rgb(0.541, 0.541, 0.541);
const VALUE_COLOR: iced::Color = iced::Color::from_rgb(0.376, 0.424, 0.463);
const SELECTION_COLOR: iced::Color = iced::Color::from_rgb(0.69, 0.69, 0.69);

impl iced::text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> iced::text_input::Style {
        iced::widget::text_input::Style {
            border_color: ACTIVE_BORDER_COLOR,
            border_width: 1.0,
            border_radius: 4.0,
            ..Default::default()
        }
    }

    fn focused(&self) -> iced::text_input::Style {
        iced::widget::text_input::Style {
            border_color: crate::style::color::PURPLE,
            border_width: 1.0,
            border_radius: 4.0,
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

pub fn text_input<'a, Message: Clone, F>(
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

pub fn labeled_text_input<'a, Message: 'a + Clone, F>(
    label: &'a str,
    state: &'a mut iced::text_input::State,
    placeholder: &'a str,
    value: &'a str,
    on_change: F,
) -> iced::Column<'a, Message>
where
    F: 'static + Fn(String) -> Message,
{
    let input: iced::Element<_> = text_input(state, placeholder, value, on_change).into();
    iced::Column::new()
        .spacing(5)
        .push(crate::widget::bold_text(label))
        .push(input)
}

// pub fn radio() {

// }

pub fn radio<F, V, Message: Clone>(
    value: V,
    label: impl Into<String>,
    selected: Option<V>,
    f: F,
) -> iced::widget::Radio<Message>
where
    V: Eq + Copy,
    F: 'static + Fn(V) -> Message,
{
    iced::widget::Radio::new(value, label, selected, f).style(crate::style::form::RadioStyle)
}

pub fn checkbox<F, Message>(
    is_checked: bool,
    label: impl Into<String>,
    f: F,
) -> iced::Checkbox<Message>
where
    F: 'static + Fn(bool) -> Message,
{
    iced::widget::Checkbox::new(is_checked, label, f)
}
