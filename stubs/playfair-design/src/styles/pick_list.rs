use crate::Theme;
use iced::widget::pick_list;
use iced::{Background, Border, Color};

fn s() -> pick_list::Style {
    pick_list::Style {
        text_color: Color::BLACK,
        placeholder_color: Color::from_rgb(0.6, 0.6, 0.6),
        handle_color: Color::from_rgb(0.5, 0.5, 0.5),
        background: Background::Color(Color::WHITE),
        border: Border { radius: 8.0.into(), width: 1.0, color: Color::from_rgb(0.8, 0.8, 0.8) },
    }
}

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style {
    let _c = theme.colors(); move |_, _| s()
}
