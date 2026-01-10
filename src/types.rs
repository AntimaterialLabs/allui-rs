//! Common type definitions shared across Allui.
//!
//! This module contains type aliases for complex types to improve code readability
//! and satisfy clippy's type_complexity lint.

use gpui::{App, ClickEvent, Window};

/// A boxed click event handler that can be stored in structs.
///
/// This type is used for components that need to store click handlers,
/// like Button, Link, and Tappable.
///
/// # Example
///
/// ```rust,ignore
/// struct MyComponent {
///     on_click: Option<ClickHandler>,
/// }
/// ```
pub type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
