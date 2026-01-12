//! VStack story.
//!
//! Demonstrates vertical stack layout with spacing and alignment options.
//!
//! ```rust,ignore
//! VStack::new()
//!     .spacing(8.0)
//!     .alignment(HorizontalAlignment::Leading)
//!     .child(Text::new("First"))
//!     .child(Text::new("Second"))
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_vstack_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new(
            "VStack centers children horizontally by default:",
        ))
        .child(
            VStack::new()
                .spacing(8.0)
                .child(Text::new("Child 1").padding(8.0).background(Color::blue()))
                .child(
                    Text::new("Child 2 (longer)")
                        .padding(8.0)
                        .background(Color::blue()),
                )
                .child(Text::new("3").padding(8.0).background(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("VStack with leading alignment:"))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    Text::new("Leading 1")
                        .padding(8.0)
                        .background(Color::green()),
                )
                .child(
                    Text::new("Leading 2 (longer)")
                        .padding(8.0)
                        .background(Color::green()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
