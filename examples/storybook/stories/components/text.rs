//! Text story.
//!
//! Demonstrates text rendering with fonts, colors, decorations, and line limits.
//!
//! ```rust,ignore
//! Text::new("Hello")
//!     .font(Font::headline())
//!     .foreground_color(Color::blue())
//!     .strikethrough(true)
//!     .line_limit(2)
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_text_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Text Styles").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Large Title").font(Font::large_title()))
                .child(Text::new("Title").font(Font::title()))
                .child(Text::new("Headline").font(Font::headline()))
                .child(Text::new("Body text").font(Font::body()))
                .child(
                    Text::new("Caption")
                        .font(Font::caption())
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Font Variations").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Bold text").bold())
                .child(Text::new("Italic text").italic())
                .child(Text::new("Bold + Italic").bold().italic())
                .child(Text::new("Colored text").foreground_color(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Font Design").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Default: The quick brown fox"))
                .child(
                    Text::new("Monospaced: let x = 42;")
                        .font(Font::body().monospaced()),
                )
                .child(
                    Text::new("Serif: Classical typography")
                        .font(Font::body().design(FontDesign::Serif)),
                )
                .child(
                    Text::new("Rounded: Friendly appearance")
                        .font(Font::body().design(FontDesign::Rounded)),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Text Decorations").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Normal text"))
                .child(Text::new("Strikethrough text").strikethrough(true))
                .child(
                    Text::new("Strikethrough + Red")
                        .strikethrough(true)
                        .foreground_color(Color::red()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Line Limit").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .alignment(HorizontalAlignment::Leading)
                        .child(Text::new(".line_limit(1):").foreground_color(Color::gray()))
                        .child(
                            Text::new("This is a very long text that should be truncated to a single line when line_limit is set to 1.")
                                .line_limit(1)
                                .frame_width(300.0)
                        ),
                )
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .alignment(HorizontalAlignment::Leading)
                        .child(Text::new(".line_limit(2):").foreground_color(Color::gray()))
                        .child(
                            Text::new("This is a very long text that demonstrates the line_limit modifier. It should wrap to two lines maximum and then truncate any remaining content.")
                                .line_limit(2)
                                .frame_width(300.0)
                        ),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
