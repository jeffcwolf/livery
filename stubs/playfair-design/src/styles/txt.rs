//! Text style functions.
//!
//! Each function takes a `&Theme` and a string value, returning a
//! styled `Text` widget with appropriate size and colour.

use iced::widget::Text;

use crate::Theme;

/// Heading level 1 (38px, primary text colour).
pub fn h1<'a>(theme: &Theme, content: impl ToString) -> Text<'a> {
    let c = theme.colors();
    iced::widget::text(content.to_string())
        .size(38)
        .color(c.text_primary)
}

/// Heading level 2 (30px, primary text colour).
pub fn h2<'a>(theme: &Theme, content: impl ToString) -> Text<'a> {
    let c = theme.colors();
    iced::widget::text(content.to_string())
        .size(30)
        .color(c.text_primary)
}

/// Heading level 3 (24px, primary text colour).
pub fn h3<'a>(theme: &Theme, content: impl ToString) -> Text<'a> {
    let c = theme.colors();
    iced::widget::text(content.to_string())
        .size(24)
        .color(c.text_primary)
}

/// Heading level 4 (20px, primary text colour).
pub fn h4<'a>(theme: &Theme, content: impl ToString) -> Text<'a> {
    let c = theme.colors();
    iced::widget::text(content.to_string())
        .size(20)
        .color(c.text_primary)
}

/// Body text (16px, primary text colour).
pub fn body<'a>(theme: &Theme, content: impl ToString) -> Text<'a> {
    let c = theme.colors();
    iced::widget::text(content.to_string())
        .size(16)
        .color(c.text_primary)
}

/// Caption text (13px, secondary text colour).
pub fn caption<'a>(theme: &Theme, content: impl ToString) -> Text<'a> {
    let c = theme.colors();
    iced::widget::text(content.to_string())
        .size(13)
        .color(c.text_secondary)
}

/// Label text (14px, secondary text colour).
pub fn label<'a>(theme: &Theme, content: impl ToString) -> Text<'a> {
    let c = theme.colors();
    iced::widget::text(content.to_string())
        .size(14)
        .color(c.text_secondary)
}
