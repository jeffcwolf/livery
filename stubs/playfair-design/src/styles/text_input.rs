use crate::Theme;
use iced::widget::text_input;
use iced::{Background, Border, Color};

fn s() -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::WHITE),
        border: Border { radius: 8.0.into(), width: 1.0, color: Color::from_rgb(0.8, 0.8, 0.8) },
        icon: Color::from_rgb(0.5, 0.5, 0.5),
        placeholder: Color::from_rgb(0.6, 0.6, 0.6),
        value: Color::BLACK,
        selection: Color::from_rgb(0.8, 0.9, 1.0),
    }
}

pub fn default(theme: &Theme) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    let _c = theme.colors(); move |_, _| s()
}

pub fn borderless(theme: &Theme) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    let _c = theme.colors(); move |_, _| s()
}
