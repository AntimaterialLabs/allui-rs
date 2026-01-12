//! HStack story.
//!
//! Demonstrates horizontal stack layout with spacing and alignment options.
//!
//! ```rust,ignore
//! HStack::new()
//!     .spacing(12.0)
//!     .alignment(VerticalAlignment::Top)
//!     .child(Text::new("Left"))
//!     .child(Text::new("Right"))
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_hstack_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("HStack centers children vertically by default:"))
        .child(
            HStack::new()
                .spacing(8.0)
                .child(Text::new("A").padding(8.0).background(Color::blue()))
                .child(
                    Text::new("Tall")
                        .padding_edges(8.0, 8.0, 32.0, 8.0)
                        .background(Color::blue()),
                )
                .child(Text::new("C").padding(8.0).background(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("HStack with top alignment:"))
        .child(
            HStack::new()
                .spacing(8.0)
                .alignment(VerticalAlignment::Top)
                .child(Text::new("Top").padding(8.0).background(Color::green()))
                .child(
                    Text::new("Aligned")
                        .padding_edges(8.0, 8.0, 32.0, 8.0)
                        .background(Color::green()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
