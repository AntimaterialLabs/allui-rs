//! EmptyView - A view that displays nothing.

use gpui::{div, App, IntoElement, RenderOnce, Window};

use crate::modifier::Modifier;

/// A view that displays nothing and takes up no space.
///
/// Useful as a placeholder or for conditional rendering.
///
/// # Example
///
/// ```rust,ignore
/// if show_content {
///     Text::new("Content")
/// } else {
///     EmptyView::new()
/// }
/// ```
#[derive(IntoElement)]
pub struct EmptyView;

impl EmptyView {
    /// Create an empty view.
    pub fn new() -> Self {
        Self
    }
}

impl Default for EmptyView {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for EmptyView {}

impl RenderOnce for EmptyView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Return an empty div that takes no space
        div()
    }
}
