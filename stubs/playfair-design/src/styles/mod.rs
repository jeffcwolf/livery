//! Iced widget style functions driven by [`Theme`](crate::Theme).
//!
//! Each submodule provides functions that take a `&Theme`, read its
//! semantic colours, and return a style closure compatible with
//! the corresponding Iced widget.

pub mod btn;
pub mod cont;
pub mod pl;
pub mod prog;
pub mod scroll;
pub mod text_input;
pub mod txt;
