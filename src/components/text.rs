//! Text - Display text content.

use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};
use gpui_component::ActiveTheme;

use crate::modifier::Modifier;
use crate::style::{Color, Font, FontWeight};

/// How text is truncated when it doesn't fit in its container.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TruncationMode {
    /// Truncate at the head of the line, replacing removed text with an ellipsis.
    Head,
    /// Truncate at the tail of the line, replacing removed text with an ellipsis.
    #[default]
    Tail,
    /// Truncate in the middle of the line, replacing removed text with an ellipsis.
    Middle,
}

/// A view that displays one or more lines of read-only text.
///
/// # Example
///
/// ```rust,ignore
/// Text::new("Hello, World!")
///     .font(Font::title())
///     .foreground_color(Color::primary())
/// ```
#[derive(IntoElement)]
pub struct Text {
    content: SharedString,
    font: Option<Font>,
    color: Option<Color>,
    line_limit: Option<usize>,
    truncation_mode: TruncationMode,
    strikethrough: bool,
}

impl Text {
    /// Create a new text view with the given content.
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            font: None,
            color: None,
            line_limit: None,
            truncation_mode: TruncationMode::default(),
            strikethrough: false,
        }
    }

    /// Set the font style.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Set the text color.
    pub fn foreground_color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Limit the number of lines.
    pub fn line_limit(mut self, limit: usize) -> Self {
        self.line_limit = Some(limit);
        self
    }

    /// Set how text is truncated when it doesn't fit.
    pub fn truncation_mode(mut self, mode: TruncationMode) -> Self {
        self.truncation_mode = mode;
        self
    }

    /// Apply strikethrough styling.
    pub fn strikethrough(mut self, active: bool) -> Self {
        self.strikethrough = active;
        self
    }

    // Convenience font methods

    /// Make the text bold.
    pub fn bold(mut self) -> Self {
        let font = self.font.take().unwrap_or_default();
        self.font = Some(font.weight(FontWeight::Bold));
        self
    }

    /// Make the text italic.
    pub fn italic(mut self) -> Self {
        let font = self.font.take().unwrap_or_default();
        self.font = Some(font.italic());
        self
    }

    /// Set font size.
    pub fn font_size(mut self, size: f32) -> Self {
        let font = self.font.take().unwrap_or_default();
        self.font = Some(Font {
            size: Some(size),
            ..font
        });
        self
    }

    /// Set font weight.
    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        let font = self.font.take().unwrap_or_default();
        self.font = Some(font.weight(weight));
        self
    }
}

impl Modifier for Text {}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().is_dark();
        let mut element = div().child(self.content);

        // Apply font properties
        if let Some(font) = &self.font {
            // Size
            if let Some(size) = font.size {
                element = element.text_size(px(size));
            }
            // Weight
            element = element.font_weight(font.weight.to_gpui());
            // Italic
            if font.italic {
                element = element.italic();
            }
            // Font family from design
            if let Some(family) = font.design.font_family() {
                element = element.font_family(family);
            }
        }

        // Apply color
        if let Some(color) = self.color {
            element = element.text_color(color.resolve(is_dark));
        }

        // Apply line limit with truncation for proper ellipsis in flex layouts
        if let Some(limit) = self.line_limit {
            element = element
                .line_clamp(limit)
                .flex_shrink()
                .min_w_0()
                .overflow_hidden()
                .text_ellipsis();
        }

        // Apply strikethrough
        if self.strikethrough {
            element = element.line_through();
        }

        element
    }
}
