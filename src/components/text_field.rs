//! TextField - Single-line text input component.
//!
//! A SwiftUI-style text field that wraps gpui-component's Input.
//!
//! # Usage
//!
//! TextField requires state management via `Entity<InputState>`. Create the state
//! in your view's constructor and pass it to TextField.
//!
//! ```rust,ignore
//! struct MyView {
//!     email_input: Entity<InputState>,
//! }
//!
//! impl MyView {
//!     fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
//!         let email_input = cx.new(|cx|
//!             InputState::new(window, cx)
//!                 .placeholder("Email")
//!         );
//!         Self { email_input }
//!     }
//! }
//!
//! // In render:
//! TextField::new(&self.email_input)
//! ```

use gpui::{App, Entity, IntoElement, RenderOnce, Window};
use gpui_component::input::Input;

// Re-export InputState for users
pub use gpui_component::input::InputState;

use crate::modifier::Modifier;

/// A single-line text input field.
///
/// This component wraps gpui-component's Input, providing a SwiftUI-like API.
/// State is managed via `Entity<InputState>` which must be created in your view.
///
/// # Example
///
/// ```rust,ignore
/// // Create state in your view
/// let name_input = cx.new(|cx|
///     InputState::new(window, cx)
///         .placeholder("Enter name")
///         .default_value("John")
/// );
///
/// // Use in render
/// TextField::new(&name_input)
///     .cleanable(true)
/// ```
#[derive(IntoElement)]
pub struct TextField {
    state: Entity<InputState>,
    cleanable: bool,
    disabled: bool,
}

impl TextField {
    /// Create a new text field with the given state.
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            cleanable: false,
            disabled: false,
        }
    }

    /// Show a clear button when the field has content.
    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.cleanable = cleanable;
        self
    }

    /// Disable the text field.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Modifier for TextField {}

impl RenderOnce for TextField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut input = Input::new(&self.state).cleanable(self.cleanable);

        if self.disabled {
            input = input.disabled(true);
        }

        input
    }
}
