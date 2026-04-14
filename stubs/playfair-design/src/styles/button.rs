use crate::Theme;
use iced::widget::button;
use iced::{Background, Border, Color};

fn s() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
        text_color: Color::BLACK,
        border: Border { radius: 5.0.into(), width: 0.0, color: Color::TRANSPARENT },
        shadow: crate::elevation::shadow::none(),
        snap: true,
    }
}

pub fn primary(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let _colors = theme.colors();
    move |_, _| s()
}

pub fn secondary(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let _colors = theme.colors();
    move |_, _| s()
}

pub fn danger(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let _colors = theme.colors();
    move |_, _| s()
}

pub fn success(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let _colors = theme.colors();
    move |_, _| s()
}

pub fn ghost(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let _colors = theme.colors();
    move |_, _| s()
}

pub fn minimal(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let _colors = theme.colors();
    move |_, _| s()
}
