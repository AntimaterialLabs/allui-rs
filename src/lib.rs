//! # Allui
//!
//! A SwiftUI-inspired UI framework for Rust, built on GPUI and gpui-component.
//!
//! Allui mirrors SwiftUI's declarative API and layout semantics as closely as
//! Rust idioms allow. The core principle is that layout behavior must match
//! SwiftUI exactly - a developer familiar with SwiftUI should be able to predict
//! how Allui components will size, align, and compose.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use allui::prelude::*;
//!
//! VStack::new()
//!     .spacing(12.0)
//!     .child(
//!         Text::new("Hello, Allui!")
//!             .font(Font::title())
//!     )
//!     .child(
//!         Button::new("Click me", || {
//!             println!("Clicked!");
//!         })
//!     )
//!     .padding(20.0)
//!     .background(Color::gray())
//! ```
//!
//! ## Modifier System
//!
//! Like SwiftUI, modifiers wrap views and order matters:
//!
//! ```rust,ignore
//! // Padding is inside the background
//! Text::new("Hello")
//!     .padding(10.0)
//!     .background(Color::red())
//!
//! // Padding is outside the background
//! Text::new("Hello")
//!     .background(Color::red())
//!     .padding(10.0)
//! ```

// Core modules
pub mod alignment;
pub mod components;
pub mod layout;
pub mod modifier;
pub mod style;
pub mod types;

pub mod prelude;

// Re-export commonly used items at crate root
pub use modifier::{Modified, Modifier, Tappable};
