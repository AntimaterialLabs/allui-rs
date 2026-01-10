//! Spacer - Flexible space component.

use gpui::{div, px, App, IntoElement, RenderOnce, Styled, Window};

use crate::modifier::Modifier;

/// The axis along which a Spacer expands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpacerAxis {
    /// Expand along the horizontal axis (for use in HStack).
    Horizontal,
    /// Expand along the vertical axis (for use in VStack).
    Vertical,
    /// Expand along both axes (legacy behavior).
    /// This is the default for backwards compatibility.
    #[default]
    Both,
}

/// A flexible space that expands along the major axis of its containing stack.
///
/// In an HStack, Spacer expands horizontally.
/// In a VStack, Spacer expands vertically.
///
/// # Example
///
/// ```rust,ignore
/// HStack::new()
///     .child(Text::new("Left"))
///     .child(Spacer::new())
///     .child(Text::new("Right"))
///
/// // With minimum length (axis-specific)
/// HStack::new()
///     .child(Text::new("Left"))
///     .child(Spacer::horizontal().min_length(50.0))
///     .child(Text::new("Right"))
/// ```
#[derive(IntoElement)]
pub struct Spacer {
    min_length: Option<f32>,
    axis: SpacerAxis,
}

impl Spacer {
    /// Create a new spacer that expands to fill available space.
    ///
    /// By default, the spacer expands along both axes. Use `horizontal()` or
    /// `vertical()` for axis-specific behavior when using `min_length()`.
    pub fn new() -> Self {
        Self {
            min_length: None,
            axis: SpacerAxis::Both,
        }
    }

    /// Create a horizontal spacer (for use in HStack).
    ///
    /// When `min_length()` is set, it only applies to the width.
    pub fn horizontal() -> Self {
        Self {
            min_length: None,
            axis: SpacerAxis::Horizontal,
        }
    }

    /// Create a vertical spacer (for use in VStack).
    ///
    /// When `min_length()` is set, it only applies to the height.
    pub fn vertical() -> Self {
        Self {
            min_length: None,
            axis: SpacerAxis::Vertical,
        }
    }

    /// Set a minimum length for the spacer.
    ///
    /// The axis this applies to depends on how the Spacer was created:
    /// - `Spacer::new()` - applies to both width and height
    /// - `Spacer::horizontal()` - applies to width only
    /// - `Spacer::vertical()` - applies to height only
    pub fn min_length(mut self, length: f32) -> Self {
        self.min_length = Some(length);
        self
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for Spacer {}

impl RenderOnce for Spacer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut spacer = div().flex_grow();

        // Apply minimum size if specified, based on axis
        if let Some(min) = self.min_length {
            spacer = match self.axis {
                SpacerAxis::Horizontal => spacer.min_w(px(min)),
                SpacerAxis::Vertical => spacer.min_h(px(min)),
                SpacerAxis::Both => spacer.min_w(px(min)).min_h(px(min)),
            };
        }

        spacer
    }
}
