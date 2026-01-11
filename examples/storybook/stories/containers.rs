use allui::prelude::*;
use gpui::{div, prelude::*, px, rgb, Context, SharedString};

use crate::Storybook;

pub fn render_scrollview_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("ScrollView - Scrollable container for content:"))
        .child(
            VStack::new()
                .spacing(16.0)
                .child(Text::new("Vertical scroll (default):"))
                .child(
                    ScrollView::new("vertical-scroll")
                        .axes(ScrollAxes::vertical())
                        .child(VStack::new().spacing(8.0).children((1..=20).map(|i| {
                            Text::new(format!("Item {}", i))
                                .padding(12.0)
                                .background(Color::tertiary_system_background())
                                .corner_radius(4.0)
                        })))
                        .frame(Frame::size(300.0, 200.0))
                        .background(Color::tertiary_system_background())
                        .corner_radius(8.0),
                )
                .child(Text::new("Horizontal scroll:"))
                .child(
                    ScrollView::new("horizontal-scroll")
                        .axes(ScrollAxes::horizontal())
                        .child(HStack::new().spacing(8.0).children((1..=15).map(|i| {
                            VStack::new()
                                .child(div().size(px(60.0)).bg(rgb(0x007AFF)).rounded(px(8.0)))
                                .child(Text::new(format!("{}", i)))
                                .spacing(4.0)
                                .padding(8.0)
                        })))
                        .frame(Frame::size(400.0, 120.0))
                        .background(Color::tertiary_system_background())
                        .corner_radius(8.0),
                ),
        )
}

pub fn render_list_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("List & Section - iOS-style grouped lists:"))
        .child(
            HStack::new()
                .spacing(24.0)
                .alignment(VerticalAlignment::Top)
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(Text::new("Inset Grouped (Dark):").foreground_color(Color::gray()))
                        .child(
                            List::new("settings-list")
                                .list_style(ListStyle::inset_grouped())
                                .child(
                                    Section::new()
                                        .header("Account")
                                        .child(Text::new("Profile"))
                                        .child(Text::new("Privacy"))
                                        .child(Text::new("Security")),
                                )
                                .child(
                                    Section::new()
                                        .header("Preferences")
                                        .footer("Customize your experience")
                                        .child(Text::new("Notifications"))
                                        .child(Text::new("Appearance"))
                                        .child(Text::new("Language")),
                                )
                                .frame(Frame::size(280.0, 380.0))
                                .background(Color::system_background())
                                .corner_radius(12.0),
                        ),
                )
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(Text::new("Plain (Dark):").foreground_color(Color::gray()))
                        .child(
                            List::new("plain-list")
                                .list_style(ListStyle::plain())
                                .child(
                                    Section::new()
                                        .child(Text::new("First Item"))
                                        .child(Text::new("Second Item"))
                                        .child(Text::new("Third Item")),
                                )
                                .frame(Frame::size(200.0, 200.0))
                                .background(Color::system_background())
                                .corner_radius(8.0),
                        ),
                ),
        )
}

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
                        .child(Text::new("String list:").foreground_color(Color::gray()))
                        .child(
                            VStack::new()
                                .spacing(4.0)
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
            Text::new("ForEach works with VStack.children(), HStack.children(), etc.")
                .foreground_color(Color::gray()),
        )
}

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
