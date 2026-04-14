//! Progress bar style functions.

use iced::widget::progress_bar;

use crate::Theme;
use crate::borders;

/// Standard progress bar style.
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    let c = theme.colors();
    move |_theme: &iced::Theme| progress_bar::Style {
        background: iced::Background::Color(c.surface_tertiary),
        bar: iced::Background::Color(c.action_primary),
        border: iced::Border {
            radius: borders::radius::SM.into(),
            ..Default::default()
        },
    }
}
