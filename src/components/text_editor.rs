//! TextEditor - Multi-line text input component.
//!
//! A SwiftUI-style multi-line text editor that wraps gpui-component's Input
//! with multi_line enabled.
//!
//! # Usage
//!
//! ```rust,ignore
//! struct NotesView {
//!     notes_input: Entity<InputState>,
//! }
//!
//! impl NotesView {
//!     fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
//!         let notes_input = cx.new(|cx|
//!             InputState::new(window, cx)
//!                 .multi_line(true)  // Enable multi-line
//!                 .placeholder("Enter notes...")
//!         );
//!         Self { notes_input }
//!     }
//! }
//!
//! // In render:
//! TextEditor::new(&self.notes_input)
//! ```

use gpui::{App, Entity, IntoElement, Pixels, RenderOnce, Window, px};
use gpui_component::input::Input;

// Use InputState from text_field module
use super::text_field::InputState;

use crate::modifier::Modifier;

/// A multi-line text editor.
///
/// This component wraps gpui-component's Input with multi_line mode.
/// The InputState should be created with `.multi_line(true)`.
///
/// # Example
///
/// ```rust,ignore
/// let notes = cx.new(|cx|
///     InputState::new(window, cx)
///         .multi_line(true)
///         .placeholder("Write something...")
///         .rows(10)  // Optional: set number of visible rows
/// );
///
/// TextEditor::new(&notes)
///     .height(200.0)
/// ```
#[derive(IntoElement)]
pub struct TextEditor {
    state: Entity<InputState>,
    height: Option<Pixels>,
    disabled: bool,
}

impl TextEditor {
    /// Create a new text editor with the given state.
    ///
    /// Note: The InputState should be created with `.multi_line(true)`.
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            height: None,
            disabled: false,
        }
    }

    /// Set the height of the editor.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(px(height));
        self
    }

    /// Disable the text editor.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Modifier for TextEditor {}

impl RenderOnce for TextEditor {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut input = Input::new(&self.state);

        if let Some(height) = self.height {
            input = input.h(height);
        }

        if self.disabled {
            input = input.disabled(true);
        }

        input
    }
}
