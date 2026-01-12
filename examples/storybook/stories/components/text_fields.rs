//! TextField and SecureField story.
//!
//! Demonstrates single-line text input and password fields.
//!
//! ```rust,ignore
//! let input = cx.new(|cx| InputState::new(window, cx).placeholder("Email..."));
//! TextField::new(&input).cleanable(true).frame_width(300.0)
//! SecureField::new(&password_input).show_toggle(true)
//! ```

use allui::prelude::*;
use gpui::{prelude::*, Entity};

pub fn render_textfields_story(
    text_input: &Entity<InputState>,
    text_input_cleanable: &Entity<InputState>,
    password_input: &Entity<InputState>,
) -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("TextField - Single-line text input:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Basic TextField:").foreground_color(Color::gray()))
                .child(TextField::new(text_input).frame_width(300.0))
                .child(
                    Text::new("TextField with cleanable (x button):")
                        .foreground_color(Color::gray()),
                )
                .child(
                    TextField::new(text_input_cleanable)
                        .cleanable(true)
                        .frame_width(300.0),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("SecureField - Password input (masked):"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    SecureField::new(password_input)
                        .show_toggle(true)
                        .frame_width(300.0),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Note: State is managed via Entity<InputState>")
                .foreground_color(Color::gray()),
        )
}
