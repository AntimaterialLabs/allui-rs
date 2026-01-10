//! Lazy stack components for virtualized rendering.
//!
//! These components render only visible items for efficient handling of large datasets.
//! Unlike regular VStack/HStack, they require integration with GPUI's state management.
//!
//! # Usage
//!
//! LazyVStack and LazyHStack are designed to be used within GPUI views that implement
//! the `Render` trait. They require:
//!
//! 1. A `VirtualListScrollHandle` stored in your view struct
//! 2. Access to the view's `Entity<Self>` via `cx.entity().clone()`
//!
//! # Example
//!
//! ```rust,ignore
//! use allui::prelude::*;
//! use gpui::{Entity, Context, Window, Render};
//!
//! pub struct MyListView {
//!     items: Vec<String>,
//!     scroll_handle: VirtualListScrollHandle,
//! }
//!
//! impl MyListView {
//!     pub fn new() -> Self {
//!         Self {
//!             items: (0..10000).map(|i| format!("Item {}", i)).collect(),
//!             scroll_handle: VirtualListScrollHandle::new(),
//!         }
//!     }
//! }
//!
//! impl Render for MyListView {
//!     fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         LazyVStack::new(cx.entity().clone(), "my-list", &self.scroll_handle)
//!             .item_height(44.0)
//!             .item_count(self.items.len())
//!             .render_item(|view, index, _window, _cx| {
//!                 Text::new(&view.items[index])
//!                     .padding(8.0)
//!             })
//!     }
//! }
//! ```

use std::rc::Rc;

use gpui::{px, size, AnyElement, App, Entity, IntoElement, Pixels, Render, Size, Window};

use crate::alignment::HorizontalAlignment;
use crate::modifier::Modifier;

// Re-export for convenience
pub use gpui_component::VirtualListScrollHandle;

/// Type alias for the item render function used by lazy stacks.
type LazyRenderFn<V> = Rc<dyn Fn(&V, usize, &mut Window, &mut App) -> AnyElement>;

/// A vertically scrolling container that only renders visible items.
///
/// Use this for large lists where rendering all items would be inefficient.
/// Items are rendered lazily as they come into view.
pub struct LazyVStack<V: Render + 'static> {
    entity: Entity<V>,
    element_id: &'static str,
    scroll_handle: VirtualListScrollHandle,
    item_count: usize,
    item_height: f32,
    spacing: f32,
    alignment: HorizontalAlignment,
    render_fn: Option<LazyRenderFn<V>>,
}

impl<V: Render + 'static> LazyVStack<V> {
    /// Create a new lazy vertical stack.
    ///
    /// # Arguments
    ///
    /// * `entity` - The GPUI entity for this view (`cx.entity().clone()`)
    /// * `element_id` - A unique identifier for this list
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
            item_count: 0,
            item_height: 44.0,
            spacing: 0.0,
            alignment: HorizontalAlignment::Center,
            render_fn: None,
        }
    }

    /// Set the number of items in the list.
    pub fn item_count(mut self, count: usize) -> Self {
        self.item_count = count;
        self
    }

    /// Set the height of each item in pixels.
    pub fn item_height(mut self, height: f32) -> Self {
        self.item_height = height;
        self
    }

    /// Set the spacing between items.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the horizontal alignment of items.
    pub fn alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.alignment = alignment;
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

    /// Build and return the virtual list element.
    pub fn build(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let item_height = self.item_height;
        let item_sizes = Rc::new(vec![
            size(px(10000.0), px(item_height + self.spacing));
            self.item_count
        ]);
        let render_fn = self.render_fn;
        let _alignment = self.alignment;

        gpui_component::v_virtual_list(
            self.entity,
            self.element_id,
            item_sizes,
            move |view, visible_range, window, cx| {
                visible_range
                    .map(|ix| {
                        if let Some(ref render) = render_fn {
                            render(view, ix, window, cx)
                        } else {
                            gpui::div().into_any_element()
                        }
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle)
    }
}

impl<V: Render + 'static> Modifier for LazyVStack<V> {}

/// A horizontally scrolling container that only renders visible items.
///
/// Use this for large horizontal lists where rendering all items would be inefficient.
/// Items are rendered lazily as they come into view.
pub struct LazyHStack<V: Render + 'static> {
    entity: Entity<V>,
    element_id: &'static str,
    scroll_handle: VirtualListScrollHandle,
    item_count: usize,
    item_width: f32,
    spacing: f32,
    render_fn: Option<LazyRenderFn<V>>,
}

impl<V: Render + 'static> LazyHStack<V> {
    /// Create a new lazy horizontal stack.
    ///
    /// # Arguments
    ///
    /// * `entity` - The GPUI entity for this view (`cx.entity().clone()`)
    /// * `element_id` - A unique identifier for this list
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
            item_count: 0,
            item_width: 100.0,
            spacing: 0.0,
            render_fn: None,
        }
    }

    /// Set the number of items in the list.
    pub fn item_count(mut self, count: usize) -> Self {
        self.item_count = count;
        self
    }

    /// Set the width of each item in pixels.
    pub fn item_width(mut self, width: f32) -> Self {
        self.item_width = width;
        self
    }

    /// Set the spacing between items.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
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

    /// Build and return the virtual list element.
    pub fn build(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let item_width = self.item_width;
        let item_sizes = Rc::new(vec![
            size(px(item_width + self.spacing), px(10000.0));
            self.item_count
        ]);
        let render_fn = self.render_fn;

        gpui_component::h_virtual_list(
            self.entity,
            self.element_id,
            item_sizes,
            move |view, visible_range, window, cx| {
                visible_range
                    .map(|ix| {
                        if let Some(ref render) = render_fn {
                            render(view, ix, window, cx)
                        } else {
                            gpui::div().into_any_element()
                        }
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle)
    }
}

impl<V: Render + 'static> Modifier for LazyHStack<V> {}

/// Helper function to calculate item sizes for variable-height lists.
pub fn calculate_item_sizes<F>(count: usize, size_fn: F) -> Rc<Vec<Size<Pixels>>>
where
    F: Fn(usize) -> Size<Pixels>,
{
    Rc::new((0..count).map(size_fn).collect())
}

/// Creates a uniform size for all items.
pub fn uniform_size(width: f32, height: f32) -> impl Fn(usize) -> Size<Pixels> {
    move |_| size(px(width), px(height))
}
