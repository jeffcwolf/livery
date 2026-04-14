use crate::Theme;
use iced::widget::radio;
use iced::{Background, Color};

fn s() -> radio::Style {
    radio::Style {
        background: Background::Color(Color::WHITE),
        dot_color: Color::from_rgb(0.5, 0.5, 0.5),
        border_width: 1.5,
        border_color: Color::from_rgb(0.8, 0.8, 0.8),
        text_color: Some(Color::BLACK),
    }
}

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, radio::Status) -> radio::Style {
    let _c = theme.colors(); move |_, _| s()
}
