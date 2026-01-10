//! Layout primitives for Allui.
//!
//! These components are built directly on GPUI for precise SwiftUI layout control.
//!
//! # Stack Layouts
//!
//! Allui layout primitives match SwiftUI's behavior:
//!
//! - **VStack**: Arranges children vertically, centers horizontally by default
//! - **HStack**: Arranges children horizontally, centers vertically by default
//! - **ZStack**: Overlays children, centers in both axes by default
//! - **Spacer**: Expands to fill available space along the stack axis
//! - **EmptyView**: Renders nothing, takes no space
//! - **Group**: Transparent grouping, no layout effect
//!
//! # Grid Layouts
//!
//! - **Grid**: Static 2D table layout with auto-sizing columns
//! - **GridRow**: A row within a Grid
//! - **LazyVGrid**: Vertically-scrolling grid with fixed columns
//! - **LazyHGrid**: Horizontally-scrolling grid with fixed rows
//!
//! # Container Components
//!
//! - **ScrollView**: Scrollable container (vertical, horizontal, or both)
//! - **List**: Styled list container with iOS-style appearance
//! - **Section**: Grouping within List with optional header/footer
//!
//! # Control Flow
//!
//! - **ForEach**: Iterate over a collection and render views
//! - **If**: Conditional rendering based on a boolean
//! - **IfLet**: Render content if an Option is Some
//!
//! # Virtualized Layouts
//!
//! For large datasets, use virtualized stacks that only render visible items:
//!
//! - **LazyVStack**: Virtualized vertical stack
//! - **LazyHStack**: Virtualized horizontal stack

#[macro_use]
mod children_macro;
mod control_flow;
mod empty_view;
mod grid;
mod grid_item;
mod group;
mod hstack;
mod lazy_hgrid;
mod lazy_stack;
mod lazy_vgrid;
mod list;
mod scroll_view;
mod spacer;
mod vstack;
mod zstack;

pub use crate::alignment::{Alignment, HorizontalAlignment, VerticalAlignment};
pub use control_flow::{ForEach, If, IfLet};
pub use empty_view::EmptyView;
pub use grid::{Grid, GridRow};
pub use grid_item::{GridItem, GridItemSize};
pub use group::Group;
pub use hstack::HStack;
pub use lazy_hgrid::LazyHGrid;
pub use lazy_stack::{
    calculate_item_sizes, uniform_size, LazyHStack, LazyVStack, VirtualListScrollHandle,
};
pub use lazy_vgrid::LazyVGrid;
pub use list::{List, ListStyle, Section};
pub use scroll_view::{ScrollAxes, ScrollView};
pub use spacer::Spacer;
pub use vstack::VStack;
pub use zstack::ZStack;
