//! Button - Interactive button component.

use gpui::{
    App, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px, rgb,
};

use crate::modifier::Modifier;
use crate::types::ClickHandler;

/// The visual style of a button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Automatic style based on context.
    #[default]
    Automatic,
    /// Plain button with no chrome.
    Plain,
    /// Button with a visible border.
    Bordered,
    /// Button with a filled background.
    BorderedProminent,
    /// Button with minimal styling.
    Borderless,
}

/// A control that initiates an action.
///
/// # Usage Patterns
///
/// **Simple buttons** - use `new()` for the common case:
/// ```rust,ignore
/// Button::new("Submit", || println!("Submitted!"))
///     .button_style(ButtonStyle::BorderedProminent)
/// ```
///
/// **Complex buttons** - use `with_id()` builder for custom IDs or deferred actions:
/// ```rust,ignore
/// Button::with_id("submit-btn")
///     .label("Submit")
///     .on_click(|| println!("Submitted!"))
///     .button_style(ButtonStyle::BorderedProminent)
/// ```
///
/// **With GPUI listener** - for access to view state:
/// ```rust,ignore
/// Button::new("Increment", cx.listener(|this, _, _, cx| {
///     this.count += 1;
///     cx.notify();
/// }))
/// ```
#[derive(IntoElement)]
pub struct Button {
    id: SharedString,
    label: SharedString,
    action: Option<ClickHandler>,
    style: ButtonStyle,
    disabled: bool,
}

impl Button {
    /// Create a new button with a label and action.
    pub fn new(label: impl Into<SharedString>, action: impl Fn() + 'static) -> Self {
        let label_str: SharedString = label.into();
        Self {
            id: label_str.clone(),
            label: label_str,
            action: Some(Box::new(move |_, _, _| action())),
            style: ButtonStyle::default(),
            disabled: false,
        }
    }

    /// Create a button with a custom ID.
    pub fn with_id(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            action: None,
            style: ButtonStyle::default(),
            disabled: false,
        }
    }

    /// Set the button label.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    /// Set the action to perform when clicked.
    pub fn on_click(mut self, action: impl Fn() + 'static) -> Self {
        self.action = Some(Box::new(move |_, _, _| action()));
        self
    }

    /// Set the button style.
    pub fn button_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Disable the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Modifier for Button {}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = gpui::ElementId::Name(self.id.clone());

        let mut button = div().id(id).cursor_pointer().px(px(12.0)).py(px(6.0));

        // Apply style
        button = match self.style {
            ButtonStyle::Automatic | ButtonStyle::Bordered => button
                .border_1()
                .border_color(rgb(0x888888))
                .rounded(px(6.0)),
            ButtonStyle::BorderedProminent => button
                .bg(rgb(0x007AFF))
                .text_color(rgb(0xFFFFFF))
                .rounded(px(6.0)),
            ButtonStyle::Plain => button,
            ButtonStyle::Borderless => button,
        };

        // Add label
        button = button.child(self.label);

        // Add click handler
        if let Some(action) = self.action {
            if !self.disabled {
                button = button.on_click(move |event, window, cx| {
                    action(event, window, cx);
                });
            }
        }

        // Apply disabled state
        if self.disabled {
            button = button.opacity(0.5).cursor_default();
        }

        button
    }
}
