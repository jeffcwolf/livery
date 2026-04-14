//! Button style functions.
//!
//! Each function takes a `&Theme` and returns a closure suitable for
//! `button(...).style(closure)`.

use iced::widget::button;
use iced::Border;

use crate::Theme;
use crate::borders;

/// Primary action button (filled background, contrasting text).
pub fn primary(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: button::Status| match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(c.action_primary)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(c.action_primary_hover)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(c.action_primary_active)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(c.action_secondary)),
            text_color: c.text_tertiary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

/// Secondary action button (subtle background, themed text).
pub fn secondary(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: button::Status| match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(c.action_secondary)),
            text_color: c.text_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(c.action_secondary_hover)),
            text_color: c.text_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(c.surface_selected)),
            text_color: c.text_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(c.action_secondary)),
            text_color: c.text_tertiary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

/// Danger button (destructive actions).
pub fn danger(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: button::Status| match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(c.action_danger)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(c.action_danger_hover)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(c.action_danger_hover)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(c.action_secondary)),
            text_color: c.text_tertiary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

/// Success button (positive/confirm actions).
pub fn success(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: button::Status| match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(c.action_success)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(c.action_success_hover)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(c.action_success_hover)),
            text_color: c.text_on_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(c.action_secondary)),
            text_color: c.text_tertiary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

/// Ghost button (transparent background, text only).
pub fn ghost(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: button::Status| match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: c.text_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(c.surface_hover)),
            text_color: c.text_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(c.surface_selected)),
            text_color: c.text_primary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: c.text_tertiary,
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

/// Minimal button (no background, no border, text-only interaction).
pub fn minimal(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: button::Status| match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: c.text_secondary,
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: None,
            text_color: c.text_primary,
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: None,
            text_color: c.text_primary,
            ..Default::default()
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: c.text_tertiary,
            ..Default::default()
        },
    }
}
