//! List - Styled list container with sections.
//!
//! List provides a scrollable container with SwiftUI-style list appearance,
//! including support for sections with headers and footers.

use gpui::{
    div, px, App, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window,
};

use crate::modifier::Modifier;
use crate::style::Color;

/// List style variants matching SwiftUI's ListStyle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListStyle {
    /// Default list style with platform-appropriate appearance.
    #[default]
    Automatic,
    /// Plain list without section headers floating.
    Plain,
    /// Grouped sections with inset rounded corners.
    InsetGrouped,
    /// Grouped sections.
    Grouped,
    /// Sidebar style for navigation lists.
    Sidebar,
}

impl ListStyle {
    pub fn automatic() -> Self {
        Self::Automatic
    }

    pub fn plain() -> Self {
        Self::Plain
    }

    pub fn inset_grouped() -> Self {
        Self::InsetGrouped
    }

    pub fn grouped() -> Self {
        Self::Grouped
    }

    pub fn sidebar() -> Self {
        Self::Sidebar
    }
}

/// A styled scrollable list container.
///
/// # Example
///
/// ```rust,ignore
/// List::new("settings-list")
///     .list_style(ListStyle::inset_grouped())
///     .child(
///         Section::new()
///             .header("Account")
///             .child(Text::new("Profile"))
///             .child(Text::new("Privacy"))
///     )
///     .child(
///         Section::new()
///             .header("General")
///             .child(Text::new("Notifications"))
///     )
/// ```
#[derive(IntoElement)]
pub struct List {
    id: SharedString,
    style: ListStyle,
    children: Vec<gpui::AnyElement>,
}

impl List {
    /// Create a new list with the given ID.
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: ListStyle::Automatic,
            children: Vec::new(),
        }
    }

    /// Set the list style.
    pub fn list_style(mut self, style: ListStyle) -> Self {
        self.style = style;
        self
    }

    impl_child_methods!();
}

impl Modifier for List {}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut container = div()
            .id(self.id)
            .size_full()
            .flex()
            .flex_col()
            .overflow_y_scroll();

        // Apply style-specific styling
        match self.style {
            ListStyle::Automatic | ListStyle::Plain => {
                // Plain appearance - no extra styling
            }
            ListStyle::InsetGrouped | ListStyle::Grouped => {
                // Add padding for grouped appearance
                container = container.p(px(16.0));
            }
            ListStyle::Sidebar => {
                // Sidebar typically has different padding
                container = container.p(px(8.0));
            }
        }

        // Add spacing between children (sections)
        container = container.gap(px(24.0));

        container.children(self.children)
    }
}

/// A section within a List with optional header and footer.
///
/// # Example
///
/// ```rust,ignore
/// Section::new()
///     .header("Account Settings")
///     .footer("Changes are saved automatically")
///     .child(Text::new("Profile"))
///     .child(Text::new("Privacy"))
/// ```
#[derive(IntoElement)]
pub struct Section {
    header: Option<SharedString>,
    footer: Option<SharedString>,
    children: Vec<gpui::AnyElement>,
    dark_mode: bool,
}

impl Section {
    /// Create a new section.
    pub fn new() -> Self {
        Self {
            header: None,
            footer: None,
            children: Vec::new(),
            dark_mode: false,
        }
    }

    /// Force dark mode colors for this section.
    ///
    /// Note: Since v0.2.0, semantic colors auto-adapt to the theme.
    /// This method is now only needed to force dark mode regardless of theme.
    #[deprecated(
        since = "0.2.0",
        note = "Semantic colors now auto-adapt. Only use if you need to force dark mode."
    )]
    pub fn dark_mode(mut self, enabled: bool) -> Self {
        self.dark_mode = enabled;
        self
    }

    /// Set the section header text.
    pub fn header(mut self, text: impl Into<SharedString>) -> Self {
        self.header = Some(text.into());
        self
    }

    /// Set the section footer text.
    pub fn footer(mut self, text: impl Into<SharedString>) -> Self {
        self.footer = Some(text.into());
        self
    }

    impl_child_methods!();
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for Section {}

impl RenderOnce for Section {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use gpui_component::ActiveTheme;

        // Get dark mode from theme (overridden by explicit dark_mode setting)
        let is_dark = if self.dark_mode {
            true
        } else {
            cx.theme().is_dark()
        };

        // Use semantic colors that auto-resolve
        let label_color = Color::secondary_label().resolve(is_dark);
        let bg_color = Color::tertiary_system_background().resolve(is_dark);
        let separator_color = Color::separator().resolve(is_dark);

        let mut section = div().flex().flex_col().w_full();

        // Header
        if let Some(header_text) = self.header {
            section = section.child(
                div()
                    .text_xs()
                    .text_color(label_color)
                    .px(px(16.0))
                    .pb(px(8.0))
                    .child(header_text.to_uppercase()),
            );
        }

        // Content container with rounded corners and background
        let mut content = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(bg_color)
            .rounded(px(10.0))
            .overflow_hidden();

        // Add rows with separators
        let row_count = self.children.len();
        for (index, child) in self.children.into_iter().enumerate() {
            let is_last = index == row_count - 1;

            let mut row = div()
                .flex()
                .items_center()
                .min_h(px(44.0))
                .px(px(16.0))
                .child(child);

            // Add separator for all but the last row
            if !is_last {
                row = row.border_b_1().border_color(separator_color);
            }

            content = content.child(row);
        }

        section = section.child(content);

        // Footer
        if let Some(footer_text) = self.footer {
            section = section.child(
                div()
                    .text_xs()
                    .text_color(label_color)
                    .px(px(16.0))
                    .pt(px(8.0))
                    .child(footer_text),
            );
        }

        section
    }
}
