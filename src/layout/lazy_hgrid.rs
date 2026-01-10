//! LazyHGrid - A horizontally-scrolling grid with fixed rows.
//!
//! Items flow top-to-bottom, left-to-right. Renders lazily for performance.

use std::rc::Rc;

use gpui::{div, px, AnyElement, App, Entity, IntoElement, ParentElement, Render, Styled, Window};

use crate::layout::grid_item::{GridItem, GridItemSize};
use crate::modifier::Modifier;

// Re-export for convenience
pub use gpui_component::VirtualListScrollHandle;

/// Type alias for the item render function.
type LazyGridRenderFn<V> = Rc<dyn Fn(&V, usize, &mut Window, &mut App) -> AnyElement>;

/// A horizontally-scrolling grid with fixed rows.
///
/// Items are laid out top-to-bottom, left-to-right. The grid renders
/// lazily for performance with large datasets.
///
/// # Example
///
/// ```rust,ignore
/// let rows = vec![
///     GridItem::fixed(80.0),
///     GridItem::fixed(80.0),
/// ];
///
/// LazyHGrid::new(cx.entity().clone(), "category-grid", &self.scroll_handle)
///     .rows(rows)
///     .spacing(12.0)
///     .item_count(self.items.len())
///     .render_item(|view, index, _, _| {
///         CategoryCard::new(&view.items[index])
///     })
///     .build(window, cx)
/// ```
pub struct LazyHGrid<V: Render + 'static> {
    entity: Entity<V>,
    element_id: &'static str,
    scroll_handle: VirtualListScrollHandle,
    rows: Vec<GridItem>,
    horizontal_spacing: f32,
    vertical_spacing: f32,
    item_count: usize,
    render_fn: Option<LazyGridRenderFn<V>>,
    /// Container height for adaptive row calculation.
    /// If not set, defaults to 300.0 for adaptive rows.
    container_height: Option<f32>,
    /// Column width for virtualization. Defaults to 100.0.
    column_width: f32,
}

impl<V: Render + 'static> LazyHGrid<V> {
    /// Create a new lazy horizontal grid.
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
            rows: vec![GridItem::flexible()],
            horizontal_spacing: 0.0,
            vertical_spacing: 0.0,
            item_count: 0,
            render_fn: None,
            container_height: None,
            column_width: 100.0,
        }
    }

    /// Set the row definitions.
    pub fn rows(mut self, rows: Vec<GridItem>) -> Self {
        self.rows = rows;
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

    /// Set the container height for adaptive row calculation.
    ///
    /// This is required for `GridItem::adaptive()` to calculate
    /// the correct number of rows.
    pub fn container_height(mut self, height: f32) -> Self {
        self.container_height = Some(height);
        self
    }

    /// Set the column width for virtualization.
    ///
    /// Defaults to 100.0. Set this to match your item widths for
    /// accurate scrolling behavior.
    pub fn column_width(mut self, width: f32) -> Self {
        self.column_width = width;
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

    /// Calculate the number of rows (handling Adaptive sizing).
    ///
    /// For Fixed and Flexible rows, returns the number of row definitions.
    /// For Adaptive rows, calculates how many rows fit in the available height.
    fn row_count(&self, available_height: f32) -> usize {
        // Check if any row is adaptive
        for row in &self.rows {
            if let GridItemSize::Adaptive { min } = row.size {
                // Calculate how many rows fit
                let effective_spacing = self.vertical_spacing;
                let rows = ((available_height + effective_spacing) / (min + effective_spacing))
                    .floor() as usize;
                return rows.max(1);
            }
        }

        // For non-adaptive, just use the row count
        self.rows.len().max(1)
    }

    /// Expand adaptive rows into the calculated count.
    fn effective_rows(&self, available_height: f32) -> Vec<GridItem> {
        let mut result = Vec::new();

        for row in &self.rows {
            if let GridItemSize::Adaptive { min } = row.size {
                let row_count = self.row_count(available_height);

                for _ in 0..row_count {
                    result.push(GridItem {
                        size: GridItemSize::Flexible {
                            min,
                            max: f32::INFINITY,
                        },
                        spacing: row.spacing,
                        alignment: row.alignment,
                    });
                }
            } else {
                result.push(row.clone());
            }
        }

        if result.is_empty() {
            result.push(GridItem::flexible());
        }

        result
    }

    /// Calculate column count based on item count and row count.
    fn column_count(&self, row_count: usize) -> usize {
        if row_count == 0 || self.item_count == 0 {
            0
        } else {
            self.item_count.div_ceil(row_count)
        }
    }

    /// Build and return the virtual grid element.
    pub fn build(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Use provided container height or default to 300.0 for adaptive calculation
        let available_height = self.container_height.unwrap_or(300.0);

        // Get effective rows (expanding adaptive if needed)
        let effective_rows = self.effective_rows(available_height);
        let row_count = effective_rows.len();
        let col_count = self.column_count(row_count);
        let item_count = self.item_count;
        let horizontal_spacing = self.horizontal_spacing;
        let vertical_spacing = self.vertical_spacing;
        let col_width = self.column_width;

        let item_sizes = Rc::new(vec![
            gpui::size(
                px(col_width + horizontal_spacing),
                px(10000.0)
            );
            col_count
        ]);

        let render_fn = self.render_fn;
        let rows = effective_rows;

        gpui_component::h_virtual_list(
            self.entity,
            self.element_id,
            item_sizes,
            move |view, visible_range, window, cx| {
                visible_range
                    .map(|col_idx| {
                        // Render a column of items
                        let mut col = div().flex().flex_col().gap(px(vertical_spacing));

                        // Apply row sizing
                        for (row_idx, row) in rows.iter().enumerate() {
                            let item_idx = col_idx * row_count + row_idx;

                            let mut cell = div();

                            // Apply row height
                            cell = match row.size {
                                GridItemSize::Fixed(size) => cell.h(px(size)),
                                GridItemSize::Flexible { .. } => cell.flex_1(),
                                GridItemSize::Adaptive { min } => cell.min_h(px(min)).flex_1(),
                            };

                            // Render item if within bounds
                            if item_idx < item_count {
                                if let Some(ref render) = render_fn {
                                    cell = cell.child(render(view, item_idx, window, cx));
                                }
                            }

                            col = col.child(cell);
                        }

                        col.into_any_element()
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle)
    }
}

impl<V: Render + 'static> Modifier for LazyHGrid<V> {}
