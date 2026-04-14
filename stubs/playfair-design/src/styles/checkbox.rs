use crate::Theme;
use iced::widget::checkbox;
use iced::{Background, Border, Color};

fn s() -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(Color::WHITE),
        icon_color: Color::BLACK,
        border: Border { radius: 5.0.into(), width: 1.0, color: Color::from_rgb(0.8, 0.8, 0.8) },
        text_color: Some(Color::BLACK),
    }
}

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, checkbox::Status) -> checkbox::Style {
    let _c = theme.colors(); move |_, _| s()
}
