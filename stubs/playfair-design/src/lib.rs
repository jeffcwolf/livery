//! Vendored snapshot of the Playfair design system for stub builds.
//!
//! Provides semantic colours, spacing constants, border tokens, and
//! style functions for Iced widgets. On developer machines the real
//! `playfair-core` crate is used via `.cargo/config.toml` path overrides.

pub mod styles;

/// Re-exports for convenient single-import usage.
pub mod prelude {
    pub use crate::styles;
    pub use crate::{
        SemanticColors, Theme, ThemeState,
        borders, spacing,
    };
}

/// Spacing constants used throughout the UI.
pub mod spacing {
    /// Small spacing (4px).
    pub const SM: f32 = 4.0;
    /// Medium spacing (8px).
    pub const MD: f32 = 8.0;
    /// Large spacing (16px).
    pub const LG: f32 = 16.0;
    /// Extra-large spacing (24px).
    pub const XL: f32 = 24.0;
}

/// Border radius and width constants.
pub mod borders {
    /// Border radius presets.
    pub mod radius {
        /// Small radius (4px).
        pub const SM: f32 = 4.0;
        /// Medium radius (8px).
        pub const MD: f32 = 8.0;
        /// Large radius (12px).
        pub const LG: f32 = 12.0;
    }
    /// Border width presets.
    pub mod width {
        /// Hairline border (0.5px).
        pub const HAIRLINE: f32 = 0.5;
        /// Default border (1px).
        pub const DEFAULT: f32 = 1.0;
        /// Thick border (2px).
        pub const THICK: f32 = 2.0;
    }
}

use iced::Color;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

/// Available colour themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Theme {
    /// Light theme with neutral greys.
    Light,
    /// Dark theme for low-light environments.
    Dark,
    /// Warm sepia tones inspired by aged paper.
    #[default]
    Sepia,
    /// Cool blue-green palette.
    Ocean,
    /// Muted purple-blue evening palette.
    Twilight,
    /// Soft green-grey natural palette.
    Sage,
    /// Cool grey professional palette.
    Slate,
    /// Warm amber-red palette.
    Ember,
}

impl Theme {
    /// Human-readable display name for this theme.
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Sepia => "Sepia",
            Theme::Ocean => "Ocean",
            Theme::Twilight => "Twilight",
            Theme::Sage => "Sage",
            Theme::Slate => "Slate",
            Theme::Ember => "Ember",
        }
    }

    /// All available theme variants.
    pub fn all() -> &'static [Theme] {
        &[
            Theme::Light,
            Theme::Dark,
            Theme::Sepia,
            Theme::Ocean,
            Theme::Twilight,
            Theme::Sage,
            Theme::Slate,
            Theme::Ember,
        ]
    }

    /// Map this theme to an `iced::Theme` for the application builder.
    pub fn to_iced(&self) -> iced::Theme {
        match self {
            Theme::Light => iced::Theme::Light,
            Theme::Dark => iced::Theme::Dark,
            Theme::Sepia => iced::Theme::custom(
                String::from("Sepia"),
                sepia_palette(),
            ),
            Theme::Ocean => iced::Theme::custom(
                String::from("Ocean"),
                sepia_palette(),
            ),
            Theme::Twilight => iced::Theme::custom(
                String::from("Twilight"),
                sepia_palette(),
            ),
            Theme::Sage => iced::Theme::custom(
                String::from("Sage"),
                sepia_palette(),
            ),
            Theme::Slate => iced::Theme::custom(
                String::from("Slate"),
                sepia_palette(),
            ),
            Theme::Ember => iced::Theme::custom(
                String::from("Ember"),
                sepia_palette(),
            ),
        }
    }

    /// Returns the semantic colour set for this theme.
    pub fn colors(&self) -> SemanticColors {
        match self {
            Theme::Light => light(),
            Theme::Dark => dark(),
            Theme::Sepia => sepia(),
            Theme::Ocean => light(),
            Theme::Twilight => light(),
            Theme::Sage => light(),
            Theme::Slate => light(),
            Theme::Ember => light(),
        }
    }
}

/// Returns an Iced palette for the sepia theme.
fn sepia_palette() -> iced::theme::Palette {
    iced::theme::Palette {
        background: Color::from_rgb(0.96, 0.94, 0.91),
        text: Color::from_rgb(0.24, 0.15, 0.14),
        primary: Color::from_rgb(0.55, 0.43, 0.39),
        success: Color::from_rgb(0.55, 0.63, 0.39),
        warning: Color::from_rgb(0.85, 0.65, 0.30),
        danger: Color::from_rgb(0.78, 0.35, 0.35),
    }
}

// ---------------------------------------------------------------------------
// ThemeState
// ---------------------------------------------------------------------------

/// Holds the active theme selection for the application.
#[derive(Debug, Clone)]
pub struct ThemeState {
    /// The currently active theme.
    pub theme: Theme,
}

impl ThemeState {
    /// Create a new theme state with the given theme.
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    /// Map the active theme to an `iced::Theme`.
    pub fn to_iced(&self) -> iced::Theme {
        self.theme.to_iced()
    }

