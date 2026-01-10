//! VStack - Vertical stack layout.

use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, Styled, Window};

use crate::alignment::HorizontalAlignment;
use crate::modifier::Modifier;

/// A view that arranges its children in a vertical line.
///
/// By default, VStack centers its children horizontally (matching SwiftUI).
///
/// # Example
///
/// ```rust,ignore
/// VStack::new()
///     .spacing(12.0)
///     .alignment(HorizontalAlignment::Leading)
///     .child(Text::new("Title"))
///     .child(Text::new("Subtitle"))
/// ```
#[derive(IntoElement)]
pub struct VStack {
    spacing: f32,
    alignment: HorizontalAlignment,
    children: Vec<gpui::AnyElement>,
}

impl VStack {
    /// Create a new vertical stack.
    pub fn new() -> Self {
        Self {
            spacing: 8.0,
            alignment: HorizontalAlignment::Center, // SwiftUI default
            children: Vec::new(),
        }
    }

    /// Set the spacing between children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the horizontal alignment of children.
    pub fn alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    impl_child_methods!();
}

impl Default for VStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for VStack {}

impl RenderOnce for VStack {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let container = div().flex().flex_col().gap(px(self.spacing));
        self.alignment
            .apply_as_items(container)
            .children(self.children)
    }
}
