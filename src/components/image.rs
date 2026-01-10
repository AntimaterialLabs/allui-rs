//! Image - Display images.

use gpui::{div, px, rgb, App, IntoElement, ParentElement, RenderOnce, Styled, Window};

use crate::modifier::{ContentMode, Modifier};

/// A view that displays an image.
///
/// # Example
///
/// ```rust,ignore
/// Image::new("photo.png")
///     .resizable()
///     .frame_size(100.0, 100.0)
/// ```
#[derive(IntoElement)]
pub struct Image {
    source: ImageSource,
    content_mode: ContentMode,
}

/// The source of an image.
#[derive(Clone)]
pub enum ImageSource {
    /// Image from a file path.
    File(String),
    /// Image from a URL.
    Url(String),
    /// System icon (SF Symbol name).
    System(String),
}

impl Image {
    /// Create an image from a file path.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            source: ImageSource::File(path.into()),
            content_mode: ContentMode::Fit,
        }
    }

    /// Create an image from a URL.
    pub fn url(url: impl Into<String>) -> Self {
        Self {
            source: ImageSource::Url(url.into()),
            content_mode: ContentMode::Fit,
        }
    }

    /// Create a system icon (SF Symbol).
    pub fn system_name(name: impl Into<String>) -> Self {
        Self {
            source: ImageSource::System(name.into()),
            content_mode: ContentMode::Fit,
        }
    }

    /// Set the content mode (fit or fill).
    pub fn content_mode(mut self, mode: ContentMode) -> Self {
        self.content_mode = mode;
        self
    }

    /// Set resizable behavior (fit mode).
    pub fn resizable(mut self) -> Self {
        self.content_mode = ContentMode::Fit;
        self
    }

    /// Scale to fill the frame.
    pub fn scale_to_fill(mut self) -> Self {
        self.content_mode = ContentMode::Fill;
        self
    }

    /// Scale to fit the frame.
    pub fn scale_to_fit(mut self) -> Self {
        self.content_mode = ContentMode::Fit;
        self
    }
}

impl Modifier for Image {}

impl RenderOnce for Image {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // TODO: Implement actual image loading via GPUI's img() element
        // For now, render a placeholder
        match &self.source {
            ImageSource::File(path) => div()
                .flex()
                .items_center()
                .justify_center()
                .size(px(48.0))
                .bg(rgb(0x333333))
                .rounded(px(4.0))
                .text_color(rgb(0x888888))
                .text_size(px(10.0))
                .child(format!("[{}]", path.split('/').next_back().unwrap_or(path))),
            ImageSource::Url(_url) => div()
                .flex()
                .items_center()
                .justify_center()
                .size(px(48.0))
                .bg(rgb(0x333333))
                .rounded(px(4.0))
                .text_color(rgb(0x888888))
                .text_size(px(10.0))
                .child("[URL]"),
            ImageSource::System(name) => {
                // TODO: Use gpui-component's Icon when available
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(24.0))
                    .text_color(rgb(0xffffff))
                    .child(format!("[{}]", name))
            }
        }
    }
}
