//! Playfair Design System — API stub for cross-repo compilation.

pub mod borders;
pub mod colors;
pub mod elevation;
pub mod icons;
pub mod spacing;
pub mod styles;
pub mod theme;
pub mod typography;
pub mod widgets;
pub mod prelude;

/// Raw bytes of the Lucide icon font (empty in stub).
pub const LUCIDE_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Lucide.ttf");

/// Font descriptor for the Lucide icon font.
pub const LUCIDE_FONT: iced::Font = iced::Font::with_name("Lucide");

pub use colors::color_to_css_hex;
pub use colors::SemanticColors;
pub use iced::Theme as IcedTheme;
pub use iced::{Color, Font};
pub use theme::Theme;
pub use theme::ThemeState;
pub use typography::{default_font, monospace_font};
