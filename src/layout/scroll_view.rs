//! ScrollView - Scrollable container.

use gpui::{
    div, App, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window,
};

use crate::modifier::Modifier;

/// Specifies which axes are scrollable.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollAxes {
    /// Scroll vertically only (default, like SwiftUI)
    #[default]
    Vertical,
    /// Scroll horizontally only
    Horizontal,
    /// Scroll in both directions
    Both,
}

impl ScrollAxes {
    /// Create axes that scroll vertically.
    pub fn vertical() -> Self {
        Self::Vertical
    }

    /// Create axes that scroll horizontally.
    pub fn horizontal() -> Self {
        Self::Horizontal
    }

    /// Create axes that scroll in both directions.
    pub fn both() -> Self {
        Self::Both
    }
}

/// A view that allows its content to be scrolled.
///
/// By default, ScrollView scrolls vertically (matching SwiftUI).
///
/// # Example
///
/// ```rust,ignore
/// ScrollView::new("my-scroll")
///     .axes(ScrollAxes::vertical())
///     .child(
///         VStack::new()
///             .children(items.iter().map(|item| ItemRow::new(item)))
///     )
/// ```
#[derive(IntoElement)]
pub struct ScrollView {
    id: SharedString,
    axes: ScrollAxes,
    shows_indicators: bool,
    children: Vec<gpui::AnyElement>,
}

impl ScrollView {
    /// Create a new scrollable container with the given ID.
    ///
    /// The ID is required for GPUI's stateful element tracking.
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            axes: ScrollAxes::Vertical,
            shows_indicators: true,
            children: Vec::new(),
        }
    }

    /// Set the scrollable axes.
    pub fn axes(mut self, axes: ScrollAxes) -> Self {
        self.axes = axes;
        self
    }

    /// Set whether to show scroll indicators.
    ///
    /// Note: GPUI may not support hiding scroll indicators in all cases.
    pub fn shows_indicators(mut self, show: bool) -> Self {
        self.shows_indicators = show;
        self
    }

    impl_child_methods!();
}

impl Modifier for ScrollView {}

impl RenderOnce for ScrollView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Create a stateful div for scrolling
        let mut container = div().id(self.id).size_full().flex();

        // Apply flex direction and scroll behavior based on axes
        container = match self.axes {
            ScrollAxes::Vertical => container.flex_col().overflow_y_scroll(),
            ScrollAxes::Horizontal => container.flex_row().overflow_x_scroll(),
            ScrollAxes::Both => container.flex_col().overflow_scroll(),
        };

        // GPUI doesn't have direct control over scroll indicator visibility
        // but we store the setting for future use
        if !self.shows_indicators {
            // Future: container = container.scrollbar_width(px(0.))
        }

        // Add children
        container.children(self.children)
    }
}
