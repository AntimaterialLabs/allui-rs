//! Grid - A static 2D layout container.
//!
//! Grid arranges children in a two-dimensional layout where columns
//! automatically size to fit their widest cell.

use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, Styled, Window};

use crate::alignment::{Alignment, VerticalAlignment};
use crate::modifier::Modifier;

/// A container that arranges views in a two-dimensional grid layout.
///
/// Unlike LazyVGrid/LazyHGrid, Grid renders all content immediately and
/// automatically sizes columns based on content.
///
/// # Example
///
/// ```rust,ignore
/// Grid::new()
///     .horizontal_spacing(8.0)
///     .vertical_spacing(4.0)
///     .child(
///         GridRow::new()
///             .child(Text::new("Name"))
///             .child(Text::new("Value"))
///     )
///     .child(
///         GridRow::new()
///             .child(Text::new("Width"))
///             .child(Text::new("100px"))
///     )
/// ```
#[derive(IntoElement)]
pub struct Grid {
    horizontal_spacing: f32,
    vertical_spacing: f32,
    alignment: Alignment,
    children: Vec<GridChild>,
}

/// Internal representation of a Grid child.
enum GridChild {
    /// A row with multiple cells.
    Row(GridRow),
    /// A view that spans all columns.
    Spanning(gpui::AnyElement),
}

impl Grid {
    /// Create a new grid.
    pub fn new() -> Self {
        Self {
            horizontal_spacing: 0.0,
            vertical_spacing: 0.0,
            alignment: Alignment::center(),
            children: Vec::new(),
        }
    }

    /// Set the horizontal spacing between columns.
    pub fn horizontal_spacing(mut self, spacing: f32) -> Self {
        self.horizontal_spacing = spacing;
        self
    }

    /// Set the vertical spacing between rows.
    pub fn vertical_spacing(mut self, spacing: f32) -> Self {
        self.vertical_spacing = spacing;
        self
    }

    /// Set both horizontal and vertical spacing.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.horizontal_spacing = spacing;
        self.vertical_spacing = spacing;
        self
    }

    /// Set the default alignment for all cells.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Add a GridRow child.
    pub fn child(mut self, row: GridRow) -> Self {
        self.children.push(GridChild::Row(row));
        self
    }

    /// Add a spanning element (non-GridRow) that spans all columns.
    pub fn spanning_child<E: IntoElement>(mut self, child: E) -> Self {
        self.children
            .push(GridChild::Spanning(child.into_any_element()));
        self
    }

    /// Calculate the maximum number of columns across all rows.
    fn max_columns(&self) -> usize {
        self.children
            .iter()
            .filter_map(|child| match child {
                GridChild::Row(row) => Some(row.cells.len()),
                GridChild::Spanning(_) => None,
            })
            .max()
            .unwrap_or(0)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for Grid {}

impl RenderOnce for Grid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let col_count = self.max_columns();

        if col_count == 0 {
            return div();
        }

        // Create a CSS grid container
        let container = div()
            .grid()
            .grid_cols(col_count as u16)
            .gap_x(px(self.horizontal_spacing))
            .gap_y(px(self.vertical_spacing));

        // Apply alignment to container
        let container = self.alignment.horizontal.apply_as_justify(container);
        let mut container = self.alignment.vertical.apply_as_items(container);

        // Add children
        for child in self.children {
            match child {
                GridChild::Row(row) => {
                    let row_col_count = row.cells.len();
                    for (idx, cell) in row.cells.into_iter().enumerate() {
                        // Apply row-level alignment if specified
                        let mut cell_container = div().child(cell);
                        if let Some(row_alignment) = row.alignment {
                            cell_container = row_alignment.apply_as_items(cell_container);
                        }
                        container = container.child(cell_container);

                        // If this row has fewer columns, add empty cells
                        if idx == row_col_count - 1 && row_col_count < col_count {
                            for _ in 0..(col_count - row_col_count) {
                                container = container.child(div());
                            }
                        }
                    }
                }
                GridChild::Spanning(element) => {
                    // Spanning element takes full width
                    container = container.child(div().col_span_full().child(element));
                }
            }
        }

        container
    }
}

/// A single row within a Grid.
///
/// # Example
///
/// ```rust,ignore
/// GridRow::new()
///     .alignment(VerticalAlignment::Top)
///     .child(Text::new("Label"))
///     .child(Text::new("Value"))
/// ```
pub struct GridRow {
    cells: Vec<gpui::AnyElement>,
    alignment: Option<VerticalAlignment>,
}

impl GridRow {
    /// Create a new grid row.
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            alignment: None,
        }
    }

    /// Set the vertical alignment for cells in this row.
    pub fn alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Add a cell to this row.
    pub fn child<E: IntoElement>(mut self, child: E) -> Self {
        self.cells.push(child.into_any_element());
        self
    }

    /// Add multiple cells to this row.
    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement,
    {
        for child in children {
            self.cells.push(child.into_any_element());
        }
        self
    }
}

impl Default for GridRow {
    fn default() -> Self {
        Self::new()
    }
}
