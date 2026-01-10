//! UI Components for Allui.
//!
//! Display components are thin wrappers on GPUI primitives.
//! Input components wrap gpui-component widgets.

// Display components
mod button;
mod divider;
mod image;
mod label;
mod link;
mod progress_view;
mod text;

// Input components
mod picker;
mod secure_field;
mod slider;
mod stepper;
mod text_editor;
mod text_field;
mod toggle;

// Display exports
pub use button::{Button, ButtonStyle};
pub use divider::Divider;
pub use image::Image;
pub use label::Label;
pub use link::Link;
pub use progress_view::{ProgressView, ProgressViewStyle};
pub use text::Text;

// Input exports
pub use picker::{
    IndexPath, Picker, PickerDelegate, PickerEvent, PickerGroup, PickerItem, PickerState,
    SearchableVec,
};
pub use secure_field::SecureField;
pub use slider::{Slider, SliderEvent, SliderState, SliderValue};
pub use stepper::{StepAction, Stepper, StepperEvent};
pub use text_editor::TextEditor;
pub use text_field::{InputState, TextField};
pub use toggle::Toggle;
