//! Control flow components for conditional and iterative rendering.
//!
//! These components enable declarative control flow in Allui views,
//! similar to SwiftUI's `ForEach` and conditional `if` statements.

use gpui::{div, App, IntoElement, ParentElement, RenderOnce, Window};

use crate::layout::EmptyView;
use crate::modifier::Modifier;

/// Iterate over a collection and render a view for each item.
///
/// Unlike `LazyVStack`, ForEach renders all items immediately.
/// For large collections, consider using `LazyVStack` or `LazyHStack`.
///
/// # Example
///
/// ```rust,ignore
/// let items = vec!["Apple", "Banana", "Cherry"];
///
/// VStack::new()
///     .children(
///         ForEach::new(&items, |item| {
///             Text::new(*item)
///         })
///     )
/// ```
pub struct ForEach<T, F, V>
where
    F: Fn(&T) -> V,
    V: IntoElement,
{
    items: Vec<T>,
    view_builder: F,
}

impl<T, F, V> ForEach<T, F, V>
where
    F: Fn(&T) -> V,
    V: IntoElement,
{
    /// Create a new ForEach from a slice and a view builder function.
    pub fn new(items: impl IntoIterator<Item = T>, view_builder: F) -> Self {
        Self {
            items: items.into_iter().collect(),
            view_builder,
        }
    }
}

impl<T, F, V> IntoIterator for ForEach<T, F, V>
where
    F: Fn(&T) -> V,
    V: IntoElement,
{
    type Item = V;
    type IntoIter = ForEachIter<T, F, V>;

    fn into_iter(self) -> Self::IntoIter {
        ForEachIter {
            items: self.items.into_iter(),
            view_builder: self.view_builder,
        }
    }
}

/// Iterator for ForEach that yields views.
pub struct ForEachIter<T, F, V>
where
    F: Fn(&T) -> V,
    V: IntoElement,
{
    items: std::vec::IntoIter<T>,
    view_builder: F,
}

impl<T, F, V> Iterator for ForEachIter<T, F, V>
where
    F: Fn(&T) -> V,
    V: IntoElement,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next().map(|item| (self.view_builder)(&item))
    }
}

/// Conditional rendering based on a boolean condition.
///
/// # Example
///
/// ```rust,ignore
/// If::new(is_logged_in)
///     .then(|| ProfileView::new(user))
///     .otherwise(|| LoginView::new())
/// ```
pub struct If<T, E>
where
    T: IntoElement + 'static,
    E: IntoElement + 'static,
{
    condition: bool,
    then_view: Option<T>,
    else_view: Option<E>,
}

impl If<EmptyView, EmptyView> {
    /// Create a new conditional view.
    pub fn new(condition: bool) -> If<EmptyView, EmptyView> {
        If {
            condition,
            then_view: None,
            else_view: None,
        }
    }
}

impl<T: IntoElement + 'static, E: IntoElement + 'static> If<T, E> {
    /// Specify the view to render when the condition is true.
    pub fn then<NewT: IntoElement + 'static>(
        self,
        view_builder: impl FnOnce() -> NewT,
    ) -> If<NewT, E> {
        If {
            condition: self.condition,
            then_view: Some(view_builder()),
            else_view: self.else_view,
        }
    }

    /// Specify the view to render when the condition is false.
    pub fn otherwise<NewE: IntoElement + 'static>(
        self,
        view_builder: impl FnOnce() -> NewE,
    ) -> If<T, NewE> {
        If {
            condition: self.condition,
            then_view: self.then_view,
            else_view: Some(view_builder()),
        }
    }
}

impl<T: IntoElement + 'static, E: IntoElement + 'static> Modifier for If<T, E> {}

impl<T: IntoElement + 'static, E: IntoElement + 'static> IntoElement for If<T, E> {
    type Element = gpui::AnyElement;

    fn into_element(self) -> Self::Element {
        IfElement { inner: self }.into_any_element()
    }
}

#[derive(IntoElement)]
struct IfElement<T: IntoElement + 'static, E: IntoElement + 'static> {
    inner: If<T, E>,
}

impl<T: IntoElement + 'static, E: IntoElement + 'static> RenderOnce for IfElement<T, E> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if self.inner.condition {
            if let Some(view) = self.inner.then_view {
                div().child(view.into_any_element())
            } else {
                div()
            }
        } else if let Some(view) = self.inner.else_view {
            div().child(view.into_any_element())
        } else {
            div()
        }
    }
}

/// Optional view - renders the content if the Option is Some.
///
/// # Example
///
/// ```rust,ignore
/// IfLet::new(maybe_user, |user| {
///     Text::new(format!("Welcome, {}!", user.name))
/// })
/// ```
pub struct IfLet<T, F, V>
where
    T: 'static,
    F: FnOnce(&T) -> V + 'static,
    V: IntoElement + 'static,
{
    value: Option<T>,
    view_builder: Option<F>,
}

impl<T, F, V> IfLet<T, F, V>
where
    T: 'static,
    F: FnOnce(&T) -> V + 'static,
    V: IntoElement + 'static,
{
    /// Create a new optional view.
    pub fn new(value: Option<T>, view_builder: F) -> Self {
        Self {
            value,
            view_builder: Some(view_builder),
        }
    }
}

impl<T, F, V> Modifier for IfLet<T, F, V>
where
    T: 'static,
    F: FnOnce(&T) -> V + 'static,
    V: IntoElement + 'static,
{
}

impl<T: 'static, F: FnOnce(&T) -> V + 'static, V: IntoElement + 'static> IntoElement
    for IfLet<T, F, V>
{
    type Element = gpui::AnyElement;

    fn into_element(self) -> Self::Element {
        IfLetElement { inner: self }.into_any_element()
    }
}

#[derive(IntoElement)]
struct IfLetElement<T: 'static, F: FnOnce(&T) -> V + 'static, V: IntoElement + 'static> {
    inner: IfLet<T, F, V>,
}

impl<T: 'static, F: FnOnce(&T) -> V + 'static, V: IntoElement + 'static> RenderOnce
    for IfLetElement<T, F, V>
{
    fn render(mut self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if let (Some(value), Some(builder)) =
            (self.inner.value.as_ref(), self.inner.view_builder.take())
        {
            div().child(builder(value).into_any_element())
        } else {
            div()
        }
    }
}
