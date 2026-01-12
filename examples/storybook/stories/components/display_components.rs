//! Display components story.
//!
//! Demonstrates Divider, Label, Link, ProgressView, and Image.
//!
//! ```rust,ignore
//! Divider::new()
//! Label::new("star.fill", "Favorites")
//! Link::new("Click here", || println!("clicked"))
//! ProgressView::new().value(0.65).progress_view_style(ProgressViewStyle::Linear)
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_display_components_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Divider").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Item above"))
                .child(Divider::new())
                .child(Text::new("Item below"))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0)
                .frame_width(300.0),
        )
        .child(Text::new("Label").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Label::new("star.fill", "Favorites"))
                .child(Label::new("folder", "Documents"))
                .child(Label::new("gear", "Settings"))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Link").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Link::new("Visit Allui", || {
                    println!("Link clicked: Visit Allui");
                }))
                .child(Link::new("Documentation", || {
                    println!("Link clicked: Documentation");
                }))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("ProgressView").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Linear (determinate):").foreground_color(Color::gray()))
                .child(
                    ProgressView::new()
                        .progress_view_style(ProgressViewStyle::Linear)
                        .value(0.65)
                        .frame_width(200.0),
                )
                .child(Text::new("Circular (indeterminate):").foreground_color(Color::gray()))
                .child(ProgressView::new().progress_view_style(ProgressViewStyle::Circular))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Image").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    Text::new("Note: Image currently renders placeholder text.")
                        .foreground_color(Color::gray()),
                )
                .child(Image::new("photo.jpg").frame_size(100.0, 100.0))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
