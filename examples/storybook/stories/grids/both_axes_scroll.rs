//! Both axes scroll story.
//!
//! Demonstrates pannable 2D grid using ScrollView with both axes enabled.
//!
//! ```rust,ignore
//! div()
//!     .id("pannable")
//!     .overflow_scroll()  // Enable both axes
//!     .w(px(400.0)).h(px(300.0))
//!     .child(large_content)
//! ```

use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb, Hsla};

pub fn render_both_axes_scroll_story(secondary_bg: Hsla) -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("ScrollView with both axes - Pannable 2D grid:"))
        .child(
            VStack::new()
                .spacing(8.0)
                .child(
                    Text::new("20x20 coordinate grid (scroll/pan in any direction):")
                        .foreground_color(Color::gray()),
                )
                .child(
                    div()
                        .id("both-axes-scroll")
                        .overflow_scroll()
                        .w(px(400.0))
                        .h(px(300.0))
                        .bg(secondary_bg)
                        .rounded(px(8.0))
                        .child(
                            div()
                                .w(px(956.0))
                                .h(px(956.0))
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .children((0..20).map(|row| {
                                    div().flex().flex_row().gap(px(4.0)).children((0..20).map(
                                        move |col| {
                                            let is_origin = row == 0 && col == 0;
                                            let is_edge = row == 0 || col == 0;
                                            div()
                                                .size(px(44.0))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded(px(4.0))
                                                .bg(rgb(if is_origin {
                                                    0xFF3B30
                                                } else if is_edge {
                                                    0x555555
                                                } else {
                                                    0x333333
                                                }))
                                                .text_color(rgb(if is_edge {
                                                    0xFFFFFF
                                                } else {
                                                    0x888888
                                                }))
                                                .text_xs()
                                                .child(format!("{},{}", col, row))
                                        },
                                    ))
                                })),
                        ),
                )
                .child(
                    Text::new("Red = origin (0,0). Gray headers show row/column indices.")
                        .foreground_color(Color::gray()),
                ),
        )
}
