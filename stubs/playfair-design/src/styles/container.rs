use crate::Theme;
use iced::widget::container;
use iced::{Background, Border, Color};

fn s() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
        text_color: Some(Color::BLACK),
        border: Border { radius: 8.0.into(), width: 0.0, color: Color::TRANSPARENT },
        shadow: crate::elevation::shadow::none(),
        snap: true,
    }
}

pub fn card(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn panel(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn modal(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn sidebar(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn section(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn inline(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn success(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn warning(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn error(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
pub fn info(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let _c = theme.colors(); move |_| s()
}