    /// Returns the semantic colour set for the active theme.
    pub fn colors(&self) -> SemanticColors {
        self.theme.colors()
    }
}

impl AsRef<Theme> for ThemeState {
    fn as_ref(&self) -> &Theme {
        &self.theme
    }
}

// ---------------------------------------------------------------------------
// SemanticColors
// ---------------------------------------------------------------------------

/// Complete semantic colour set for a theme.
///
/// All fields are `iced::Color` values. Organised into text, surface,
/// action, and border categories.
#[derive(Debug, Clone, Copy)]
pub struct SemanticColors {
    // Text
    /// Primary text colour (headings, body).
    pub text_primary: Color,
    /// Secondary text colour (labels, captions).
    pub text_secondary: Color,
    /// Tertiary text colour (hints, disabled).
    pub text_tertiary: Color,
    /// Text on primary action backgrounds.
    pub text_on_primary: Color,

    // Surfaces
    /// Primary surface (main background).
    pub surface_primary: Color,
    /// Secondary surface (cards, panels).
    pub surface_secondary: Color,
    /// Tertiary surface (nested sections).
    pub surface_tertiary: Color,
    /// Surface on hover.
    pub surface_hover: Color,
    /// Surface when selected.
    pub surface_selected: Color,

    // Actions
    /// Primary action colour (main buttons).
    pub action_primary: Color,
    /// Primary action hover state.
    pub action_primary_hover: Color,
    /// Primary action active/pressed state.
    pub action_primary_active: Color,
    /// Secondary action colour (secondary buttons).
    pub action_secondary: Color,
    /// Secondary action hover state.
    pub action_secondary_hover: Color,
    /// Danger/destructive action colour.
    pub action_danger: Color,
    /// Danger action hover state.
    pub action_danger_hover: Color,
    /// Success action colour.
    pub action_success: Color,
    /// Success action hover state.
    pub action_success_hover: Color,
    /// Warning colour.
    pub action_warning: Color,

    // Borders
    /// Default border colour.
    pub border_default: Color,
    /// Border on hover.
    pub border_hover: Color,
    /// Border on focus.
    pub border_focus: Color,
    /// Divider line colour.
    pub border_divider: Color,
}

// ---------------------------------------------------------------------------
// Colour sets
// ---------------------------------------------------------------------------

/// Light theme colours.
fn light() -> SemanticColors {
    SemanticColors {
        text_primary: Color::from_rgb(0.13, 0.13, 0.13),
        text_secondary: Color::from_rgb(0.40, 0.40, 0.40),
        text_tertiary: Color::from_rgb(0.60, 0.60, 0.60),
        text_on_primary: Color::WHITE,
        surface_primary: Color::WHITE,
        surface_secondary: Color::from_rgb(0.97, 0.97, 0.97),
        surface_tertiary: Color::from_rgb(0.94, 0.94, 0.94),
        surface_hover: Color::from_rgb(0.95, 0.95, 0.95),
        surface_selected: Color::from_rgb(0.92, 0.92, 0.92),
        action_primary: Color::from_rgb(0.20, 0.40, 0.72),
        action_primary_hover: Color::from_rgb(0.16, 0.34, 0.64),
        action_primary_active: Color::from_rgb(0.13, 0.28, 0.56),
        action_secondary: Color::from_rgb(0.94, 0.94, 0.94),
        action_secondary_hover: Color::from_rgb(0.90, 0.90, 0.90),
        action_danger: Color::from_rgb(0.80, 0.20, 0.20),
        action_danger_hover: Color::from_rgb(0.70, 0.15, 0.15),
        action_success: Color::from_rgb(0.20, 0.65, 0.32),
        action_success_hover: Color::from_rgb(0.15, 0.55, 0.25),
        action_warning: Color::from_rgb(0.90, 0.65, 0.10),
        border_default: Color::from_rgb(0.82, 0.82, 0.82),
        border_hover: Color::from_rgb(0.65, 0.65, 0.65),
        border_focus: Color::from_rgb(0.20, 0.40, 0.72),
        border_divider: Color::from_rgb(0.90, 0.90, 0.90),
    }
}

