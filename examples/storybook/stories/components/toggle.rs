//! Toggle story.
//!
//! Demonstrates boolean switch with label and state handling.
//!
//! ```rust,ignore
//! Toggle::new_with_handler("Dark Mode", is_enabled,
//!     cx.listener(|this, checked: &bool, _, cx| {
//!         this.is_enabled = *checked;
//!         cx.notify();
//!     })
//! )
//! ```

use allui::prelude::*;
use gpui::{prelude::*, Context};

use crate::Storybook;

pub fn render_toggle_story(storybook: &Storybook, cx: &mut Context<Storybook>) -> impl IntoElement {
    let toggle_value = storybook.toggle_value;

    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Toggle - Boolean switch component:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .child(
                    HStack::new()
                        .spacing(12.0)
                        .child(Toggle::new_with_handler(
                            "Dark Mode",
                            toggle_value,
                            cx.listener(|this: &mut Storybook, checked: &bool, _window, cx| {
                                this.toggle_value = *checked;
                                cx.notify();
                            }),
                        ))
                        .child(
                            Text::new(if toggle_value { "ON" } else { "OFF" }).foreground_color(
                                if toggle_value {
                                    Color::green()
                                } else {
                                    Color::gray()
                                },
                            ),
                        ),
                )
                .child(Toggle::new("Disabled Toggle", true, |_| {}).disabled(true))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Note: Toggle wraps gpui-component's Switch.")
                .foreground_color(Color::gray()),
        )
}
