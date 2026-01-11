//! Divider - Visual separator line.

use gpui::{App, IntoElement, RenderOnce, Styled, Window, div, px, rgb};
use gpui_component::ActiveTheme;

use crate::modifier::Modifier;
use crate::style::Color;

/// A visual element that can be used to separate content.
///
/// # Example
///
/// ```rust,ignore
/// VStack::new()
///     .child(Text::new("Section 1"))
///     .child(Divider::new())
///     .child(Text::new("Section 2"))
/// ```
#[derive(IntoElement)]
pub struct Divider {
    color: Option<Color>,
}

impl Divider {
    /// Create a new divider.
    pub fn new() -> Self {
        Self { color: None }
    }

    /// Set the divider color.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for Divider {}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().is_dark();
        let color = self
            .color
            .map(|c| c.resolve(is_dark))
            .unwrap_or_else(|| rgb(0x3c3c3c).into());

        div().w_full().h(px(1.0)).bg(color)
    }
}
