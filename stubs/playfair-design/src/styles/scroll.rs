//! Scrollable style functions.

use iced::widget::scrollable;
use iced::{Border, Color};

use crate::Theme;
use crate::borders;

/// Standard scrollable style.
pub fn standard(
    theme: &Theme,
) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme, status: scrollable::Status| {
        let rail = scrollable::Rail {
            background: Some(iced::Background::Color(c.surface_secondary)),
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            scroller: scrollable::Scroller {
                background: iced::Background::Color(c.border_default),
                border: Border {
                    radius: borders::radius::SM.into(),
                    ..Default::default()
                },
            },
        };

        let hovered_rail = scrollable::Rail {
            background: Some(iced::Background::Color(c.surface_tertiary)),
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            scroller: scrollable::Scroller {
                background: iced::Background::Color(c.border_hover),
                border: Border {
                    radius: borders::radius::SM.into(),
                    ..Default::default()
                },
            },
        };

        let dragged_rail = scrollable::Rail {
            background: Some(iced::Background::Color(c.surface_tertiary)),
            border: Border {
                radius: borders::radius::SM.into(),
                ..Default::default()
            },
            scroller: scrollable::Scroller {
                background: iced::Background::Color(c.action_primary),
                border: Border {
                    radius: borders::radius::SM.into(),
                    ..Default::default()
                },
            },
        };

        let auto_scroll = scrollable::AutoScroll {
            background: iced::Background::Color(c.surface_secondary),
            border: Border::default(),
            shadow: iced::Shadow::default(),
            icon: Color::TRANSPARENT,
        };

        match status {
            scrollable::Status::Active { .. } => scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: rail,
                horizontal_rail: rail,
                gap: None,
                auto_scroll,
            },
            scrollable::Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                ..
            } => scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_hovered {
                    hovered_rail
                } else {
                    rail
                },
                horizontal_rail: if is_horizontal_scrollbar_hovered {
                    hovered_rail
                } else {
                    rail
                },
                gap: None,
                auto_scroll,
            },
            scrollable::Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                ..
            } => scrollable::Style {
                container: iced::widget::container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_rail
                } else {
                    rail
                },
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_rail
                } else {
                    rail
                },
                gap: None,
                auto_scroll,
            },
        }
    }
}
