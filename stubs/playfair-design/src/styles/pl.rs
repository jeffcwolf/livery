//! Pick list style functions.

use iced::widget::pick_list;
use iced::Border;

use crate::Theme;
use crate::borders;

/// Standard pick list style.
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: pick_list::Status| match status {
        pick_list::Status::Active => pick_list::Style {
            background: iced::Background::Color(c.surface_primary),
            border: Border {
                radius: borders::radius::SM.into(),
                color: c.border_default,
                width: borders::width::DEFAULT,
            },
            text_color: c.text_primary,
            placeholder_color: c.text_tertiary,
            handle_color: c.text_secondary,
        },
        pick_list::Status::Hovered => pick_list::Style {
            background: iced::Background::Color(c.surface_hover),
            border: Border {
                radius: borders::radius::SM.into(),
                color: c.border_hover,
                width: borders::width::DEFAULT,
            },
            text_color: c.text_primary,
            placeholder_color: c.text_tertiary,
            handle_color: c.text_primary,
        },
        pick_list::Status::Opened { .. } => pick_list::Style {
            background: iced::Background::Color(c.surface_primary),
            border: Border {
                radius: borders::radius::SM.into(),
                color: c.border_focus,
                width: borders::width::THICK,
            },
            text_color: c.text_primary,
            placeholder_color: c.text_tertiary,
            handle_color: c.text_primary,
        },
    }
}
