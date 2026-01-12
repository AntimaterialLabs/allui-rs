//! ScrollView story.
//!
//! Demonstrates scrollable containers with vertical and horizontal axes.
//!
//! ```rust,ignore
//! ScrollView::new("my-scroll")
//!     .axes(ScrollAxes::vertical())
//!     .child(VStack::new().children(...))
//!     .frame(Frame::size(300.0, 200.0))
//! ```

use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb};

pub fn render_scrollview_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("ScrollView - Scrollable container for content:"))
        .child(
            VStack::new()
                .spacing(16.0)
                .child(Text::new("Vertical scroll (default):"))
                .child(
                    ScrollView::new("vertical-scroll")
                        .axes(ScrollAxes::vertical())
                        .child(VStack::new().spacing(8.0).children((1..=20).map(|i| {
                            Text::new(format!("Item {}", i))
                                .padding(12.0)
                                .background(Color::tertiary_system_background())
                                .corner_radius(4.0)
                        })))
                        .frame(Frame::size(300.0, 200.0))
                        .background(Color::tertiary_system_background())
                        .corner_radius(8.0),
                )
                .child(Text::new("Horizontal scroll:"))
                .child(
                    ScrollView::new("horizontal-scroll")
                        .axes(ScrollAxes::horizontal())
                        .child(HStack::new().spacing(8.0).children((1..=15).map(|i| {
                            VStack::new()
                                .child(div().size(px(60.0)).bg(rgb(0x007AFF)).rounded(px(8.0)))
                                .child(Text::new(format!("{}", i)))
                                .spacing(4.0)
                                .padding(8.0)
                        })))
                        .frame(Frame::size(400.0, 120.0))
                        .background(Color::tertiary_system_background())
                        .corner_radius(8.0),
                ),
        )
}
