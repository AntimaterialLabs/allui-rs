//! List configuration story.
//!
//! Demonstrates list row spacing, insets, section spacing, and per-row configuration.
//!
//! ```rust,ignore
//! List::new("id")
//!     .list_row_spacing(8.0)
//!     .list_row_insets(EdgeInsets::init(8.0, 32.0, 8.0, 32.0))
//!     .list_section_spacing(ListSectionSpacing::compact())
//!     .min_row_height(60.0)
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_list_config_story() -> impl IntoElement {
    use allui::prelude::{EdgeInsetsExt, ListSectionSpacing, RowConfiguration};

    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new(
            "List Configuration - Row/Section spacing and insets:",
        ))
        .child(
            HStack::new()
                .spacing(24.0)
                .alignment(VerticalAlignment::Top)
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(
                            Text::new("Custom Row Spacing (8pt):").foreground_color(Color::gray()),
                        )
                        .child(
                            List::new("spaced-list")
                                .list_style(ListStyle::inset_grouped())
                                .list_row_spacing(8.0)
                                .section(
                                    Section::new()
                                        .header("With Row Spacing")
                                        .row(Text::new("Row 1"))
                                        .row(Text::new("Row 2"))
                                        .row(Text::new("Row 3")),
                                )
                                .frame(Frame::size(220.0, 220.0))
                                .background(Color::system_background())
                                .corner_radius(12.0),
                        ),
                )
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(Text::new("Custom Row Insets:").foreground_color(Color::gray()))
                        .child(
                            List::new("inset-list")
                                .list_style(ListStyle::inset_grouped())
                                .list_row_insets(EdgeInsets::init(8.0, 32.0, 8.0, 32.0))
                                .section(
                                    Section::new()
                                        .header("Large Insets")
                                        .row(Text::new("Padded Row 1"))
                                        .row(Text::new("Padded Row 2")),
                                )
                                .frame(Frame::size(220.0, 200.0))
                                .background(Color::system_background())
                                .corner_radius(12.0),
                        ),
                )
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(
                            Text::new("Compact Section Spacing:").foreground_color(Color::gray()),
                        )
                        .child(
                            List::new("compact-list")
                                .list_style(ListStyle::inset_grouped())
                                .list_section_spacing(ListSectionSpacing::compact())
                                .section(
                                    Section::new().header("Section A").row(Text::new("Item 1")),
                                )
                                .section(
                                    Section::new().header("Section B").row(Text::new("Item 2")),
                                )
                                .section(
                                    Section::new().header("Section C").row(Text::new("Item 3")),
                                )
                                .frame(Frame::size(220.0, 280.0))
                                .background(Color::system_background())
                                .corner_radius(12.0),
                        ),
                ),
        )
        .child(Text::new("Per-row configuration override:"))
        .child(
            List::new("per-row-config")
                .list_style(ListStyle::inset_grouped())
                .section(
                    Section::new()
                        .header("Mixed Insets")
                        .row(Text::new("Default insets"))
                        .row_with_config(
                            Text::new("Extra indented (50pt leading)"),
                            RowConfiguration::new().insets(EdgeInsets::init(0.0, 50.0, 0.0, 16.0)),
                        )
                        .row(Text::new("Back to default")),
                )
                .frame(Frame::size(400.0, 180.0))
                .background(Color::system_background())
                .corner_radius(12.0),
        )
        .child(Text::new("Min row height (60pt):"))
        .child(
            List::new("tall-rows")
                .list_style(ListStyle::inset_grouped())
                .min_row_height(60.0)
                .section(
                    Section::new()
                        .header("Tall Rows")
                        .row(Text::new("Taller row 1"))
                        .row(Text::new("Taller row 2")),
                )
                .frame(Frame::size(300.0, 200.0))
                .background(Color::system_background())
                .corner_radius(12.0),
        )
}
