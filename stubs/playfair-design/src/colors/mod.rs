pub mod primitives;
pub mod semantic;

pub use semantic::SemanticColors;

use crate::Theme;
use iced::Color;

pub fn color_to_css_hex(color: Color) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    )
}

impl Theme {
    pub fn colors(&self) -> SemanticColors { semantic::stub() }

    pub fn to_css_tokens(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}
