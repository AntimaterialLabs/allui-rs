//! Slider story.
//!
//! Demonstrates range value selection with SliderState.
//!
//! ```rust,ignore
//! let slider = cx.new(|_| SliderState::new().min(0.0).max(100.0).default_value(50.0));
//! cx.subscribe(&slider, |this, _, event: &SliderEvent, cx| { ... });
//! Slider::new(&slider).frame_width(200.0)
//! ```

use allui::prelude::*;
use gpui::{prelude::*, Entity};

pub fn render_sliders_story(
    slider_state: &Entity<SliderState>,
    slider_value: f32,
) -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Slider - Range value selection:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Horizontal slider:").foreground_color(Color::gray()))
                .child(
                    HStack::new()
                        .spacing(16.0)
                        .child(Slider::new(slider_state).frame_width(200.0))
                        .child(
                            Text::new(format!("{:.0}", slider_value))
                                .foreground_color(Color::green()),
                        ),
                )
                .child(
                    Text::new(format!("Current value: {:.1}", slider_value))
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Note: Subscribe to SliderEvent for value changes")
                .foreground_color(Color::gray()),
        )
}
