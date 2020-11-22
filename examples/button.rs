use iced::Sandbox;

struct App {
    state: iced::button::State,
}

impl iced::Sandbox for App {
    type Message = ();
    fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }
    fn title(&self) -> String {
        "Example: Button".to_string()
    }
    fn update(&mut self, _message: Self::Message) {}
    fn view(&mut self) -> iced::Element<Self::Message> {
        let button = iced::Button::new(&mut self.state, iced::Text::new("DEFAULT BUTTON"))
            .style(iced_milligram_theme::style::outlined_button())
            .height(iced::Length::Units(45));
        iced::Container::new(button)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
fn main() {
    App::run(iced::Settings::default())
}
