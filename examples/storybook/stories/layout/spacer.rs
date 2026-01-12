//! Spacer story.
//!
//! Demonstrates flexible space that expands to fill available space in stacks.
//!
//! ```rust,ignore
//! HStack::new()
//!     .child(Text::new("Left"))
//!     .child(Spacer::new())
//!     .child(Text::new("Right"))
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_spacer_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Spacer pushes content to edges:"))
        .child(
            HStack::new()
                .child(Text::new("Left").padding(8.0).background(Color::blue()))
                .child(Spacer::new())
                .child(Text::new("Right").padding(8.0).background(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0)
                .frame_width(400.0),
        )
        .child(Text::new("Multiple spacers distribute evenly:"))
        .child(
            HStack::new()
                .child(Spacer::new())
                .child(Text::new("Center").padding(8.0).background(Color::green()))
                .child(Spacer::new())
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0)
                .frame_width(400.0),
        )
}
