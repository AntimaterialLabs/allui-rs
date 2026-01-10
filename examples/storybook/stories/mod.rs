//! Story definitions and organization.
//!
//! Each story module contains render functions for a category of components.

mod components;
mod containers;
mod grids;
mod layout;

pub use components::*;
pub use containers::*;
pub use grids::*;
pub use layout::*;

/// All available stories in the storybook.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Story {
    #[default]
    VStack,
    HStack,
    ZStack,
    Spacer,
    Text,
    Button,
    Modifiers,
    Toggle,
    TapGesture,
    TextFields,
    Sliders,
    MoreInputs,
    DisplayComponents,
    ScrollView,
    List,
    ForEach,
    Conditional,
    Grid,
    LazyVGrid,
    LazyHGrid,
    BothAxesScroll,
}

/// Story metadata for sidebar organization.
pub struct StoryInfo {
    pub name: &'static str,
    pub story: Story,
}

/// Get all stories grouped by tier.
pub fn layout_stories() -> &'static [StoryInfo] {
    &[
        StoryInfo {
            name: "VStack",
            story: Story::VStack,
        },
        StoryInfo {
            name: "HStack",
            story: Story::HStack,
        },
        StoryInfo {
            name: "ZStack",
            story: Story::ZStack,
        },
        StoryInfo {
            name: "Spacer",
            story: Story::Spacer,
        },
    ]
}

pub fn component_stories() -> &'static [StoryInfo] {
    &[
        StoryInfo {
            name: "Text",
            story: Story::Text,
        },
        StoryInfo {
            name: "Button",
            story: Story::Button,
        },
        StoryInfo {
            name: "Modifiers",
            story: Story::Modifiers,
        },
        StoryInfo {
            name: "Toggle",
            story: Story::Toggle,
        },
        StoryInfo {
            name: "TapGesture",
            story: Story::TapGesture,
        },
        StoryInfo {
            name: "TextFields",
            story: Story::TextFields,
        },
        StoryInfo {
            name: "Sliders",
            story: Story::Sliders,
        },
        StoryInfo {
            name: "More Inputs",
            story: Story::MoreInputs,
        },
        StoryInfo {
            name: "Display",
            story: Story::DisplayComponents,
        },
    ]
}

pub fn container_stories() -> &'static [StoryInfo] {
    &[
        StoryInfo {
            name: "ScrollView",
            story: Story::ScrollView,
        },
        StoryInfo {
            name: "List",
            story: Story::List,
        },
        StoryInfo {
            name: "ForEach",
            story: Story::ForEach,
        },
        StoryInfo {
            name: "Conditional",
            story: Story::Conditional,
        },
    ]
}

pub fn grid_stories() -> &'static [StoryInfo] {
    &[
        StoryInfo {
            name: "Grid",
            story: Story::Grid,
        },
        StoryInfo {
            name: "LazyVGrid",
            story: Story::LazyVGrid,
        },
        StoryInfo {
            name: "LazyHGrid",
            story: Story::LazyHGrid,
        },
        StoryInfo {
            name: "Both Axes Scroll",
            story: Story::BothAxesScroll,
        },
    ]
}
