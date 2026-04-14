use iced::Color;

#[derive(Debug, Clone, Copy)]
pub struct SemanticColors {
    pub surface_primary: Color,
    pub surface_secondary: Color,
    pub surface_tertiary: Color,
    pub surface_hover: Color,
    pub surface_selected: Color,
    pub surface_disabled: Color,

    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_tertiary: Color,
    pub text_disabled: Color,
    pub text_on_primary: Color,

    pub border_default: Color,
    pub border_hover: Color,
    pub border_focus: Color,
    pub border_divider: Color,

    pub action_primary: Color,
    pub action_primary_hover: Color,
    pub action_primary_active: Color,
    pub action_secondary: Color,
    pub action_secondary_hover: Color,
    pub action_danger: Color,
    pub action_danger_hover: Color,
    pub action_success: Color,
    pub action_warning: Color,

    pub state_info: Color,
    pub state_success: Color,
    pub state_warning: Color,
    pub state_error: Color,
}

const C: Color = Color::from_rgb(0.5, 0.5, 0.5);

/// Stub: returns placeholder colors for all fields.
pub fn stub() -> SemanticColors {
    SemanticColors {
        surface_primary: C, surface_secondary: C, surface_tertiary: C,
        surface_hover: C, surface_selected: C, surface_disabled: C,
        text_primary: C, text_secondary: C, text_tertiary: C,
        text_disabled: C, text_on_primary: C,
        border_default: C, border_hover: C, border_focus: C, border_divider: C,
        action_primary: C, action_primary_hover: C, action_primary_active: C,
        action_secondary: C, action_secondary_hover: C,
        action_danger: C, action_danger_hover: C,
        action_success: C, action_warning: C,
        state_info: C, state_success: C, state_warning: C, state_error: C,
    }
}

// The real crate exposes per-theme functions; provide them for API compat.
pub fn light() -> SemanticColors { stub() }
pub fn dark() -> SemanticColors { stub() }
pub fn sepia() -> SemanticColors { stub() }
pub fn ocean() -> SemanticColors { stub() }
pub fn twilight() -> SemanticColors { stub() }
pub fn sage() -> SemanticColors { stub() }
pub fn slate() -> SemanticColors { stub() }
pub fn ember() -> SemanticColors { stub() }
