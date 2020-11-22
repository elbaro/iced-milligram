// margin bottom 20px

pub fn text(label: &str) -> iced::Text {
    iced::Text::new(label)
        .color(crate::style::DARK_GREY)
        .size(16)
        .font(crate::style::ROBOTO_LIGHT)
}

pub fn bold_text(label: &str) -> iced::Text {
    iced::Text::new(label)
        .color(crate::style::DARK_GREY)
        .size(16)
        .font(crate::style::ROBOTO_BOLD)
}

pub fn paragraph_text<'a, Message: 'a>(txt: &str) -> iced::Column<'a, Message> {
    iced::Column::new()
        .push(text(txt))
        .push(iced::Space::with_height(iced::Length::Units(25)))
}

pub fn h<'a, Message: 'a>(label: &'a str, font_size: u16) -> iced::Column<'a, Message> {
    iced::Column::new()
        .push(
            iced::Text::new(label)
                .font(crate::style::ROBOTO_LIGHT)
                .color(crate::style::DARK_GREY)
                .size(font_size),
        )
        .push(iced::Space::with_height(iced::Length::Units(20)))
}

pub fn h1<'a, Message: 'a>(label: &'a str) -> iced::Column<'a, Message> {
    h(label, 46)
}

pub fn h2<'a, Message: 'a>(label: &'a str) -> iced::Column<'a, Message> {
    h(label, 36)
}

pub fn h3<'a, Message: 'a>(label: &'a str) -> iced::Column<'a, Message> {
    h(label, 28)
}

pub fn h4<'a, Message: 'a>(label: &'a str) -> iced::Column<'a, Message> {
    h(label, 22)
}

pub fn h5<'a, Message: 'a>(label: &'a str) -> iced::Column<'a, Message> {
    h(label, 18)
}

pub fn h6<'a, Message: 'a>(label: &'a str) -> iced::Column<'a, Message> {
    h(label, 16)
}
