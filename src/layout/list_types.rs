//! Types for List layout configuration.

use crate::modifier::Padding;

use super::ListStyle;

/// Edge insets for list row content positioning.
/// Type alias for [`Padding`] with SwiftUI-compatible naming.
pub type EdgeInsets = Padding;

pub trait EdgeInsetsExt {
    /// Matches SwiftUI's `EdgeInsets(top:leading:bottom:trailing:)` initializer.
    fn init(top: f32, leading: f32, bottom: f32, trailing: f32) -> Self;
}

impl EdgeInsetsExt for EdgeInsets {
    fn init(top: f32, leading: f32, bottom: f32, trailing: f32) -> Self {
        Padding::edges(top, leading, bottom, trailing)
    }
}

/// Spacing options between adjacent sections in a list.
/// Matches SwiftUI's `ListSectionSpacing` struct.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ListSectionSpacing {
    /// Platform default spacing between sections. Resolves based on ListStyle.
    #[default]
    Default,
    /// Compact spacing - reduced visual separation.
    Compact,
    /// Custom spacing value in points.
    Custom(f32),
}

impl ListSectionSpacing {
    #[must_use]
    pub fn default_spacing() -> Self {
        Self::Default
    }

    #[must_use]
    pub fn compact() -> Self {
        Self::Compact
    }

    #[must_use]
    pub fn custom(value: f32) -> Self {
        Self::Custom(value)
    }

    /// Resolve to actual pixel value based on list style.
    #[must_use]
    pub fn resolve(&self, style: ListStyle) -> f32 {
        match self {
            Self::Default => match style {
                ListStyle::InsetGrouped | ListStyle::Grouped => 24.0,
                ListStyle::Plain | ListStyle::Automatic => 16.0,
                ListStyle::Sidebar => 12.0,
            },
            Self::Compact => match style {
                ListStyle::InsetGrouped | ListStyle::Grouped => 8.0,
                _ => 4.0,
            },
            Self::Custom(v) => *v,
        }
    }
}

impl From<f32> for ListSectionSpacing {
    fn from(value: f32) -> Self {
        Self::Custom(value)
    }
}

/// Set of edges for margin application. Matches SwiftUI's `Edge.Set`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EdgeSet {
    pub top: bool,
    pub leading: bool,
    pub bottom: bool,
    pub trailing: bool,
}

impl EdgeSet {
    #[must_use]
    pub const fn all() -> Self {
        Self {
            top: true,
            leading: true,
            bottom: true,
            trailing: true,
        }
    }

    #[must_use]
    pub const fn horizontal() -> Self {
        Self {
            top: false,
            leading: true,
            bottom: false,
            trailing: true,
        }
    }

    #[must_use]
    pub const fn vertical() -> Self {
        Self {
            top: true,
            leading: false,
            bottom: true,
            trailing: false,
        }
    }

    #[must_use]
    pub const fn top() -> Self {
        Self {
            top: true,
            leading: false,
            bottom: false,
            trailing: false,
        }
    }

    #[must_use]
    pub const fn bottom() -> Self {
        Self {
            top: false,
            leading: false,
            bottom: true,
            trailing: false,
        }
    }

    #[must_use]
    pub const fn leading() -> Self {
        Self {
            top: false,
            leading: true,
            bottom: false,
            trailing: false,
        }
    }

    #[must_use]
    pub const fn trailing() -> Self {
        Self {
            top: false,
            leading: false,
            bottom: false,
            trailing: true,
        }
    }
}

/// Margins around a section. Matches SwiftUI's `listSectionMargins(_:_:)` modifier.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SectionMargins {
    pub edges: EdgeSet,
    pub length: Option<f32>,
}

impl SectionMargins {
    #[must_use]
    pub fn new(edges: EdgeSet, length: Option<f32>) -> Self {
        Self { edges, length }
    }

    #[must_use]
    pub fn all(length: f32) -> Self {
        Self {
            edges: EdgeSet::all(),
            length: Some(length),
        }
    }

    #[must_use]
    pub fn horizontal(length: f32) -> Self {
        Self {
            edges: EdgeSet::horizontal(),
            length: Some(length),
        }
    }

    #[must_use]
    pub fn vertical(length: f32) -> Self {
        Self {
            edges: EdgeSet::vertical(),
            length: Some(length),
        }
    }
}

/// Configuration for an individual row within a Section.
#[derive(Clone, Debug, Default)]
pub struct RowConfiguration {
    pub insets: Option<EdgeInsets>,
    pub spacing: Option<f32>,
}

impl RowConfiguration {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn insets(mut self, insets: impl Into<EdgeInsets>) -> Self {
        self.insets = Some(insets.into());
        self
    }

    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }
}

/// Configuration passed from List to its child Sections.
#[derive(Clone, Debug, Default)]
pub struct ListConfiguration {
    pub default_row_insets: Option<EdgeInsets>,
    pub default_row_spacing: Option<f32>,
    pub default_section_spacing: ListSectionSpacing,
    pub min_row_height: Option<f32>,
    pub min_header_height: Option<f32>,
    pub style: ListStyle,
}
