//! LazyVGrid story.
//!
//! Demonstrates vertically-scrolling grid with fixed columns.
//!
//! ```rust,ignore
//! LazyVGrid::new(cx.entity().clone(), "id", &scroll_handle)
//!     .columns(vec![GridItem::flexible(); 3])
//!     .spacing(8.0)
//!     .item_count(50)
//!     .render_item(|view, idx, _, _| { ... })
//!     .build(window, cx)
//! ```

use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb};

pub fn render_lazy_vgrid_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new(
            "LazyVGrid - Vertically-scrolling grid with fixed columns:",
        ))
        .child(
            VStack::new()
                .spacing(8.0)
                .child(
                    Text::new("Photo gallery mockup (3 columns):").foreground_color(Color::gray()),
                )
                .child(
                    ScrollView::new("vgrid-demo")
                        .axes(ScrollAxes::vertical())
                        .child(VStack::new().spacing(8.0).children((0..6).map(|row| {
                            HStack::new().spacing(8.0).children((0..3).map(move |col| {
                                let index = row * 3 + col;
                                let colors = [0x007AFF, 0x34C759, 0xFF9500, 0xFF3B30, 0xAF52DE];
                                let color = colors[index % colors.len()];
                                VStack::new()
                                    .child(
                                        div().size(px(80.0)).bg(rgb(color as u32)).rounded(px(8.0)),
                                    )
                                    .child(
                                        Text::new(format!("Photo {}", index + 1))
                                            .font(Font::caption())
                                            .foreground_color(Color::gray()),
                                    )
                                    .spacing(4.0)
                            }))
                        })))
                        .frame(Frame::size(320.0, 300.0))
                        .background(Color::tertiary_system_background())
                        .corner_radius(8.0),
                )
                .child(
                    Text::new("Items flow left-to-right, top-to-bottom. Scroll vertically.")
                        .foreground_color(Color::gray()),
                )
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .child(Text::new("Usage:").bold())
                        .child(
                            Text::new(
                                "LazyVGrid::new(cx.entity().clone(), \"id\", &scroll_handle)",
                            )
                            .font(Font::caption())
                            .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("    .columns(vec![GridItem::flexible(); 3])")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("    .spacing(8.0).item_count(50)")
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
