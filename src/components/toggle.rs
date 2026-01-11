//! Toggle - Boolean switch component.
//!
//! A SwiftUI-style toggle that wraps gpui-component's Switch.

use gpui::{App, IntoElement, RenderOnce, SharedString, Window};
use gpui_component::Disableable;
use gpui_component::switch::Switch;

use crate::modifier::Modifier;

/// Handler type for toggle changes with GPUI context access.
pub type ToggleHandler = Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

/// A control for toggling between on and off states.
///
/// # Example
///
/// ```rust,ignore
/// // Simple callback (no state update)
/// Toggle::new("Dark Mode", is_dark_mode, |new_value| {
///     println!("Changed to: {}", new_value);
/// })
///
/// // With GPUI context for state updates
/// Toggle::new_with_handler("Dark Mode", is_dark_mode,
///     cx.listener(|view, checked: &bool, _window, cx| {
///         view.is_dark_mode = *checked;
///         cx.notify();
///     })
/// )
/// ```
#[derive(IntoElement)]
pub struct Toggle {
    id: SharedString,
    label: Option<SharedString>,
    is_on: bool,
    on_change: Option<ToggleHandler>,
    disabled: bool,
}

impl Toggle {
    /// Create a new toggle with a label, initial state, and simple change handler.
    ///
    /// Note: This handler cannot update GPUI state. Use `new_with_handler` for state updates.
    pub fn new(
        label: impl Into<SharedString>,
        is_on: bool,
        on_change: impl Fn(bool) + 'static,
    ) -> Self {
        let label_str: SharedString = label.into();
        Self {
            id: label_str.clone(),
            label: Some(label_str),
            is_on,
            on_change: Some(Box::new(move |checked: &bool, _window, _cx| {
                on_change(*checked);
            })),
            disabled: false,
        }
    }

    /// Create a new toggle with a GPUI-compatible handler for state updates.
    ///
    /// Use `cx.listener()` to create the handler:
    /// ```rust,ignore
    /// Toggle::new_with_handler("Label", value,
    ///     cx.listener(|view, checked: &bool, _window, cx| {
    ///         view.value = *checked;
    ///         cx.notify();
    ///     })
    /// )
    /// ```
    pub fn new_with_handler(
        label: impl Into<SharedString>,
        is_on: bool,
        on_change: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        let label_str: SharedString = label.into();
        Self {
            id: label_str.clone(),
            label: Some(label_str),
            is_on,
            on_change: Some(Box::new(on_change)),
            disabled: false,
        }
    }

    /// Create a toggle without a label.
    pub fn unlabeled(id: impl Into<SharedString>, is_on: bool) -> Self {
        Self {
            id: id.into(),
            label: None,
            is_on,
            on_change: None,
            disabled: false,
        }
    }

    /// Set the change handler (simple, no GPUI context).
    pub fn on_change(mut self, handler: impl Fn(bool) + 'static) -> Self {
        self.on_change = Some(Box::new(move |checked: &bool, _window, _cx| {
            handler(*checked);
        }));
        self
    }

    /// Set the change handler with GPUI context access.
    pub fn on_change_with(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Set the label text.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Disable the toggle.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Modifier for Toggle {}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut switch = Switch::new(self.id).checked(self.is_on);

        if let Some(label) = self.label {
            switch = switch.label(label);
        }

        if self.disabled {
            switch = switch.disabled(true);
        }

        if let Some(on_change) = self.on_change {
            switch = switch.on_click(move |checked: &bool, window, cx| {
                on_change(checked, window, cx);
            });
        }

        switch
    }
}
