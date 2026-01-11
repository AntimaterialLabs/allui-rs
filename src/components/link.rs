//! Link - Tappable text that triggers an action.

use gpui::{
    App, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div,
};
use gpui_component::ActiveTheme;

use crate::modifier::Modifier;
use crate::style::Color;
use crate::types::ClickHandler;

/// A control for navigating to a URL or triggering an action.
///
/// # Example
///
/// ```rust,ignore
/// Link::new("Visit Website", || {
///     open_url("https://example.com");
/// })
/// ```
#[derive(IntoElement)]
pub struct Link {
    id: SharedString,
    label: SharedString,
    action: Option<ClickHandler>,
    color: Option<Color>,
}

impl Link {
    /// Create a new link with a label and action.
    pub fn new(label: impl Into<SharedString>, action: impl Fn() + 'static) -> Self {
        let label_str: SharedString = label.into();
        Self {
            id: label_str.clone(),
            label: label_str,
            action: Some(Box::new(move |_, _, _| action())),
            color: None,
        }
    }

    /// Set the link color.
    pub fn foreground_color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Modifier for Link {}

impl RenderOnce for Link {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().is_dark();
        let id = gpui::ElementId::Name(self.id.clone());
        let color = self.color.unwrap_or(Color::blue());

        let mut link = div()
            .id(id)
            .cursor_pointer()
            .text_color(color.resolve(is_dark))
            .child(self.label);

        if let Some(action) = self.action {
            link = link.on_click(move |event, window, cx| {
                action(event, window, cx);
            });
        }

        link
    }
}
