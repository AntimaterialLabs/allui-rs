//! Macro for common child/children builder methods.

/// Implements `child()` and `children()` methods for container types.
///
/// The type must have a `children: Vec<gpui::AnyElement>` field.
///
/// # Example
///
/// ```rust,ignore
/// struct MyContainer {
///     children: Vec<gpui::AnyElement>,
/// }
///
/// impl MyContainer {
///     pub fn new() -> Self {
///         Self { children: Vec::new() }
///     }
///
///     impl_child_methods!();
/// }
/// ```
macro_rules! impl_child_methods {
    () => {
        /// Add a child view.
        pub fn child<E: gpui::IntoElement>(mut self, child: E) -> Self {
            self.children.push(child.into_any_element());
            self
        }

        /// Add multiple children.
        pub fn children<I, E>(mut self, children: I) -> Self
        where
            I: IntoIterator<Item = E>,
            E: gpui::IntoElement,
        {
            self.children
                .extend(children.into_iter().map(|c| c.into_any_element()));
            self
        }
    };
}
