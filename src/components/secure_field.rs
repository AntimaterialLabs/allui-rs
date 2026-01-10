//! SecureField - Password input component.
//!
//! A SwiftUI-style secure text field (password input) that wraps gpui-component's Input
//! with masking enabled.
//!
//! # Usage
//!
//! ```rust,ignore
//! struct LoginView {
//!     password_input: Entity<InputState>,
//! }
//!
//! impl LoginView {
//!     fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
//!         let password_input = cx.new(|cx|
//!             InputState::new(window, cx)
//!                 .placeholder("Password")
//!                 .masked(true)  // Important: enable masking
//!         );
//!         Self { password_input }
//!     }
//! }
//!
//! // In render:
//! SecureField::new(&self.password_input)
//!     .show_toggle(true)  // Optional: show reveal button
//! ```

use gpui::{App, Entity, IntoElement, RenderOnce, Window};
use gpui_component::input::Input;

// Use InputState from text_field module
use super::text_field::InputState;

use crate::modifier::Modifier;

/// A secure text input field for passwords.
///
/// This component wraps gpui-component's Input with masking enabled.
/// The InputState should be created with `.masked(true)`.
///
/// # Example
///
/// ```rust,ignore
/// let password = cx.new(|cx|
///     InputState::new(window, cx)
///         .placeholder("Password")
///         .masked(true)
/// );
///
/// SecureField::new(&password)
///     .show_toggle(true)  // Show reveal/hide button
/// ```
#[derive(IntoElement)]
pub struct SecureField {
    state: Entity<InputState>,
    show_toggle: bool,
    disabled: bool,
}

impl SecureField {
    /// Create a new secure field with the given state.
    ///
    /// Note: The InputState should be created with `.masked(true)` for proper masking.
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            show_toggle: false,
            disabled: false,
        }
    }

    /// Show a toggle button to reveal/hide the password.
    pub fn show_toggle(mut self, show: bool) -> Self {
        self.show_toggle = show;
        self
    }

    /// Disable the secure field.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Modifier for SecureField {}

impl RenderOnce for SecureField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut input = Input::new(&self.state);

        if self.show_toggle {
            input = input.mask_toggle();
        }

        if self.disabled {
            input = input.disabled(true);
        }

        input
    }
}
