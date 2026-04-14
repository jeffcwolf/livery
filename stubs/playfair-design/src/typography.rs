pub mod scale;
pub mod weights;

use iced::Font;

pub use scale::{line_height, size};
pub use weights::{BOLD, LIGHT, MEDIUM, REGULAR, SEMIBOLD};

pub mod sizes {
    pub const XS: f32 = 10.0;
    pub const SM: f32 = 12.0;
    pub const BASE: f32 = 14.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 18.0;
    pub const XL: f32 = 20.0;
    pub const H1: f32 = 32.0;
    pub const H2: f32 = 28.0;
    pub const H3: f32 = 24.0;
    pub const H4: f32 = 20.0;
    pub const H5: f32 = 18.0;
    pub const H6: f32 = 16.0;
}

pub mod fonts {
    use super::Font;
    pub const DEFAULT: Font = Font::DEFAULT;
    pub const MONOSPACE: Font = Font::MONOSPACE;
    pub fn bold() -> Font { Font::with_name("Helvetica Neue Bold") }
    pub fn italic() -> Font { Font::with_name("Helvetica Neue Italic") }
}

pub fn default_font() -> Font { Font::DEFAULT }
pub fn monospace_font() -> Font { Font::MONOSPACE }
