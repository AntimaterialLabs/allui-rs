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

use gpui::{
    px, App, Entity, Hsla, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, Window,
};
use gpui_component::h_flex;
use gpui_component::input::Input;
use gpui_component::{ActiveTheme, Icon, IconName};

pub use gpui_component::input::InputState;

use crate::modifier::Modifier;
use crate::style::Color;

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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().is_dark();
        let bg_color = Color::text_field_background().resolve(is_dark);
        let border_color = Color::text_field_border().resolve(is_dark);

        let mut input = Input::new(&self.state)
            .bg(bg_color)
            .border_color(border_color);

        if self.cleanable {
            let has_text = self.state.read(cx).text().len() > 0;
            if has_text {
                let state = self.state.clone();
                let icon_color = Hsla {
                    a: 0.5,
                    ..cx.theme().foreground
                };
                let hover_bg = Hsla {
                    a: 0.1,
                    ..cx.theme().foreground
                };
                let clear_button = h_flex()
                    .id("clear-input")
                    .items_center()
                    .justify_center()
                    .p(px(2.0))
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(hover_bg))
                    .child(Icon::new(IconName::Close).size_4().text_color(icon_color))
                    .on_click(move |_, window, cx| {
                        state.update(cx, |state, cx| {
                            state.set_value("", window, cx);
                        });
                    });
                input = input.suffix(clear_button);
            }
        }

        if self.disabled {
            input = input.disabled(true);
        }

        input
    }
}
