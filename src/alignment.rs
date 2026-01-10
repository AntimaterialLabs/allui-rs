//! Alignment types for Allui layout.
//!
//! This module defines the core alignment types used throughout Allui
//! for positioning content within containers and frames.
//!
//! # Types
//!
//! - [`Alignment`]: Composite 2D alignment (horizontal + vertical)
//! - [`HorizontalAlignment`]: Horizontal positioning within a container
//! - [`VerticalAlignment`]: Vertical positioning within a container
//!
//! # Examples
//!
//! ```rust,ignore
//! use allui::prelude::*;
//!
//! // VStack uses HorizontalAlignment (controls horizontal positioning of children)
//! VStack::new()
//!     .alignment(HorizontalAlignment::Leading)
//!
//! // HStack uses VerticalAlignment (controls vertical positioning of children)
//! HStack::new()
//!     .alignment(VerticalAlignment::Top)
//!
//! // ZStack uses composite Alignment (controls both axes)
//! ZStack::new()
//!     .alignment(Alignment::bottom_trailing())
//! ```

use gpui::Styled;

/// Alignment for positioning within a frame or stack.
///
/// Combines horizontal and vertical alignment into a single type,
/// useful for 2D positioning in ZStack, Grid, and frame modifiers.
#[derive(Clone, Copy, Debug, Default)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

impl Alignment {
    pub const fn center() -> Self {
        Self {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Center,
        }
    }

    pub const fn top_leading() -> Self {
        Self {
            horizontal: HorizontalAlignment::Leading,
            vertical: VerticalAlignment::Top,
        }
    }

    pub const fn top() -> Self {
        Self {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Top,
        }
    }

    pub const fn top_trailing() -> Self {
        Self {
            horizontal: HorizontalAlignment::Trailing,
            vertical: VerticalAlignment::Top,
        }
    }

    pub const fn leading() -> Self {
        Self {
            horizontal: HorizontalAlignment::Leading,
            vertical: VerticalAlignment::Center,
        }
    }

    pub const fn trailing() -> Self {
        Self {
            horizontal: HorizontalAlignment::Trailing,
            vertical: VerticalAlignment::Center,
        }
    }

    pub const fn bottom_leading() -> Self {
        Self {
            horizontal: HorizontalAlignment::Leading,
            vertical: VerticalAlignment::Bottom,
        }
    }

    pub const fn bottom() -> Self {
        Self {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Bottom,
        }
    }

    pub const fn bottom_trailing() -> Self {
        Self {
            horizontal: HorizontalAlignment::Trailing,
            vertical: VerticalAlignment::Bottom,
        }
    }
}

/// Horizontal alignment within a container.
///
/// Used by VStack to control horizontal positioning of children,
/// and by Grid/Frame for horizontal alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HorizontalAlignment {
    Leading,
    #[default]
    Center,
    Trailing,
}

impl HorizontalAlignment {
    /// Apply as cross-axis alignment using flexbox `align-items`.
    ///
    /// Used by VStack where horizontal alignment controls the cross-axis.
    pub fn apply_as_items<S: Styled>(self, styled: S) -> S {
        match self {
            Self::Leading => styled.items_start(),
            Self::Center => styled.items_center(),
            Self::Trailing => styled.items_end(),
        }
    }

    /// Apply as main-axis alignment using flexbox `justify-content`.
    ///
    /// Used by ZStack, Frame, and Grid where horizontal alignment
    /// controls the main-axis of row-direction containers.
    pub fn apply_as_justify<S: Styled>(self, styled: S) -> S {
        match self {
            Self::Leading => styled.justify_start(),
            Self::Center => styled.justify_center(),
            Self::Trailing => styled.justify_end(),
        }
    }
}

/// Vertical alignment within a container.
///
/// Matches SwiftUI's `VerticalAlignment` type.
///
/// Used by HStack to control vertical positioning of children,
/// and by Grid/Frame for vertical alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Align to the top edge.
    Top,
    /// Align to the vertical center.
    #[default]
    Center,
    /// Align to the bottom edge.
    Bottom,
    /// Align to the baseline of the first line of text.
    ///
    /// In CSS/Flexbox, this maps to `align-items: baseline`.
    FirstTextBaseline,
    /// Align to the baseline of the last line of text.
    ///
    /// Note: GPUI's flexbox only supports a single baseline alignment,
    /// so this behaves identically to `FirstTextBaseline` in practice.
    /// The distinction is preserved for SwiftUI API compatibility.
    LastTextBaseline,
}

impl VerticalAlignment {
    /// Apply as cross-axis alignment using flexbox `align-items`.
    ///
    /// Used by HStack, ZStack, Frame, and Grid where vertical alignment
    /// controls the cross-axis positioning.
    pub fn apply_as_items<S: Styled>(self, styled: S) -> S {
        match self {
            Self::Top => styled.items_start(),
            Self::Center => styled.items_center(),
            Self::Bottom => styled.items_end(),
            Self::FirstTextBaseline | Self::LastTextBaseline => styled.items_baseline(),
        }
    }
}
