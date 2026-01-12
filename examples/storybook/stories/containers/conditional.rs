//! Conditional rendering story.
//!
//! Demonstrates If and IfLet for conditional view rendering.
//!
//! ```rust,ignore
//! If::new(is_logged_in)
//!     .then(|| ProfileView::new())
//!     .otherwise(|| LoginView::new())
//!
//! IfLet::new(selected_item, |item| Text::new(item.name))
//! ```

use allui::prelude::*;
use gpui::{prelude::*, Context, SharedString};

use crate::Storybook;

pub fn render_conditional_story(
    storybook: &Storybook,
    cx: &mut Context<Storybook>,
) -> impl IntoElement {
    let show_content = storybook.show_content;
    let selected_fruit = storybook.selected_fruit;
    let entity = cx.entity().clone();
    let entity2 = cx.entity().clone();

    let fruits = ["Apple", "Banana", "Cherry"];

    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("If - Conditional rendering:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .child(HStack::new().spacing(12.0).child(Toggle::new_with_handler(
                    "Show Content",
                    show_content,
                    cx.listener(|this: &mut Storybook, checked: &bool, _window, cx| {
                        this.show_content = *checked;
                        cx.notify();
                    }),
                )))
                .child(
                    If::new(show_content)
                        .then(|| {
                            Text::new("Content is visible!")
                                .padding(16.0)
                                .background(Color::green())
                                .corner_radius(8.0)
                        })
                        .otherwise(|| {
                            Text::new("Content is hidden")
                                .padding(16.0)
                                .background(Color::red())
                                .corner_radius(8.0)
                        }),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("IfLet - Render when Option is Some:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .child(
                    HStack::new()
                        .spacing(8.0)
                        .children(fruits.iter().enumerate().map(|(idx, fruit)| {
                            let is_selected = selected_fruit == Some(idx);
                            let entity = entity.clone();
                            Text::new(*fruit)
                                .padding(8.0)
                                .background(if is_selected {
                                    Color::blue()
                                } else {
                                    Color::secondary()
                                })
                                .corner_radius(4.0)
                                .on_tap_gesture_with(
                                    SharedString::from(format!("fruit-{}", idx)),
                                    move |_, _, cx| {
                                        entity.update(cx, |this, cx| {
                                            this.selected_fruit = Some(idx);
                                            cx.notify();
                                        });
                                    },
                                )
                        }))
                        .child(
                            Text::new("None")
                                .padding(8.0)
                                .background(if selected_fruit.is_none() {
                                    Color::blue()
                                } else {
                                    Color::secondary()
                                })
                                .corner_radius(4.0)
                                .on_tap_gesture_with("fruit-none", move |_, _, cx| {
                                    entity2.update(cx, |this, cx| {
                                        this.selected_fruit = None;
                                        cx.notify();
                                    });
                                }),
                        ),
                )
                .child(IfLet::new(selected_fruit, move |idx| {
                    let fruit_names = ["Apple", "Banana", "Cherry"];
                    Text::new(format!("Selected: {} (index {})", fruit_names[*idx], idx))
                        .padding(12.0)
                        .background(Color::green())
                        .corner_radius(8.0)
                }))
                .child(
                    If::new(selected_fruit.is_none())
                        .then(|| {
                            Text::new("No fruit selected")
                                .padding(12.0)
                                .foreground_color(Color::gray())
                        })
                        .otherwise(EmptyView::new),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
