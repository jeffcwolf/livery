use crate::Theme;
use iced::widget::slider;
use iced::{Background, Border, Color};

fn s() -> slider::Style {
    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
                Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            ),
            width: 4.0,
            border: Border { radius: 999.0.into(), width: 0.0, color: Color::TRANSPARENT },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style {
    let _c = theme.colors(); move |_, _| s()
}
