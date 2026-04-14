use crate::Theme;
use iced::widget::toggler;
use iced::{Background, Color};

fn s() -> toggler::Style {
    toggler::Style {
        background: Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground: Background::Color(Color::WHITE),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(Color::BLACK),
        border_radius: None,
        padding_ratio: 0.0,
    }
}

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, toggler::Status) -> toggler::Style {
    let _c = theme.colors(); move |_, _| s()
}
