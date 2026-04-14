//! Container style functions.
//!
//! Each function takes a `&Theme` and returns a closure suitable for
//! `container(...).style(closure)`.

use iced::widget::container;
use iced::Border;

use crate::Theme;
use crate::borders;

/// Card container (raised surface with rounded corners).
pub fn card(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_secondary)),
        border: Border {
            radius: borders::radius::MD.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Panel container (secondary surface, subtle border).
pub fn panel(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_secondary)),
        border: Border {
            radius: borders::radius::SM.into(),
            color: c.border_default,
            width: borders::width::HAIRLINE,
        },
        ..Default::default()
    }
}

/// Sidebar container (vertical navigation background).
pub fn sidebar(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_secondary)),
        ..Default::default()
    }
}

/// Section container (subtle tertiary surface).
pub fn section(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_tertiary)),
        border: Border {
            radius: borders::radius::SM.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Info container (highlighted information callout).
pub fn info(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_selected)),
        border: Border {
            radius: borders::radius::SM.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Warning container (amber/caution callout).
pub fn warning(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_secondary)),
        border: Border {
            radius: borders::radius::SM.into(),
            color: c.action_warning,
            width: borders::width::DEFAULT,
        },
        ..Default::default()
    }
}

/// Success container (green/positive callout).
pub fn success(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_secondary)),
        border: Border {
            radius: borders::radius::SM.into(),
            color: c.action_success,
            width: borders::width::DEFAULT,
        },
        ..Default::default()
    }
}

/// Inline container (no border, transparent or minimal background).
pub fn inline(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(c.surface_primary)),
        ..Default::default()
    }
}
