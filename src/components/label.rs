//! Label - Text with icon.

use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};
use gpui_component::{ActiveTheme, Icon, IconName};

use crate::modifier::Modifier;
use crate::style::Color;

/// A standard label for user interface items, consisting of an icon and a title.
///
/// # Example
///
/// ```rust,ignore
/// // Using SF Symbol-style name (mapped to gpui-component icons)
/// Label::new("star.fill", "Favorites")
///     .foreground_color(Color::yellow())
///
/// // Using IconName directly
/// Label::with_icon(IconName::Star, "Favorites")
/// ```
#[derive(IntoElement)]
pub struct Label {
    title: SharedString,
    icon: Option<LabelIcon>,
    color: Option<Color>,
}

/// The icon to display in a Label.
enum LabelIcon {
    /// A gpui-component IconName.
    Name(IconName),
    /// A string name that will be mapped to an IconName if possible.
    String(SharedString),
}

impl Label {
    /// Create a new label with a system image name and title.
    ///
    /// The system image name is mapped to gpui-component icons where possible.
    /// Common mappings:
    /// - "star", "star.fill" -> Star
    /// - "folder", "folder.fill" -> Folder
    /// - "gear", "gearshape" -> Settings
    /// - "heart", "heart.fill" -> Heart
    /// - "bell", "bell.fill" -> Bell
    /// - "magnifyingglass" -> Search
    /// - "plus" -> Plus
    /// - "minus" -> Minus
    /// - "checkmark" -> Check
    /// - "xmark" -> Close
    pub fn new(system_image: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: Some(LabelIcon::String(system_image.into())),
            color: None,
        }
    }

    /// Create a label with a specific IconName.
    pub fn with_icon(icon: IconName, title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: Some(LabelIcon::Name(icon)),
            color: None,
        }
    }

    /// Create a label with just a title.
    pub fn title(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            color: None,
        }
    }

    /// Set the foreground color for both icon and text.
    pub fn foreground_color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Modifier for Label {}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().is_dark();
        let mut container = div().flex().flex_row().items_center().gap(px(6.0));

        let color = self.color.map(|c| c.resolve(is_dark));

        // Apply color to container for text
        if let Some(c) = color {
            container = container.text_color(c);
        }

        // Add icon if present
        if let Some(label_icon) = self.icon {
            let (icon_name, fallback_text) = match &label_icon {
                LabelIcon::Name(name) => (Some(name.clone()), None),
                LabelIcon::String(s) => (map_system_image_to_icon(s), Some(s.clone())),
            };

            if let Some(name) = icon_name {
                let mut icon = Icon::new(name);
                if let Some(c) = color {
                    icon = icon.text_color(c);
                }
                container = container.child(icon);
            } else if let Some(text) = fallback_text {
                // Fallback to text placeholder for unmapped icons
                container = container.child(div().text_size(px(12.0)).child(format!("[{}]", text)));
            }
        }

        // Add title
        container = container.child(self.title);

        container
    }
}

/// Maps SF Symbol-style names to gpui-component IconName.
fn map_system_image_to_icon(name: &str) -> Option<IconName> {
    // Normalize: remove ".fill" suffix and convert to lowercase
    let normalized = name.to_lowercase().replace(".fill", "");

    match normalized.as_str() {
        "star" => Some(IconName::Star),
        "star.off" | "staroff" => Some(IconName::StarOff),
        "folder" => Some(IconName::Folder),
        "folder.open" | "folderopen" => Some(IconName::FolderOpen),
        "gear" | "gearshape" | "settings" => Some(IconName::Settings),
        "heart" => Some(IconName::Heart),
        "heart.off" | "heartoff" => Some(IconName::HeartOff),
        "bell" => Some(IconName::Bell),
        "magnifyingglass" | "search" => Some(IconName::Search),
        "plus" | "plus.circle" => Some(IconName::Plus),
        "minus" | "minus.circle" => Some(IconName::Minus),
        "checkmark" | "check" | "checkmark.circle" => Some(IconName::Check),
        "xmark" | "close" | "xmark.circle" => Some(IconName::Close),
        "trash" | "delete" => Some(IconName::Delete),
        "doc" | "file" | "doc.text" => Some(IconName::File),
        "info" | "info.circle" => Some(IconName::Info),
        "globe" => Some(IconName::Globe),
        "sun" | "sun.max" => Some(IconName::Sun),
        "moon" => Some(IconName::Moon),
        "person" | "person.circle" | "user" => Some(IconName::User),
        "eye" => Some(IconName::Eye),
        "eye.slash" | "eyeoff" => Some(IconName::EyeOff),
        "arrow.up" | "arrowup" => Some(IconName::ArrowUp),
        "arrow.down" | "arrowdown" => Some(IconName::ArrowDown),
        "arrow.left" | "arrowleft" => Some(IconName::ArrowLeft),
        "arrow.right" | "arrowright" => Some(IconName::ArrowRight),
        "chevron.up" | "chevronup" => Some(IconName::ChevronUp),
        "chevron.down" | "chevrondown" => Some(IconName::ChevronDown),
        "chevron.left" | "chevronleft" => Some(IconName::ChevronLeft),
        "chevron.right" | "chevronright" => Some(IconName::ChevronRight),
        "calendar" => Some(IconName::Calendar),
        "link" | "link.circle" => Some(IconName::ExternalLink),
        "doc.on.doc" | "copy" => Some(IconName::Copy),
        "ellipsis" | "ellipsis.circle" => Some(IconName::Ellipsis),
        "line.horizontal.3" | "menu" => Some(IconName::Menu),
        "square.and.arrow.up" | "redo" => Some(IconName::Redo),
        "square.and.arrow.down" | "undo" => Some(IconName::Undo),
        "exclamationmark.triangle" | "warning" => Some(IconName::TriangleAlert),
        "hand.thumbsup" | "thumbsup" => Some(IconName::ThumbsUp),
        "hand.thumbsdown" | "thumbsdown" => Some(IconName::ThumbsDown),
        "tray" | "inbox" => Some(IconName::Inbox),
        "book" | "book.closed" => Some(IconName::BookOpen),
        _ => None,
    }
}
