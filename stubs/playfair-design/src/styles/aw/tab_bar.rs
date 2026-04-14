use crate::Theme;
use iced_aw::style::status::Status;
use iced_aw::style::tab_bar;

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> tab_bar::Style {
    let _c = theme.colors();
    move |_, _| tab_bar::Style {
        background: None,
        border_color: None,
        border_width: 0.0,
        tab_border_radius: 5.0.into(),
        tab_label_background: iced::Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9)),
        tab_label_border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
        tab_label_border_width: 1.0,
        icon_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
        icon_background: None,
        icon_border_radius: 5.0.into(),
        text_color: iced::Color::BLACK,
    }
}
