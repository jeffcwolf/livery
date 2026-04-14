use crate::Theme;
use iced_aw::style::card;
use iced_aw::style::status::Status;

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> card::Style {
    let _c = theme.colors();
    let bg = iced::Background::Color(iced::Color::WHITE);
    move |_, _| card::Style {
        background: bg,
        border_radius: 8.0,
        border_width: 1.0,
        border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
        head_background: bg,
        head_text_color: iced::Color::BLACK,
        body_background: bg,
        body_text_color: iced::Color::BLACK,
        foot_background: bg,
        foot_text_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
        close_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
    }
}
