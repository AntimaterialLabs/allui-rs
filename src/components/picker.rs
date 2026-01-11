//! Picker - Selection component.
//!
//! A SwiftUI-style picker that wraps gpui-component's Select.
//!
//! # Usage
//!
//! Picker requires state management via `Entity<SelectState>`. Create the state
//! in your view's constructor and pass it to Picker.
//!
//! ```rust,ignore
//! struct SettingsView {
//!     // Vec<&'static str> is the delegate type
//!     theme_picker: Entity<SelectState<Vec<&'static str>>>,
//! }
//!
//! impl SettingsView {
//!     fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
//!         let theme_picker = cx.new(|cx|
//!             SelectState::new(
//!                 vec!["Light", "Dark", "System"],
//!                 Some(IndexPath::default()),  // Select first item
//!                 window,
//!                 cx,
//!             )
//!         );
//!         Self { theme_picker }
//!     }
//! }
//!
//! // In render:
//! Picker::new(&self.theme_picker)
//!     .placeholder("Select theme...")
//! ```

use gpui::{px, App, Entity, IntoElement, Pixels, RenderOnce, SharedString, Styled, Window};
use gpui_component::select::{Select, SelectDelegate, SelectState};

// Re-export types for users
pub use gpui_component::select::SearchableVec;
pub use gpui_component::select::SelectDelegate as PickerDelegate;
pub use gpui_component::select::SelectEvent as PickerEvent;
pub use gpui_component::select::SelectGroup as PickerGroup;
pub use gpui_component::select::SelectItem as PickerItem;
pub use gpui_component::select::SelectState as PickerState;
pub use gpui_component::IndexPath;

use crate::modifier::Modifier;

/// A control for picking from a list of options.
///
/// This component wraps gpui-component's Select (dropdown).
///
/// The generic parameter `D` is the delegate type (e.g., `Vec<&'static str>`,
/// `SearchableVec<String>`, etc.) that provides the list of options.
///
/// # Example
///
/// ```rust,ignore
/// // Create state in your view with Vec as the delegate
/// let country: Entity<SelectState<Vec<&'static str>>> = cx.new(|cx|
///     SelectState::new(
///         vec!["USA", "Canada", "UK", "Germany"],
///         None,  // No initial selection
///         window,
///         cx,
///     )
/// );
///
/// // Subscribe to changes
/// cx.subscribe(&country, |this, _, event: &SelectEvent<_>, cx| {
///     if let SelectEvent::Confirm(value) = event {
///         this.on_country_changed(value);
///     }
/// });
///
/// // Use in render
/// Picker::new(&country)
///     .placeholder("Select country...")
/// ```
#[derive(IntoElement)]
pub struct Picker<D: SelectDelegate + 'static> {
    state: Entity<SelectState<D>>,
    placeholder: Option<SharedString>,
    cleanable: bool,
    width: Option<Pixels>,
    disabled: bool,
}

impl<D: SelectDelegate + 'static> Picker<D> {
    /// Create a new picker with the given state.
    pub fn new(state: &Entity<SelectState<D>>) -> Self {
        Self {
            state: state.clone(),
            placeholder: None,
            cleanable: false,
            width: None,
            disabled: false,
        }
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Show a clear button when an item is selected.
    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.cleanable = cleanable;
        self
    }

    /// Set the width of the picker.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(px(width));
        self
    }

    /// Disable the picker.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<D: SelectDelegate + 'static> Modifier for Picker<D> {}

impl<D: SelectDelegate + 'static> RenderOnce for Picker<D> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut select = Select::new(&self.state);

        if let Some(placeholder) = self.placeholder {
            select = select.placeholder(placeholder);
        }

        if self.cleanable {
            select = select.cleanable(true);
        }

        if let Some(width) = self.width {
            select = select.w(width);
        }

        if self.disabled {
            select = select.disabled(true);
        }

        select
    }
}
