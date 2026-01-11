//! Slider - Range value selection component.
//!
//! A SwiftUI-style slider that wraps gpui-component's Slider.
//!
//! # Usage
//!
//! Slider requires state management via `Entity<SliderState>`. Create the state
//! in your view's constructor and pass it to Slider.
//!
//! ```rust,ignore
//! struct VolumeControl {
//!     volume_slider: Entity<SliderState>,
//! }
//!
//! impl VolumeControl {
//!     fn new(cx: &mut Context<Self>) -> Self {
//!         let volume_slider = cx.new(|_|
//!             SliderState::new()
//!                 .min(0.0)
//!                 .max(100.0)
//!                 .default_value(50.0)
//!                 .step(1.0)
//!         );
//!         Self { volume_slider }
//!     }
//! }
//!
//! // In render:
//! Slider::new(&self.volume_slider)
//! ```

use gpui::{App, Entity, IntoElement, Pixels, RenderOnce, Styled, Window, px};
use gpui_component::slider::Slider as GpuiSlider;

// Re-export SliderState for users
pub use gpui_component::slider::SliderEvent;
pub use gpui_component::slider::SliderState;
pub use gpui_component::slider::SliderValue;

use crate::modifier::Modifier;

/// A control for selecting a value from a bounded range.
///
/// This component wraps gpui-component's Slider.
///
/// # Example
///
/// ```rust,ignore
/// // Create state in your view
/// let brightness = cx.new(|_|
///     SliderState::new()
///         .min(0.0)
///         .max(100.0)
///         .step(1.0)
///         .default_value(75.0)
/// );
///
/// // Subscribe to changes
/// cx.subscribe(&brightness, |this, _, event: &SliderEvent, cx| {
///     if let SliderEvent::Change(value) = event {
///         this.on_brightness_changed(value.start());
///     }
/// });
///
/// // Use in render
/// Slider::new(&brightness)
/// ```
#[derive(IntoElement)]
pub struct Slider {
    state: Entity<SliderState>,
    vertical: bool,
    height: Option<Pixels>,
    disabled: bool,
}

impl Slider {
    /// Create a new slider with the given state.
    pub fn new(state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            vertical: false,
            height: None,
            disabled: false,
        }
    }

    /// Make the slider vertical.
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// Set the height (useful for vertical sliders).
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(px(height));
        self
    }

    /// Disable the slider.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Modifier for Slider {}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut slider = GpuiSlider::new(&self.state);

        if self.vertical {
            slider = slider.vertical();
        }

        if let Some(height) = self.height {
            slider = slider.h(height);
        }

        if self.disabled {
            slider = slider.disabled(true);
        }

        slider
    }
}
