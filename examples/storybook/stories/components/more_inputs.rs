//! More inputs story.
//!
//! Demonstrates TextEditor (multi-line), Stepper, and Picker usage.
//!
//! ```rust,ignore
//! let editor = cx.new(|cx| InputState::new(window, cx).multi_line(true));
//! TextEditor::new(&editor).height(150.0)
//! Stepper::new(&stepper_input)  // Subscribe to StepperEvent
//! ```

use allui::prelude::*;
use gpui::{prelude::*, Entity};

pub fn render_more_inputs_story(
    text_editor_input: &Entity<InputState>,
    stepper_input: &Entity<InputState>,
    stepper_value: i32,
) -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("TextEditor - Multi-line text input:"))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    TextEditor::new(text_editor_input)
                        .height(150.0)
                        .frame_width(300.0),
                )
                .child(
                    Text::new("Note: Create InputState with .multi_line(true)")
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Stepper - Increment/decrement control:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    HStack::new()
                        .spacing(16.0)
                        .child(Text::new(format!("Value: {}", stepper_value)))
                        .child(Stepper::new(stepper_input)),
                )
                .child(
                    Text::new("Stepper triggers increment/decrement callbacks")
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Picker - Selection from options:"))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    Text::new(
                        "Picker requires PickerState + PickerDelegate. See picker.rs for usage.",
                    )
                    .foreground_color(Color::gray()),
                )
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .child(Text::new("Usage:").bold())
                        .child(
                            Text::new("let state = PickerState::new(...);")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("Picker::new(\"id\", &state).build(window, cx)")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .padding(12.0)
                        .background(Color::secondary_system_background())
                        .corner_radius(8.0),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
