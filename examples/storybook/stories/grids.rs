use gpui::{div, prelude::*, px, rgb, Hsla};
use allui::prelude::*;

pub fn render_grid_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Grid - Static 2D table layout:"))
        .child(
            VStack::new()
                .spacing(16.0)
                .child(Text::new("Data table with GridRow:").foreground_color(Color::gray()))
                .child(
                    Grid::new()
                        .horizontal_spacing(16.0)
                        .vertical_spacing(8.0)
                        .child(
                            GridRow::new()
                                .child(
                                    Text::new("Name")
                                        .bold()
                                        .foreground_color(Color::secondary_label()),
                                )
                                .child(
                                    Text::new("Type")
                                        .bold()
                                        .foreground_color(Color::secondary_label()),
                                )
                                .child(
                                    Text::new("Size")
                                        .bold()
                                        .foreground_color(Color::secondary_label()),
                                ),
                        )
                        .child(
                            GridRow::new()
                                .child(Text::new("main.rs"))
                                .child(Text::new("Rust"))
                                .child(Text::new("4.2 KB")),
                        )
                        .child(
                            GridRow::new()
                                .child(Text::new("Cargo.toml"))
                                .child(Text::new("TOML"))
                                .child(Text::new("1.1 KB")),
                        )
                        .child(
                            GridRow::new()
                                .child(Text::new("README.md"))
                                .child(Text::new("Markdown"))
                                .child(Text::new("8.7 KB")),
                        )
                        .child(
                            GridRow::new()
                                .child(Text::new("lib.rs"))
                                .child(Text::new("Rust"))
                                .child(Text::new("2.3 KB")),
                        )
                        .padding(16.0)
                        .background(Color::tertiary_system_background())
                        .corner_radius(8.0),
                )
                .child(
                    Text::new("Columns auto-size based on content width.")
                        .foreground_color(Color::gray()),
                ),
        )
}

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
