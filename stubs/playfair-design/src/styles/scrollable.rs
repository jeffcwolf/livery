use crate::Theme;
use iced::widget::{container, scrollable};
use iced::{Background, Border, Color, Shadow, Vector};

fn s() -> scrollable::Style {
    let rail = scrollable::Rail {
        background: None,
        border: Border { radius: 999.0.into(), width: 0.0, color: Color::TRANSPARENT },
        scroller: scrollable::Scroller {
            background: Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
            border: Border { radius: 999.0.into(), width: 0.0, color: Color::TRANSPARENT },
        },
    };
    scrollable::Style {
        container: container::Style {
            background: None, text_color: None,
            border: Border { radius: 0.0.into(), width: 0.0, color: Color::TRANSPARENT },
            shadow: Shadow { color: Color::TRANSPARENT, offset: Vector::ZERO, blur_radius: 0.0 },
            snap: false,
        },
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(Color::WHITE),
            border: Border { radius: 5.0.into(), width: 0.0, color: Color::TRANSPARENT },
            shadow: Shadow { color: Color::TRANSPARENT, offset: Vector::ZERO, blur_radius: 0.0 },
            icon: Color::from_rgb(0.5, 0.5, 0.5),
        },
    }
}

pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style {
    let _c = theme.colors(); move |_, _| s()
}

pub fn thin(theme: &Theme) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style {
    let _c = theme.colors(); move |_, _| s()
}
