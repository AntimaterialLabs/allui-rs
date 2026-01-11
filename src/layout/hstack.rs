//! HStack - Horizontal stack layout.

use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

use crate::alignment::VerticalAlignment;
use crate::modifier::Modifier;

/// A view that arranges its children in a horizontal line.
///
/// By default, HStack centers its children vertically (matching SwiftUI).
///
/// # Example
///
/// ```rust,ignore
/// HStack::new()
///     .spacing(8.0)
///     .child(Image::system_name("star"))
///     .child(Text::new("Favorites"))
///     .child(Spacer::new())
///     .child(Text::new("12"))
/// ```
#[derive(IntoElement)]
pub struct HStack {
    spacing: f32,
    alignment: VerticalAlignment,
    children: Vec<gpui::AnyElement>,
}

impl HStack {
    /// Create a new horizontal stack.
    pub fn new() -> Self {
        Self {
            spacing: 8.0,
            alignment: VerticalAlignment::Center, // SwiftUI default
            children: Vec::new(),
        }
    }

    /// Set the spacing between children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the vertical alignment of children.
    pub fn alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    impl_child_methods!();
}

impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for HStack {}

impl RenderOnce for HStack {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // w_full() matches SwiftUI behavior when Spacer is used - stacks become "filling" views.
        // overflow_hidden() establishes containing block for child truncation to work.
        let container = div()
            .flex()
            .flex_row()
            .w_full()
            .overflow_hidden()
            .gap(px(self.spacing));
        self.alignment
            .apply_as_items(container)
            .children(self.children)
    }
}
