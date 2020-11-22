// pub fn container<
//     'a,
//     Message,
//     Renderer: iced::widget::container::Renderer,
//     T: Into<iced::Element<'a, Message, Renderer>>,
// >(
//     content: T,
// ) -> iced::Container<'a, Message> {
//     iced::Container::new(content).width(iced::Length::Units(800))
// }

pub fn paragraph<'a, Message: 'a, C: Into<iced::Element<'a, Message>>>(
    content: C,
) -> iced::Column<'a, Message> {
    iced::Column::new()
        .push(content)
        .push(iced::Space::with_height(iced::Length::Units(25)))
}
