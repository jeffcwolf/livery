//! Text input style functions.

use iced::widget::text_input;
use iced::Border;

use crate::Theme;
use crate::borders;

/// Default text input style.
pub fn default(theme: &Theme) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: text_input::Status| match status {
        text_input::Status::Active => text_input::Style {
            background: iced::Background::Color(c.surface_primary),
            border: Border {
                radius: borders::radius::SM.into(),
                color: c.border_default,
                width: borders::width::DEFAULT,
            },
            icon: c.text_tertiary,
            placeholder: c.text_tertiary,
            value: c.text_primary,
            selection: c.surface_selected,
        },
        text_input::Status::Hovered => text_input::Style {
            background: iced::Background::Color(c.surface_primary),
            border: Border {
                radius: borders::radius::SM.into(),
                color: c.border_hover,
                width: borders::width::DEFAULT,
            },
            icon: c.text_tertiary,
            placeholder: c.text_tertiary,
            value: c.text_primary,
            selection: c.surface_selected,
        },
        text_input::Status::Focused { .. } => text_input::Style {
            background: iced::Background::Color(c.surface_primary),
            border: Border {
                radius: borders::radius::SM.into(),
                color: c.border_focus,
                width: borders::width::THICK,
            },
            icon: c.text_secondary,
            placeholder: c.text_tertiary,
            value: c.text_primary,
            selection: c.surface_selected,
        },
        text_input::Status::Disabled => text_input::Style {
            background: iced::Background::Color(c.surface_tertiary),
            border: Border {
                radius: borders::radius::SM.into(),
                color: c.border_default,
                width: borders::width::DEFAULT,
            },
            icon: c.text_tertiary,
            placeholder: c.text_tertiary,
            value: c.text_tertiary,
            selection: c.surface_tertiary,
        },
    }
}
