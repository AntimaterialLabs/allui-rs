use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb};

pub fn render_vstack_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new(
            "VStack centers children horizontally by default:",
        ))
        .child(
            VStack::new()
                .spacing(8.0)
                .child(Text::new("Child 1").padding(8.0).background(Color::blue()))
                .child(
                    Text::new("Child 2 (longer)")
                        .padding(8.0)
                        .background(Color::blue()),
                )
                .child(Text::new("3").padding(8.0).background(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("VStack with leading alignment:"))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    Text::new("Leading 1")
                        .padding(8.0)
                        .background(Color::green()),
                )
                .child(
                    Text::new("Leading 2 (longer)")
                        .padding(8.0)
                        .background(Color::green()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}

pub fn render_hstack_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("HStack centers children vertically by default:"))
        .child(
            HStack::new()
                .spacing(8.0)
                .child(Text::new("A").padding(8.0).background(Color::blue()))
                .child(
                    Text::new("Tall")
                        .padding_edges(8.0, 8.0, 32.0, 8.0)
                        .background(Color::blue()),
                )
                .child(Text::new("C").padding(8.0).background(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("HStack with top alignment:"))
        .child(
            HStack::new()
                .spacing(8.0)
                .alignment(VerticalAlignment::Top)
                .child(Text::new("Top").padding(8.0).background(Color::green()))
                .child(
                    Text::new("Aligned")
                        .padding_edges(8.0, 8.0, 32.0, 8.0)
                        .background(Color::green()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}

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

pub fn render_spacer_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Spacer pushes content to edges:"))
        .child(
            HStack::new()
                .child(Text::new("Left").padding(8.0).background(Color::blue()))
                .child(Spacer::new())
                .child(Text::new("Right").padding(8.0).background(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0)
                .frame_width(400.0),
        )
        .child(Text::new("Multiple spacers distribute evenly:"))
        .child(
            HStack::new()
                .child(Spacer::new())
                .child(Text::new("Center").padding(8.0).background(Color::green()))
                .child(Spacer::new())
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0)
                .frame_width(400.0),
        )
}
