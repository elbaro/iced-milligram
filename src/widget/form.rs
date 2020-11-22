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
}
