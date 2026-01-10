//! LazyVGrid - A vertically-scrolling grid with fixed columns.
//!
//! Items flow left-to-right, top-to-bottom. Renders lazily for performance.

use std::rc::Rc;

use gpui::{div, px, AnyElement, App, Entity, IntoElement, ParentElement, Render, Styled, Window};

use crate::layout::grid_item::{GridItem, GridItemSize};
use crate::modifier::Modifier;

// Re-export for convenience
pub use gpui_component::VirtualListScrollHandle;

/// Type alias for the item render function.
type LazyGridRenderFn<V> = Rc<dyn Fn(&V, usize, &mut Window, &mut App) -> AnyElement>;

/// A vertically-scrolling grid with fixed columns.
///
/// Items are laid out left-to-right, top-to-bottom. The grid renders
/// lazily for performance with large datasets.
///
/// # Example
///
/// ```rust,ignore
/// let columns = vec![
///     GridItem::flexible(),
///     GridItem::flexible(),
///     GridItem::fixed(100.0),
/// ];
///
/// LazyVGrid::new(cx.entity().clone(), "photo-grid", &self.scroll_handle)
///     .columns(columns)
///     .spacing(8.0)
///     .item_count(self.photos.len())
///     .render_item(|view, index, _, _| {
///         PhotoCell::new(&view.photos[index])
///     })
///     .build(window, cx)
/// ```
pub struct LazyVGrid<V: Render + 'static> {
    entity: Entity<V>,
    element_id: &'static str,
    scroll_handle: VirtualListScrollHandle,
    columns: Vec<GridItem>,
    horizontal_spacing: f32,
    vertical_spacing: f32,
    item_count: usize,
    render_fn: Option<LazyGridRenderFn<V>>,
    /// Container width for adaptive column calculation.
    /// If not set, defaults to 400.0 for adaptive columns.
    container_width: Option<f32>,
    /// Row height for virtualization. Defaults to 100.0.
    row_height: f32,
}

