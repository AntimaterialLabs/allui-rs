//! ZStack story.
//!
//! Demonstrates overlay/layered layout where children are stacked on top of each other.
//!
//! ```rust,ignore
//! ZStack::new()
//!     .child(background_layer)
//!     .child(foreground_layer)
//! ```

use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb};

pub fn render_zstack_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("ZStack overlays children, centered by default:"))
        .child(
            ZStack::new()
                .child(div().size(px(150.0)).bg(rgb(0x007AFF)).rounded(px(8.0)))
                .child(div().size(px(80.0)).bg(rgb(0xFF3B30)).rounded_full())
                .frame_size(200.0, 200.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
