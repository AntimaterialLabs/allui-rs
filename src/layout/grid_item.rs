//! GridItem - Describes column/row sizing for lazy grids.
//!
//! GridItem is used by LazyVGrid to define columns and by LazyHGrid to define rows.

use crate::alignment::Alignment;

/// The sizing behavior for a grid column or row.
#[derive(Clone, Copy, Debug)]
pub enum GridItemSize {
    /// A fixed size in pixels.
    Fixed(f32),
    /// A flexible size that expands to fill available space.
    /// Constrained by min and max values.
    Flexible {
        /// Minimum size in pixels.
        min: f32,
        /// Maximum size in pixels (f32::INFINITY for unbounded).
        max: f32,
    },
    /// Creates as many columns/rows as fit in the available space.
    /// Each item will be at least `min` pixels.
    Adaptive {
        /// Minimum size for each item.
        min: f32,
    },
}

impl GridItemSize {
    /// Create a fixed size.
    pub fn fixed(size: f32) -> Self {
        Self::Fixed(size)
    }

    /// Create a flexible size with default constraints.
    pub fn flexible() -> Self {
        Self::Flexible {
            min: 10.0,
            max: f32::INFINITY,
        }
    }

    /// Create a flexible size with custom constraints.
    pub fn flexible_range(min: f32, max: f32) -> Self {
        Self::Flexible { min, max }
    }

    /// Create an adaptive size.
    pub fn adaptive(min: f32) -> Self {
        Self::Adaptive { min }
    }
}

/// Describes a single column (in LazyVGrid) or row (in LazyHGrid).
///
/// # Example
///
/// ```rust,ignore
/// let columns = vec![
///     GridItem::flexible(),
///     GridItem::flexible(),
///     GridItem::fixed(100.0),
/// ];
/// ```
#[derive(Clone, Debug)]
pub struct GridItem {
    /// The size specification for this item.
    pub size: GridItemSize,
    /// Optional spacing after this item (overrides grid's default spacing).
    pub spacing: Option<f32>,
    /// Optional alignment for content in this column/row.
    pub alignment: Option<Alignment>,
}

impl GridItem {
    /// Create a grid item with fixed size.
    pub fn fixed(size: f32) -> Self {
        Self {
            size: GridItemSize::Fixed(size),
            spacing: None,
            alignment: None,
        }
    }

    /// Create a grid item with flexible size (default min: 10, max: infinity).
    pub fn flexible() -> Self {
        Self {
            size: GridItemSize::flexible(),
            spacing: None,
            alignment: None,
        }
    }

    /// Create a grid item with flexible size and custom constraints.
    pub fn flexible_range(min: f32, max: f32) -> Self {
        Self {
            size: GridItemSize::flexible_range(min, max),
            spacing: None,
            alignment: None,
        }
    }

    /// Create a grid item with adaptive size.
    ///
    /// The grid will create as many columns/rows as fit, each at least `min` pixels.
    pub fn adaptive(min: f32) -> Self {
        Self {
            size: GridItemSize::Adaptive { min },
            spacing: None,
            alignment: None,
        }
    }

    /// Set custom spacing after this item.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// Set alignment for content in this column/row.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }
}

impl Default for GridItem {
    fn default() -> Self {
        Self::flexible()
    }
}
