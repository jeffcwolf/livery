use crate::Theme;
use iced::widget::progress_bar;
use iced::{Background, Border, Color};

fn s() -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
        bar: Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
        border: Border { radius: 5.0.into(), width: 0.0, color: Color::TRANSPARENT },
    }
}

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn subtle(theme: &Theme) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn danger(theme: &Theme) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    let _c = theme.colors(); move |_| s()
}
