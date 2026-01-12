//! List story.
//!
//! Demonstrates iOS-style grouped lists with sections, headers, and footers.
//!
//! ```rust,ignore
//! List::new("settings")
//!     .list_style(ListStyle::inset_grouped())
//!     .section(Section::new()
//!         .header("Account")
//!         .row(Text::new("Profile"))
//!         .footer("Manage your account"))
//! ```

use allui::prelude::*;
use gpui::prelude::*;

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
                                .section(
                                    Section::new()
                                        .header("Account")
                                        .row(Text::new("Profile"))
                                        .row(Text::new("Privacy"))
                                        .row(Text::new("Security")),
                                )
                                .section(
                                    Section::new()
                                        .header("Preferences")
                                        .footer("Customize your experience")
                                        .row(Text::new("Notifications"))
                                        .row(Text::new("Appearance"))
                                        .row(Text::new("Language")),
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
                                .section(
                                    Section::new()
                                        .row(Text::new("First Item"))
                                        .row(Text::new("Second Item"))
                                        .row(Text::new("Third Item")),
                                )
                                .frame(Frame::size(200.0, 200.0))
                                .background(Color::system_background())
                                .corner_radius(8.0),
                        ),
                ),
        )
}
