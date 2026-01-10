//! ZStack - Overlay stack layout.

use gpui::{div, App, IntoElement, ParentElement, RenderOnce, Styled, Window};

use crate::alignment::Alignment;
use crate::modifier::Modifier;

/// A view that overlays its children, aligning them in both axes.
///
/// By default, ZStack centers its children (matching SwiftUI).
///
/// # Example
///
/// ```rust,ignore
/// ZStack::new()
///     .alignment(Alignment::bottom_trailing())
///     .child(Image::new("background"))
///     .child(Text::new("Badge"))
/// ```
#[derive(IntoElement)]
pub struct ZStack {
    alignment: Alignment,
    children: Vec<gpui::AnyElement>,
}

impl ZStack {
    /// Create a new overlay stack.
    pub fn new() -> Self {
        Self {
            alignment: Alignment::center(), // SwiftUI default
            children: Vec::new(),
        }
    }

    /// Set the alignment of children within the stack.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    impl_child_methods!();
}

impl Default for ZStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for ZStack {}

impl RenderOnce for ZStack {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Strategy: Use relative container + absolute positioned children
        // - Container is relative, sizes to its largest child via a "sizer" approach
        // - First child determines size (rendered normally)
        // - All children rendered absolutely on top with alignment
        //
        // For proper "size to largest", we render the first child twice:
        // once for sizing (invisible), and all children absolutely for display.
        //
        // Actually simpler: just use absolute positioning with inset-0 and flex
        // to align within. The container needs an explicit size or to get size
        // from somewhere.
        //
        // The REAL issue: When ZStack is wrapped in frame_size(200, 200), the
        // Frame modifier creates a container. The ZStack's container with
        // size_full() should fill that. Then absolute children with inset_0
        // fill the ZStack container, and flex aligns within.

        let alignment = self.alignment;

        let positioned_children: Vec<_> = self
            .children
            .into_iter()
            .map(|child| {
                // Absolute positioning fills the container via inset_0
                // Flex + alignment positions the child within
                let wrapper = div().absolute().inset_0().flex();
                let wrapper = alignment.horizontal.apply_as_justify(wrapper);
                let wrapper = alignment.vertical.apply_as_items(wrapper);

                wrapper.child(child)
            })
            .collect();

        // Container needs relative for absolute children to position against
        // size_full so it fills any frame wrapper
        div().relative().size_full().children(positioned_children)
    }
}
