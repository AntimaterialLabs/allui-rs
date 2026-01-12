//! Grid story.
//!
//! Demonstrates static 2D table layout with GridRow and auto-sizing columns.
//!
//! ```rust,ignore
//! Grid::new()
//!     .horizontal_spacing(16.0)
//!     .vertical_spacing(8.0)
//!     .child(GridRow::new().child(Text::new("Name")).child(Text::new("Value")))
//! ```

use allui::prelude::*;
use gpui::prelude::*;

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
