//! Tap gesture story.
//!
//! Demonstrates adding tap handlers to any view using on_tap_gesture.
//!
//! ```rust,ignore
//! Text::new("Tap me!")
//!     .on_tap_gesture_with("unique-id", move |_event, _window, cx| {
//!         entity.update(cx, |this, cx| {
//!             this.count += 1;
//!             cx.notify();
//!         });
//!     })
//! ```

use allui::prelude::*;
use gpui::{prelude::*, Context};

use crate::Storybook;

pub fn render_tap_gesture_story(
    storybook: &Storybook,
    cx: &mut Context<Storybook>,
) -> impl IntoElement {
    let tap_count = storybook.tap_count;
    let entity = cx.entity().clone();

    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("on_tap_gesture - Add tap handlers to any view:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .child(
                    Text::new("Tap me!")
                        .padding(16.0)
                        .background(Color::blue())
                        .corner_radius(8.0)
                        .on_tap_gesture_with("tap-me-button", move |_event, _window, cx| {
                            entity.update(cx, |this, cx| {
                                this.tap_count += 1;
                                cx.notify();
                            });
                        }),
                )
                .child(
                    Text::new(format!("Tap count: {}", tap_count)).foreground_color(Color::green()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Any view can be made tappable with .on_tap_gesture()")
                .foreground_color(Color::gray()),
        )
}
