use iced::Sandbox;

#[derive(Default)]
struct App {
    button1: iced::button::State,
    button2: iced::button::State,
    button3: iced::button::State,
    button4: iced::button::State,
    text_input1: iced::text_input::State,
    text_input2: iced::text_input::State,
    text_value: String,
}

#[derive(Clone, Debug)]
enum Message {
    ButtonPressed,
    TextChanged(String),
}

impl iced::Sandbox for App {
    type Message = Message;
    fn new() -> Self {
        Self::default()
    }
    fn title(&self) -> String {
        "Example: Button".to_string()
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextChanged(text) => {
                self.text_value = text;
            }
            _ => {}
        }
    }
    fn view(&mut self) -> iced::Element<Self::Message> {
        let col = iced::Column::new()
            .push(iced_milligram_theme::widget::h1("Getting Started"))
            .push(iced_milligram_theme::widget::paragraph_text(
                "This is iced-milligram-theme.",
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
            .push(iced_milligram_theme::widget::paragraph(
                iced_milligram_theme::widget::labeled_text_input::<Self::Message, _>(
                    "Label",
                    &mut self.text_input1,
                    "placeholder",
                    &self.text_value,
                    |s| Message::TextChanged(s),
                ),
            ))
            .push(iced_milligram_theme::widget::paragraph(
                iced_milligram_theme::widget::text_input::<Self::Message, _>(
                    &mut self.text_input2,
                    "TextInput without label",
                    &self.text_value,
                    |s| Message::TextChanged(s),
                ),
            ))
            .push(iced_milligram_theme::widget::h1("Table (text-only)"))
            .push(iced_milligram_theme::widget::text_table(
                &["Name", "Age", "Height"],
                &[100, 100, 100],
                vec![
                    &["Stephen Curry", "27", "1,91", "Akron, OH"],
                    &["Klay Thompson", "25", "2,01", "	Los Angeles, CA"],
                ],
                // &[
                //     &["Stephen Curry", "27", "1,91", "Akron, OH"],
                //     &["Klay Thompson", "25", "2,01", "	Los Angeles, CA"],
                // ],
            ));

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
        window: iced::window::Settings {
            size: (1200, 1200),
            resizable: true,
            decorations: true,
        },
        ..Default::default()
    })
}
