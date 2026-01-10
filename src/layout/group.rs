//! Group - Transparent grouping container.

use gpui::{div, App, IntoElement, ParentElement, RenderOnce, Window};

use crate::modifier::Modifier;

/// A transparent container that groups views without affecting layout.
///
/// Group allows you to apply modifiers to multiple views at once
/// without introducing a new layout container.
///
/// # Example
///
/// ```rust,ignore
/// Group::new()
///     .child(Text::new("One"))
///     .child(Text::new("Two"))
///     .child(Text::new("Three"))
///     .foreground_color(Color::red())
/// ```
#[derive(IntoElement)]
pub struct Group {
    children: Vec<gpui::AnyElement>,
}

impl Group {
    /// Create a new group.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    impl_child_methods!();
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for Group {}

impl RenderOnce for Group {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Group renders as a transparent container
        // Use display: contents semantics - children laid out as if group doesn't exist
        // In GPUI/flexbox we approximate with a simple div
        div().children(self.children)
    }
}
