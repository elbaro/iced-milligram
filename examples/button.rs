use iced::Sandbox;

#[derive(Default)]
struct App {
    button1: iced::button::State,
    button2: iced::button::State,
    button3: iced::button::State,
    button4: iced::button::State,
    button5: iced::button::State,
    button6: iced::button::State,
    text_input: iced::text_input::State,
}

#[derive(Clone, Debug)]
enum Message {
    ButtonPressed,
}

impl iced::Sandbox for App {
    type Message = Message;
    fn new() -> Self {
        Self::default()
    }
    fn title(&self) -> String {
        "Example: Button".to_string()
    }
    fn update(&mut self, _message: Self::Message) {}
    fn view(&mut self) -> iced::Element<Self::Message> {
        let col = iced::Column::new()
            .push(iced_milligram_theme::widget::h1("Getting Started"))
            .push(iced_milligram_theme::widget::paragraph_text(
                "This is a port of ",
            ))
            .push(iced_milligram_theme::widget::h1("Buttons"))
            .push(iced_milligram_theme::widget::paragraph(
                iced_milligram_theme::widget::large_button(&mut self.button1, "large button")
                    .on_press(Message::ButtonPressed),
            ))
            .push(iced_milligram_theme::widget::paragraph(
                iced_milligram_theme::widget::large_outlined_button(
                    &mut self.button2,
                    "large outlined button",
                )
                .on_press(Message::ButtonPressed),
            ))
            .push(iced_milligram_theme::widget::paragraph(
                iced_milligram_theme::widget::large_clear_button(
                    &mut self.button3,
                    "large cleard button",
                )
                .on_press(Message::ButtonPressed),
            ))
            .push(iced_milligram_theme::widget::paragraph(
                iced_milligram_theme::widget::button(&mut self.button4, "normal default button")
                    .on_press(Message::ButtonPressed),
            ))
            .push(iced_milligram_theme::widget::h1("Text List"))
            .push(iced_milligram_theme::widget::unordered_list(&[
                "Apple", "Banana", "Orange",
            ]))
            .push(
                iced_milligram_theme::widget::text_input::<Self::Message, _>(
                    &mut self.text_input,
                    "placeholder",
                    "value",
                    |_s| Message::ButtonPressed,
                ),
            );

        iced::Container::new(iced::Container::new(col).width(iced::Length::Units(800)))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
fn main() {
    use iced::Settings;

    App::run(Settings {
        // default_font: Some(iced_milligram_theme::style::robot()),
        ..Default::default()
    })
}
