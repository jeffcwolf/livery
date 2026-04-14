use crate::Theme;
use iced_aw::style::number_input;
use iced_aw::style::status::Status;

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> number_input::Style {
    let _c = theme.colors();
    move |_, _| number_input::Style {
        button_background: Some(iced::Background::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))),
        icon_color: iced::Color::WHITE,
    }
}
