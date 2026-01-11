//! Convenient imports for Allui users.
//!
//! # Example
//!
//! ```rust,ignore
//! use allui::prelude::*;
//! ```

// Layout primitives
pub use crate::layout::{
    Alignment, EdgeInsets, EdgeInsetsExt, EdgeSet, EmptyView, ForEach, Grid, GridItem,
    GridItemSize, GridRow, Group, HStack, HorizontalAlignment, If, IfLet, LazyHGrid, LazyHStack,
    LazyVGrid, LazyVStack, List, ListSectionSpacing, ListStyle, RowConfiguration, ScrollAxes,
    ScrollView, Section, SectionMargins, Spacer, VStack, VerticalAlignment,
    VirtualListScrollHandle, ZStack,
};

// Display components
pub use crate::components::{
    Button, ButtonStyle, Divider, Image, Label, Link, ProgressView, ProgressViewStyle, Text,
    TruncationMode,
};

// Re-export IconName from gpui-component for Label::with_icon
pub use gpui_component::IconName;

// Input components
pub use crate::components::{
    IndexPath, InputState, Picker, PickerDelegate, PickerEvent, PickerGroup, PickerItem,
    PickerState, SearchableVec, SecureField, Slider, SliderEvent, SliderState, SliderValue,
    StepAction, Stepper, StepperEvent, TextEditor, TextField, Toggle,
};

// Modifier trait and types
pub use crate::modifier::{ContentMode, Frame, Modified, Modifier, Padding, Tappable};

// Common types
pub use crate::types::ClickHandler;

// Styling
pub use crate::style::{Color, Font, FontDesign, FontWeight, SemanticColor};
