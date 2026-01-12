//! Button story.
//!
//! Demonstrates button styles: BorderedProminent, Bordered, Plain.
//!
//! ```rust,ignore
//! Button::new("Click me", || println!("Clicked!"))
//!     .button_style(ButtonStyle::BorderedProminent)
//!     .disabled(false)
//! ```

use allui::prelude::*;
use gpui::prelude::*;

pub fn render_button_story() -> impl IntoElement {
    VStack::new()
        .spacing(12.0)
        .alignment(HorizontalAlignment::Leading)
        .child(
            Button::new("Bordered Prominent", || {
                println!("Bordered Prominent clicked!");
            })
            .button_style(ButtonStyle::BorderedProminent),
        )
        .child(
            Button::new("Bordered", || {
                println!("Bordered clicked!");
            })
            .button_style(ButtonStyle::Bordered),
        )
        .child(
            Button::new("Plain", || {
                println!("Plain clicked!");
            })
            .button_style(ButtonStyle::Plain),
        )
        .child(
            Button::new("Disabled", || {})
                .button_style(ButtonStyle::BorderedProminent)
                .disabled(true),
        )
}
