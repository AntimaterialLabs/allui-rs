//! LazyHGrid story.
//!
//! Demonstrates horizontally-scrolling grid with fixed rows.
//!
//! ```rust,ignore
//! LazyHGrid::new(cx.entity().clone(), "id", &scroll_handle)
//!     .rows(vec![GridItem::fixed(90.0); 2])
//!     .spacing(12.0)
//!     .item_count(30)
//!     .render_item(|view, idx, _, _| { ... })
//!     .build(window, cx)
//! ```

use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb};

pub fn render_lazy_hgrid_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new(
            "LazyHGrid - Horizontally-scrolling grid with fixed rows:",
        ))
        .child(
            VStack::new()
                .spacing(8.0)
                .child(
                    Text::new("Category carousel mockup (2 rows):").foreground_color(Color::gray()),
                )
                .child(
                    ScrollView::new("hgrid-demo")
                        .axes(ScrollAxes::horizontal())
                        .child(HStack::new().spacing(12.0).children((0..8).map(|col| {
                            VStack::new().spacing(12.0).children((0..2).map(move |row| {
                                let index = col * 2 + row;
                                let colors =
                                    [0x007AFF, 0x34C759, 0xFF9500, 0xFF3B30, 0xAF52DE, 0x5856D6];
                                let color = colors[index % colors.len()];
                                let categories =
                                    ["Music", "Movies", "Books", "Games", "Apps", "Podcasts"];
                                VStack::new()
                                    .child(
                                        div()
                                            .w(px(70.0))
                                            .h(px(50.0))
                                            .bg(rgb(color as u32))
                                            .rounded(px(8.0)),
                                    )
                                    .child(
                                        Text::new(categories[index % categories.len()])
                                            .font(Font::caption())
                                            .foreground_color(Color::gray()),
                                    )
                                    .spacing(4.0)
                            }))
                        })))
                        .frame(Frame::size(400.0, 200.0))
                        .background(Color::tertiary_system_background())
                        .corner_radius(8.0),
                )
                .child(
                    Text::new("Items flow top-to-bottom, left-to-right. Scroll horizontally.")
                        .foreground_color(Color::gray()),
                )
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .child(Text::new("Usage:").bold())
                        .child(
                            Text::new(
                                "LazyHGrid::new(cx.entity().clone(), \"id\", &scroll_handle)",
                            )
                            .font(Font::caption())
                            .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("    .rows(vec![GridItem::fixed(90.0); 2])")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("    .spacing(12.0).item_count(30)")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("    .render_item(|view, idx, _, _| { ... })")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("    .build(window, cx)")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .padding(12.0)
                        .background(Color::secondary_system_background())
                        .corner_radius(8.0),
                ),
        )
}
