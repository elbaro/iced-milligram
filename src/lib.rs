/// This crate provides a milligram theme in iced-rs.
///
/// You can use the theme in 2 ways:
///
/// ```
/// // 1 (recommended)
/// let button = iced_milligram::widget::outlined_button("text");
///
/// // 3 (builder)
/// use iced_milligram::style::MilligramButton; // or iced_milligram::prelude::*;
/// let button1 = iced::Button::new(..).outlined_default().large();
/// let button2 = iced::Button::new(..).outlined().black().normal_size();
///
/// // 3 (manual)
/// let button = iced::Button::new(iced::Text::new(..).color(..).size(..)..).style(iced_milligram::style::ButtonStyle{..}).width(iced_milligram::style::BUTTON_WIDTH).height(..);
///
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
pub mod style;
pub mod widget;
