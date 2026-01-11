//! ProgressView - Progress indicator.

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px, relative,
    rgb,
};
use gpui_component::{ActiveTheme, spinner::Spinner};

use crate::modifier::Modifier;
use crate::style::Color;

/// The style of a progress view.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressViewStyle {
    /// Circular spinner.
    #[default]
    Circular,
    /// Linear progress bar.
    Linear,
}

/// A view that shows the progress toward completion of a task.
///
/// # Example
///
/// ```rust,ignore
/// // Indeterminate
/// ProgressView::new()
///
/// // Determinate
/// ProgressView::new()
///     .value(0.65)
///     .label("Downloading...")
/// ```
#[derive(IntoElement)]
pub struct ProgressView {
    value: Option<f32>,
    label: Option<SharedString>,
    style: ProgressViewStyle,
    tint: Option<Color>,
}

impl ProgressView {
    /// Create an indeterminate progress view.
    pub fn new() -> Self {
        Self {
            value: None,
            label: None,
            style: ProgressViewStyle::default(),
            tint: None,
        }
    }

    /// Set the progress value (0.0 to 1.0).
    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value.clamp(0.0, 1.0));
        self
    }

    /// Set a label to display.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the progress view style.
    pub fn progress_view_style(mut self, style: ProgressViewStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the tint color.
    pub fn tint(mut self, color: impl Into<Color>) -> Self {
        self.tint = Some(color.into());
        self
    }
}

impl Default for ProgressView {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for ProgressView {}

impl RenderOnce for ProgressView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().is_dark();
        let tint = self
            .tint
            .map(|c| c.resolve(is_dark))
            .unwrap_or_else(|| rgb(0x007AFF).into());

        match self.style {
            ProgressViewStyle::Circular => {
                let mut container = div().flex().flex_col().items_center().gap(px(8.0));

                // Use gpui-component's animated Spinner
                let spinner = Spinner::new().color(tint);
                container = container.child(spinner);

                if let Some(label) = self.label {
                    container = container.child(label);
                }

                container
            }
            ProgressViewStyle::Linear => {
                let progress = self.value.unwrap_or(0.0);

                let mut container = div().flex().flex_col().gap(px(4.0)).w_full();

                // Progress bar track
                let track = div()
                    .w_full()
                    .h(px(4.0))
                    .rounded_full()
                    .bg(rgb(0x333333))
                    .child(div().h_full().rounded_full().bg(tint).w(relative(progress)));

                container = container.child(track);

                if let Some(label) = self.label {
                    container = container.child(label);
                }

                container
            }
        }
    }
}
