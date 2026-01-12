//! Modifiers story.
//!
//! Demonstrates how modifier order affects rendering (padding, background, etc.).
//!
//! ```rust,ignore
//! // Order matters!
//! Text::new("A").padding(16.0).background(Color::red())  // padding inside
//! Text::new("B").background(Color::red()).padding(16.0)  // padding outside
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_modifiers_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Modifier order matters:"))
        .child(
            HStack::new()
                .spacing(32.0)
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(Text::new(".padding().background()"))
                        .child(Text::new("Hello").padding(16.0).background(Color::red())),
                )
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(Text::new(".background().padding()"))
                        .child(Text::new("Hello").background(Color::red()).padding(16.0)),
                ),
        )
        .child(Text::new("Chained modifiers:"))
        .child(
            Text::new("Styled Text")
                .padding(12.0)
                .background(Color::blue())
                .corner_radius(8.0)
                .padding(4.0)
                .background(Color::green())
                .corner_radius(12.0),
        )
}
