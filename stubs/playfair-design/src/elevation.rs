use iced::{Color, Shadow, Vector};

pub mod shadow {
    use super::*;
    pub fn none() -> Shadow { Shadow { color: Color::TRANSPARENT, offset: Vector::ZERO, blur_radius: 0.0 } }
    pub fn sm() -> Shadow { none() }
    pub fn md() -> Shadow { none() }
    pub fn lg() -> Shadow { none() }
    pub fn xl() -> Shadow { none() }
}
