use crate::Theme;
use iced_aw::style::badge;
use iced_aw::style::status::Status;

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> badge::Style {
    let _c = theme.colors();
    move |_, _| badge::Style {
        background: iced::Background::Color(iced::Color::WHITE),
        border_radius: Some(999.0),
        border_width: 1.0,
        border_color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
        text_color: iced::Color::BLACK,
    }
}
