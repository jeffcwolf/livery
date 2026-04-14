use iced::theme::Palette;
use iced::Color;
use iced::Theme as IcedTheme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    Sepia,
    Ocean,
    Twilight,
    Sage,
    Slate,
    Ember,
}

impl Theme {
    pub fn to_iced(&self) -> IcedTheme {
        IcedTheme::custom("Stub".to_string(), self.palette())
    }

    pub fn palette(&self) -> Palette {
        Palette::LIGHT
    }

    pub fn all() -> &'static [Theme] {
        &[
            Theme::Light, Theme::Dark, Theme::Sepia, Theme::Ocean,
            Theme::Twilight, Theme::Sage, Theme::Slate, Theme::Ember,
        ]
    }

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

    pub fn default_font(&self) -> iced::Font { iced::Font::DEFAULT }
    pub fn monospace_font(&self) -> iced::Font { iced::Font::MONOSPACE }
}

pub struct ThemeState {
    pub theme: Theme,
}

impl ThemeState {
    pub fn new(theme: Theme) -> Self { Self { theme } }
    pub fn to_iced(&self) -> IcedTheme { self.theme.to_iced() }
    pub fn colors(&self) -> crate::SemanticColors { self.theme.colors() }
    pub fn to_css_tokens(&self) -> std::collections::HashMap<String, String> {
        self.theme.to_css_tokens()
    }
}

impl Default for ThemeState {
    fn default() -> Self { Self::new(Theme::default()) }
}

impl AsRef<Theme> for ThemeState {
    fn as_ref(&self) -> &Theme { &self.theme }
}
