//! Stepper - Increment/decrement control.
//!
//! A SwiftUI-style stepper that wraps gpui-component's NumberInput.
//!
//! # Usage
//!
//! Stepper requires state management via `Entity<InputState>`. Create the state
//! in your view's constructor and pass it to Stepper.
//!
//! ```rust,ignore
//! struct QuantityView {
//!     quantity_input: Entity<InputState>,
//!     quantity: i32,
//! }
//!
//! impl QuantityView {
//!     fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
//!         let quantity_input = cx.new(|cx|
//!             InputState::new(window, cx)
//!                 .default_value("1")
//!                 .placeholder("Qty")
//!         );
//!         Self { quantity_input, quantity: 1 }
//!     }
//! }
//!
//! // In render:
//! Stepper::new(&self.quantity_input)
//! ```

use gpui::{App, Entity, IntoElement, RenderOnce, Window};
use gpui_component::input::NumberInput;
use gpui_component::Disableable;

// Re-use InputState from text_field
use super::text_field::InputState;

// Re-export NumberInput event types
pub use gpui_component::input::NumberInputEvent as StepperEvent;
pub use gpui_component::input::StepAction;

use crate::modifier::Modifier;

/// A control for incrementing and decrementing a value.
///
/// This component wraps gpui-component's NumberInput.
///
/// # Example
///
/// ```rust,ignore
/// // Create state in your view
/// let quantity = cx.new(|cx|
///     InputState::new(window, cx)
///         .default_value("1")
/// );
///
/// // Subscribe to step events
/// cx.subscribe(&quantity, |this, state, event: &NumberInputEvent, cx| {
///     match event {
///         NumberInputEvent::Step(StepAction::Increment) => {
///             this.quantity += 1;
///             state.update(cx, |input, cx| {
///                 input.set_value(this.quantity.to_string(), window, cx);
///             });
///         }
///         NumberInputEvent::Step(StepAction::Decrement) => {
///             this.quantity -= 1;
///             state.update(cx, |input, cx| {
///                 input.set_value(this.quantity.to_string(), window, cx);
///             });
///         }
///     }
/// });
///
/// // Use in render
/// Stepper::new(&quantity)
/// ```
#[derive(IntoElement)]
pub struct Stepper {
    state: Entity<InputState>,
    disabled: bool,
}

impl Stepper {
    /// Create a new stepper with the given state.
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            disabled: false,
        }
    }

    /// Disable the stepper.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Modifier for Stepper {}

impl RenderOnce for Stepper {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut input = NumberInput::new(&self.state);

        if self.disabled {
            input = input.disabled(true);
        }

        input
    }
}