/// Dark theme colours.
fn dark() -> SemanticColors {
    SemanticColors {
        text_primary: Color::from_rgb(0.93, 0.93, 0.93),
        text_secondary: Color::from_rgb(0.70, 0.70, 0.70),
        text_tertiary: Color::from_rgb(0.50, 0.50, 0.50),
        text_on_primary: Color::WHITE,
        surface_primary: Color::from_rgb(0.12, 0.12, 0.12),
        surface_secondary: Color::from_rgb(0.16, 0.16, 0.16),
        surface_tertiary: Color::from_rgb(0.20, 0.20, 0.20),
        surface_hover: Color::from_rgb(0.22, 0.22, 0.22),
        surface_selected: Color::from_rgb(0.25, 0.25, 0.25),
        action_primary: Color::from_rgb(0.40, 0.60, 0.90),
        action_primary_hover: Color::from_rgb(0.45, 0.65, 0.95),
        action_primary_active: Color::from_rgb(0.35, 0.55, 0.85),
        action_secondary: Color::from_rgb(0.20, 0.20, 0.20),
        action_secondary_hover: Color::from_rgb(0.25, 0.25, 0.25),
        action_danger: Color::from_rgb(0.90, 0.35, 0.35),
        action_danger_hover: Color::from_rgb(0.95, 0.40, 0.40),
        action_success: Color::from_rgb(0.35, 0.75, 0.45),
        action_success_hover: Color::from_rgb(0.40, 0.80, 0.50),
        action_warning: Color::from_rgb(0.95, 0.75, 0.25),
        border_default: Color::from_rgb(0.30, 0.30, 0.30),
        border_hover: Color::from_rgb(0.45, 0.45, 0.45),
        border_focus: Color::from_rgb(0.40, 0.60, 0.90),
        border_divider: Color::from_rgb(0.22, 0.22, 0.22),
    }
}

/// Sepia theme colours (warm, aged paper tones).
fn sepia() -> SemanticColors {
    SemanticColors {
        text_primary: Color::from_rgb(0.24, 0.15, 0.14),
        text_secondary: Color::from_rgb(0.40, 0.32, 0.30),
        text_tertiary: Color::from_rgb(0.56, 0.48, 0.46),
        text_on_primary: Color::from_rgb(1.0, 1.0, 1.0),
        surface_primary: Color::from_rgb(0.96, 0.94, 0.91),
        surface_secondary: Color::from_rgb(0.93, 0.90, 0.87),
        surface_tertiary: Color::from_rgb(0.90, 0.87, 0.83),
        surface_hover: Color::from_rgb(0.91, 0.88, 0.84),
        surface_selected: Color::from_rgb(0.88, 0.85, 0.80),
        action_primary: Color::from_rgb(0.55, 0.43, 0.39),
        action_primary_hover: Color::from_rgb(0.48, 0.36, 0.32),
        action_primary_active: Color::from_rgb(0.42, 0.30, 0.26),
        action_secondary: Color::from_rgb(0.93, 0.90, 0.87),
        action_secondary_hover: Color::from_rgb(0.90, 0.87, 0.83),
        action_danger: Color::from_rgb(0.78, 0.35, 0.35),
        action_danger_hover: Color::from_rgb(0.70, 0.28, 0.28),
        action_success: Color::from_rgb(0.55, 0.63, 0.39),
        action_success_hover: Color::from_rgb(0.48, 0.56, 0.32),
        action_warning: Color::from_rgb(0.85, 0.65, 0.30),
        border_default: Color::from_rgb(0.80, 0.76, 0.72),
        border_hover: Color::from_rgb(0.65, 0.58, 0.54),
        border_focus: Color::from_rgb(0.55, 0.43, 0.39),
        border_divider: Color::from_rgb(0.85, 0.82, 0.78),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_is_sepia() {
        assert_eq!(Theme::default(), Theme::Sepia);
    }

    #[test]
    fn all_returns_eight_variants() {
        assert_eq!(Theme::all().len(), 8);
    }

    #[test]
    fn theme_name_matches_variant() {
        assert_eq!(Theme::Light.name(), "Light");
        assert_eq!(Theme::Dark.name(), "Dark");
        assert_eq!(Theme::Sepia.name(), "Sepia");
        assert_eq!(Theme::Ocean.name(), "Ocean");
        assert_eq!(Theme::Twilight.name(), "Twilight");
        assert_eq!(Theme::Sage.name(), "Sage");
        assert_eq!(Theme::Slate.name(), "Slate");
        assert_eq!(Theme::Ember.name(), "Ember");
    }

    #[test]
    fn theme_state_delegates_to_theme() {
        let state = ThemeState::new(Theme::Dark);
        let colors = state.colors();
        let direct = Theme::Dark.colors();
        assert_eq!(colors.text_primary.r, direct.text_primary.r);
    }

    #[test]
    fn as_ref_returns_inner_theme() {
        let state = ThemeState::new(Theme::Ocean);
        assert_eq!(*state.as_ref(), Theme::Ocean);
    }

    #[test]
    fn sepia_colours_are_warm() {
        let c = sepia();
        // Sepia text should have warm red channel > green > blue
        assert!(c.text_primary.r > c.text_primary.b);
    }

    #[test]
    fn light_surface_primary_is_white() {
        let c = light();
        assert!((c.surface_primary.r - 1.0).abs() < 0.01);
        assert!((c.surface_primary.g - 1.0).abs() < 0.01);
        assert!((c.surface_primary.b - 1.0).abs() < 0.01);
    }

    #[test]
    fn dark_surface_is_dark() {
        let c = dark();
        assert!(c.surface_primary.r < 0.2);
    }

    #[test]
    fn serde_round_trip_theme() {
        let theme = Theme::Twilight;
        let json = serde_json::to_string(&theme).unwrap();
        let back: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(back, theme);
    }
}
