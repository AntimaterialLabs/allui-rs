//! ForEach story.
//!
//! Demonstrates iterating over collections to generate views.
//!
//! ```rust,ignore
//! let items = vec!["Apple", "Banana", "Cherry"];
//! VStack::new().children(ForEach::new(items, |item| {
//!     Text::new(*item)
//! }))
//! ```

use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb};

pub fn render_foreach_story() -> impl IntoElement {
    let fruits = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    let numbers = vec![1, 2, 3, 4, 5];

    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("ForEach - Iterate over collections:"))
        .child(
            HStack::new()
                .spacing(32.0)
                .alignment(VerticalAlignment::Top)
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .alignment(HorizontalAlignment::Leading)
                        .child(Text::new("String list:").foreground_color(Color::gray()))
                        .child(
                            VStack::new()
                                .spacing(4.0)
                                .alignment(HorizontalAlignment::Leading)
                                .children(ForEach::new(fruits, |fruit| {
                                    HStack::new()
                                        .spacing(8.0)
                                        .child(div().size(px(8.0)).bg(rgb(0x34C759)).rounded_full())
                                        .child(Text::new(*fruit))
                                }))
                                .padding(16.0)
                                .background(Color::tertiary_system_background())
                                .corner_radius(8.0),
                        ),
                )
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .alignment(HorizontalAlignment::Leading)
                        .child(Text::new("Number grid:").foreground_color(Color::gray()))
                        .child(
                            HStack::new()
                                .spacing(8.0)
                                .children(ForEach::new(numbers, |num| {
                                    Text::new(format!("{}", num))
                                        .padding(16.0)
                                        .background(Color::blue())
                                        .corner_radius(8.0)
                                }))
                                .padding(16.0)
                                .background(Color::tertiary_system_background())
                                .corner_radius(8.0),
                        ),
                ),
        )
        .child(
            Text::new("ForEach works with VStack, HStack, List, etc.")
                .foreground_color(Color::gray()),
        )
}