impl<V: Render + 'static> LazyVGrid<V> {
    /// Create a new lazy vertical grid.
    ///
    /// # Arguments
    ///
    /// * `entity` - The GPUI entity for this view (`cx.entity().clone()`)
    /// * `element_id` - A unique identifier for this grid
    /// * `scroll_handle` - Reference to a VirtualListScrollHandle stored in your view
    pub fn new(
        entity: Entity<V>,
        element_id: &'static str,
        scroll_handle: &VirtualListScrollHandle,
    ) -> Self {
        Self {
            entity,
            element_id,
            scroll_handle: scroll_handle.clone(),
            columns: vec![GridItem::flexible()],
            horizontal_spacing: 0.0,
            vertical_spacing: 0.0,
            item_count: 0,
            render_fn: None,
            container_width: None,
            row_height: 100.0,
        }
    }

    /// Set the column definitions.
    pub fn columns(mut self, columns: Vec<GridItem>) -> Self {
        self.columns = columns;
        self
    }

    /// Set both horizontal and vertical spacing.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.horizontal_spacing = spacing;
        self.vertical_spacing = spacing;
        self
    }

    /// Set horizontal spacing between columns.
    pub fn horizontal_spacing(mut self, spacing: f32) -> Self {
        self.horizontal_spacing = spacing;
        self
    }

    /// Set vertical spacing between rows.
    pub fn vertical_spacing(mut self, spacing: f32) -> Self {
        self.vertical_spacing = spacing;
        self
    }

    /// Set the number of items in the grid.
    pub fn item_count(mut self, count: usize) -> Self {
        self.item_count = count;
        self
    }

    /// Set the container width for adaptive column calculation.
    ///
    /// This is required for `GridItem::adaptive()` to calculate
    /// the correct number of columns.
    pub fn container_width(mut self, width: f32) -> Self {
        self.container_width = Some(width);
        self
    }

    /// Set the row height for virtualization.
    ///
    /// Defaults to 100.0. Set this to match your item heights for
    /// accurate scrolling behavior.
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height;
        self
    }

    /// Set the render function for items.
    ///
    /// The function receives the view, item index, window, and app context,
    /// and should return the element to render for that item.
    pub fn render_item<F, E>(mut self, render_fn: F) -> Self
    where
        F: Fn(&V, usize, &mut Window, &mut App) -> E + 'static,
        E: IntoElement,
    {
        self.render_fn = Some(Rc::new(move |view, index, window, cx| {
            render_fn(view, index, window, cx).into_any_element()
        }));
        self
    }

    /// Calculate the number of columns (handling Adaptive sizing).
    ///
    /// For Fixed and Flexible columns, returns the number of column definitions.
    /// For Adaptive columns, calculates how many columns fit in the available width.
    fn column_count(&self, available_width: f32) -> usize {
        // Check if any column is adaptive
        for col in &self.columns {
            if let GridItemSize::Adaptive { min } = col.size {
                // Calculate how many columns fit
                // Formula: (available_width + spacing) / (min_width + spacing)
                let effective_spacing = self.horizontal_spacing;
                let columns = ((available_width + effective_spacing) / (min + effective_spacing))
                    .floor() as usize;
                return columns.max(1);
            }
        }

        // For non-adaptive, just use the column count
        self.columns.len().max(1)
    }

    /// Expand adaptive columns into the calculated count.
    ///
    /// If columns contain an Adaptive item, expands it to the calculated column count.
    /// Returns the effective columns to use for rendering.
    fn effective_columns(&self, available_width: f32) -> Vec<GridItem> {
        let mut result = Vec::new();

        for col in &self.columns {
            if let GridItemSize::Adaptive { min } = col.size {
                // Calculate how many columns fit
                let col_count = self.column_count(available_width);

                // Create that many flexible columns with the min width
                for _ in 0..col_count {
                    result.push(GridItem {
                        size: GridItemSize::Flexible {
                            min,
                            max: f32::INFINITY,
                        },
                        spacing: col.spacing,
                        alignment: col.alignment,
                    });
                }
            } else {
                result.push(col.clone());
            }
        }

        if result.is_empty() {
            result.push(GridItem::flexible());
        }

        result
    }

    /// Calculate row count based on item count and column count.
    fn row_count(&self, col_count: usize) -> usize {
        if col_count == 0 || self.item_count == 0 {
            0
        } else {
            self.item_count.div_ceil(col_count)
        }
    }

    /// Build and return the virtual grid element.
    pub fn build(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Use provided container width or default to 400.0 for adaptive calculation
        let available_width = self.container_width.unwrap_or(400.0);

        // Get effective columns (expanding adaptive if needed)
        let effective_cols = self.effective_columns(available_width);
        let col_count = effective_cols.len();
        let row_count = self.row_count(col_count);
        let item_count = self.item_count;
        let horizontal_spacing = self.horizontal_spacing;
        let vertical_spacing = self.vertical_spacing;
        let row_height = self.row_height;

        let item_sizes = Rc::new(vec![
            gpui::size(
                px(10000.0),
                px(row_height + vertical_spacing)
            );
            row_count
        ]);

        let render_fn = self.render_fn;
        let columns = effective_cols;

        gpui_component::v_virtual_list(
            self.entity,
            self.element_id,
            item_sizes,
            move |view, visible_range, window, cx| {
                visible_range
                    .map(|row_idx| {
                        // Render a row of items
                        let mut row = div().flex().flex_row().gap(px(horizontal_spacing));

                        // Apply column sizing
                        for (col_idx, col) in columns.iter().enumerate() {
                            let item_idx = row_idx * col_count + col_idx;

                            let mut cell = div();

                            // Apply column width
                            cell = match col.size {
                                GridItemSize::Fixed(size) => cell.w(px(size)),
                                GridItemSize::Flexible { .. } => cell.flex_1(),
                                GridItemSize::Adaptive { min } => cell.min_w(px(min)).flex_1(),
                            };

                            // Render item if within bounds
                            if item_idx < item_count {
                                if let Some(ref render) = render_fn {
                                    cell = cell.child(render(view, item_idx, window, cx));
                                }
                            }

                            row = row.child(cell);
                        }

                        row.into_any_element()
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle)
    }
}

impl<V: Render + 'static> Modifier for LazyVGrid<V> {}
