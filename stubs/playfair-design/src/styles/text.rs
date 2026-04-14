use crate::Theme;
use iced::widget::text;

fn t<'a>(content: impl ToString) -> text::Text<'a> {
    text(content.to_string())
}

pub fn display<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn h1<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn h2<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn h3<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn h4<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn h5<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn h6<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn body_lg<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn body<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn body_sm<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn caption<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn tiny<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn code<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn emphasized<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn muted<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn link<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn error<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn success<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn warning<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
pub fn label<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> { let _ = theme; t(content) }
