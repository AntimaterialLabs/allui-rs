//! Core modifier system for Allui.
//!
//! Modifiers wrap views in container elements, but visual modifiers (background,
//! corner_radius, border) are merged onto a single div for correct rendering.
//! This is necessary because GPUI's overflow clipping doesn't respect border-radius.

use gpui::{
    AnyElement, App, ClickEvent, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};
use gpui_component::ActiveTheme;

use crate::style::Color;
use crate::types::ClickHandler;

pub use crate::alignment::{Alignment, HorizontalAlignment, VerticalAlignment};

/// Accumulates visual styles (background, corner_radius, border) on a single div.
/// Required because GPUI cannot clip overflow to rounded bounds.
pub struct StyledContainer<V> {
    child: V,
    background: Option<Color>,
    corner_radius: Option<f32>,
    border_color: Option<Color>,
    border_width: Option<f32>,
}

impl<V> StyledContainer<V> {
    pub fn new(child: V) -> Self {
        Self {
            child,
            background: None,
            corner_radius: None,
            border_color: None,
            border_width: None,
        }
    }

    fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    fn with_border(mut self, color: Color, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = Some(width);
        self
    }

    #[must_use]
    pub fn corner_radius(self, radius: f32) -> Self {
        self.with_corner_radius(radius)
    }

    #[must_use]
    pub fn border(self, color: impl Into<Color>, width: f32) -> Self {
        self.with_border(color.into(), width)
    }
}

impl<V: IntoElement + 'static> IntoElement for StyledContainer<V> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        StyledContainerElement { container: self }.into_any_element()
    }
}

#[derive(IntoElement)]
struct StyledContainerElement<V: IntoElement + 'static> {
    container: StyledContainer<V>,
}

impl<V: IntoElement + 'static> RenderOnce for StyledContainerElement<V> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_dark = cx.theme().is_dark();
        let mut container = div().flex_grow();

        if let Some(color) = self.container.background {
            container = container.bg(color.resolve(is_dark));
        }

        if let Some(radius) = self.container.corner_radius {
            container = container.rounded(px(radius)).overflow_hidden();
        }

        if let Some(color) = self.container.border_color {
            container = container.border_color(color.resolve(is_dark));
            if let Some(width) = self.container.border_width {
                container = if width <= 1.0 {
                    container.border_1()
                } else if width <= 2.0 {
                    container.border_2()
                } else if width <= 4.0 {
                    container.border_4()
                } else {
                    container.border_8()
                };
            }
        }

        container.child(self.container.child)
    }
}

/// A view that has been wrapped with a modifier.
///
/// Each modifier application creates a new `Modified` wrapper,
/// allowing modifiers to be composed while maintaining order semantics.
pub struct Modified<V> {
    pub(crate) child: V,
    pub(crate) modifier: ModifierKind,
}

#[derive(Clone)]
pub enum ModifierKind {
    Padding(Padding),
    Foreground(Color),
    CornerRadius(f32),
    Border {
        color: Color,
        width: f32,
    },
    Shadow {
        radius: f32,
        color: Option<Color>,
        x: f32,
        y: f32,
    },
    Opacity(f32),
    Frame(Frame),
    Hidden(bool),
    Disabled(bool),
    Scale(f32),
    Tint(Color),
    FixedSize {
        horizontal: bool,
        vertical: bool,
    },
    AspectRatio {
        ratio: f32,
        content_mode: ContentMode,
    },
}

/// Padding values for each edge.
#[derive(Clone, Copy, Debug, Default)]
pub struct Padding {
    pub top: f32,
    pub leading: f32,
    pub bottom: f32,
    pub trailing: f32,
}

impl Padding {
    /// Uniform padding on all sides.
    pub fn all(value: f32) -> Self {
        Self {
            top: value,
            leading: value,
            bottom: value,
            trailing: value,
        }
    }

    /// Horizontal and vertical padding.
    pub fn axes(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            leading: horizontal,
            bottom: vertical,
            trailing: horizontal,
        }
    }

    /// Padding for specific edges.
    pub fn edges(top: f32, leading: f32, bottom: f32, trailing: f32) -> Self {
        Self {
            top,
            leading,
            bottom,
            trailing,
        }
    }
}

impl From<f32> for Padding {
    fn from(value: f32) -> Self {
        Self::all(value)
    }
}

impl From<(f32, f32)> for Padding {
    fn from((horizontal, vertical): (f32, f32)) -> Self {
        Self::axes(horizontal, vertical)
    }
}

impl From<(f32, f32, f32, f32)> for Padding {
    fn from((top, leading, bottom, trailing): (f32, f32, f32, f32)) -> Self {
        Self::edges(top, leading, bottom, trailing)
    }
}

/// Frame dimensions and constraints.
///
/// Mirrors SwiftUI's frame modifier parameters. SwiftUI has two frame overloads:
///
/// 1. **Simple fixed frame**: `.frame(width:height:alignment:)`
///    - Sets exact dimensions, `nil` means inherit from child
///
/// 2. **Flexible frame**: `.frame(minWidth:idealWidth:maxWidth:minHeight:idealHeight:maxHeight:alignment:)`
///    - Sets constraints that the layout system uses to size the view
///
/// # SwiftUI Behavior Rules
///
/// - If only `width` or `height` is set, the other dimension inherits from the child
/// - `min/max` constraints clamp the proposed size from the parent
/// - `ideal` dimensions are used when the parent proposes `nil` (unspecified size)
/// - When both `width` and `min_width`/`max_width` are set, `width` takes precedence
/// - `alignment` controls where the child is positioned within the frame
///
/// # Examples
///
/// ```rust,ignore
/// // Fixed 100x50 frame, centered (default)
/// Text::new("Hello").frame(Frame::size(100.0, 50.0))
///
/// // Fixed width, height from content
/// Text::new("Hello").frame(Frame::width(200.0))
///
/// // Expand to fill available width
/// Text::new("Hello").frame(Frame::max_width(f32::INFINITY))
///
/// // Flexible with constraints
/// Text::new("Hello").frame(Frame::new()
///     .min_width(100.0)
///     .max_width(300.0)
///     .alignment(Alignment::leading()))
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Frame {
    /// Fixed width. If set, overrides min/max width constraints.
    pub width: Option<f32>,
    /// Fixed height. If set, overrides min/max height constraints.
    pub height: Option<f32>,
    /// Minimum width constraint.
    pub min_width: Option<f32>,
    /// Ideal width - used when parent proposes unspecified size.
    pub ideal_width: Option<f32>,
    /// Maximum width constraint. Use `f32::INFINITY` to fill available space.
    pub max_width: Option<f32>,
    /// Minimum height constraint.
    pub min_height: Option<f32>,
    /// Ideal height - used when parent proposes unspecified size.
    pub ideal_height: Option<f32>,
    /// Maximum height constraint. Use `f32::INFINITY` to fill available space.
    pub max_height: Option<f32>,
    /// Alignment of the child within the frame.
    pub alignment: Alignment,
}

impl Frame {
    /// Creates an empty frame configuration.
    ///
    /// Use builder methods to add constraints:
    /// ```rust,ignore
    /// Frame::new().min_width(100.0).max_width(300.0)
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a frame with fixed width and height.
    ///
    /// Equivalent to SwiftUI's `.frame(width: w, height: h)`.
    #[must_use]
    pub fn size(width: f32, height: f32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
            ..Default::default()
        }
    }

    /// Creates a frame with fixed width only.
    ///
    /// Height will be determined by the child's natural size.
    #[must_use]
    pub fn width(width: f32) -> Self {
        Self {
            width: Some(width),
            ..Default::default()
        }
    }

    /// Creates a frame with fixed height only.
    ///
    /// Width will be determined by the child's natural size.
    #[must_use]
    pub fn height(height: f32) -> Self {
        Self {
            height: Some(height),
            ..Default::default()
        }
    }

    /// Creates a frame that expands to fill available width.
    ///
    /// Equivalent to SwiftUI's `.frame(maxWidth: .infinity)`.
    #[must_use]
    pub fn fill_width() -> Self {
        Self {
            max_width: Some(f32::INFINITY),
            ..Default::default()
        }
    }

    /// Creates a frame that expands to fill available height.
    ///
    /// Equivalent to SwiftUI's `.frame(maxHeight: .infinity)`.
    #[must_use]
    pub fn fill_height() -> Self {
        Self {
            max_height: Some(f32::INFINITY),
            ..Default::default()
        }
    }

    /// Creates a frame that expands to fill all available space.
    ///
    /// Equivalent to SwiftUI's `.frame(maxWidth: .infinity, maxHeight: .infinity)`.
    #[must_use]
    pub fn fill() -> Self {
        Self {
            max_width: Some(f32::INFINITY),
            max_height: Some(f32::INFINITY),
            ..Default::default()
        }
    }

    // Builder methods for chaining

    /// Sets minimum width constraint.
    #[must_use]
    pub fn min_width(mut self, value: f32) -> Self {
        self.min_width = Some(value);
        self
    }

    /// Sets ideal width (used when parent proposes unspecified size).
    #[must_use]
    pub fn ideal_width(mut self, value: f32) -> Self {
        self.ideal_width = Some(value);
        self
    }

    /// Sets maximum width constraint.
    #[must_use]
    pub fn max_width(mut self, value: f32) -> Self {
        self.max_width = Some(value);
        self
    }

    /// Sets minimum height constraint.
    #[must_use]
    pub fn min_height(mut self, value: f32) -> Self {
        self.min_height = Some(value);
        self
    }

    /// Sets ideal height (used when parent proposes unspecified size).
    #[must_use]
    pub fn ideal_height(mut self, value: f32) -> Self {
        self.ideal_height = Some(value);
        self
    }

    /// Sets maximum height constraint.
    #[must_use]
    pub fn max_height(mut self, value: f32) -> Self {
        self.max_height = Some(value);
        self
    }

    /// Sets alignment of the child within the frame.
    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

/// Content mode for aspect ratio.
#[derive(Clone, Copy, Debug, Default)]
pub enum ContentMode {
    #[default]
    Fit,
    Fill,
}

/// The core modifier trait that all Allui views implement.
///
/// This trait provides the SwiftUI-like modifier chain API.
pub trait Modifier: Sized {
    // Layout modifiers

    /// Add padding around the view.
    fn padding(self, value: impl Into<Padding>) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Padding(value.into()),
        }
    }

    /// Add padding to specific edges.
    fn padding_edges(self, top: f32, leading: f32, bottom: f32, trailing: f32) -> Modified<Self> {
        self.padding(Padding::edges(top, leading, bottom, trailing))
    }

    /// Apply a frame with the specified configuration.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Fixed dimensions
    /// Text::new("Hello").frame(Frame::size(100.0, 50.0))
    ///
    /// // Just width
    /// Text::new("Hello").frame(Frame::width(200.0))
    ///
    /// // Fill available width
    /// Text::new("Hello").frame(Frame::fill_width())
    ///
    /// // Flexible constraints
    /// Text::new("Hello").frame(Frame::new().min_width(100.0).max_width(300.0))
    /// ```
    fn frame(self, frame: Frame) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Frame(frame),
        }
    }

    /// Set fixed width and height. Shorthand for `frame(Frame::size(w, h))`.
    #[must_use]
    fn frame_size(self, width: f32, height: f32) -> Modified<Self> {
        self.frame(Frame::size(width, height))
    }

    /// Set fixed width only. Shorthand for `frame(Frame::width(w))`.
    #[must_use]
    fn frame_width(self, width: f32) -> Modified<Self> {
        self.frame(Frame::width(width))
    }

    /// Set fixed height only. Shorthand for `frame(Frame::height(h))`.
    #[must_use]
    fn frame_height(self, height: f32) -> Modified<Self> {
        self.frame(Frame::height(height))
    }

    /// Prevent the view from expanding beyond its ideal size.
    fn fixed_size(self, horizontal: bool, vertical: bool) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::FixedSize {
                horizontal,
                vertical,
            },
        }
    }

    /// Constrain the view to a specific aspect ratio.
    fn aspect_ratio(self, ratio: f32, content_mode: ContentMode) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::AspectRatio {
                ratio,
                content_mode,
            },
        }
    }

    // Visual modifiers

    fn background(self, color: impl Into<Color>) -> StyledContainer<Self> {
        StyledContainer::new(self).with_background(color.into())
    }

    /// Set the foreground (text) color.
    fn foreground_color(self, color: impl Into<Color>) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Foreground(color.into()),
        }
    }

    /// Set the tint color for interactive elements.
    fn tint(self, color: impl Into<Color>) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Tint(color.into()),
        }
    }

    /// Round the corners.
    fn corner_radius(self, radius: f32) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::CornerRadius(radius),
        }
    }

    /// Add a border.
    fn border(self, color: impl Into<Color>, width: f32) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Border {
                color: color.into(),
                width,
            },
        }
    }

    /// Add a shadow.
    fn shadow(self, radius: f32) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Shadow {
                radius,
                color: None,
                x: 0.0,
                y: 0.0,
            },
        }
    }

    /// Add a shadow with full configuration.
    fn shadow_with(self, radius: f32, color: impl Into<Color>, x: f32, y: f32) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Shadow {
                radius,
                color: Some(color.into()),
                x,
                y,
            },
        }
    }

    /// Set the opacity.
    fn opacity(self, value: f32) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Opacity(value),
        }
    }

    // Behavior modifiers

    /// Hide the view.
    fn hidden(self, is_hidden: bool) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Hidden(is_hidden),
        }
    }

    /// Disable interaction.
    fn disabled(self, is_disabled: bool) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Disabled(is_disabled),
        }
    }

    /// Apply a scale transform.
    fn scale(self, value: f32) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::Scale(value),
        }
    }

    /// Add a tap gesture handler.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// Text::new("Tap me")
    ///     .on_tap_gesture("my-button", || {
    ///         println!("Tapped!");
    ///     })
    /// ```
    fn on_tap_gesture(
        self,
        id: impl Into<SharedString>,
        handler: impl Fn() + 'static,
    ) -> Tappable<Self> {
        Tappable {
            child: self,
            handler: Box::new(move |_, _, _| handler()),
            id: id.into(),
        }
    }

    /// Add a tap gesture handler with access to the event.
    fn on_tap_gesture_with(
        self,
        id: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Tappable<Self> {
        Tappable {
            child: self,
            handler: Box::new(handler),
            id: id.into(),
        }
    }
}

// Implement Modifier for Modified so modifiers can be chained
impl<V> Modifier for Modified<V> {}

// Implement Modifier for StyledContainer so other modifiers can be chained
impl<V: IntoElement + 'static> Modifier for StyledContainer<V> {}

/// A view wrapped with a tap gesture handler.
pub struct Tappable<V> {
    child: V,
    handler: ClickHandler,
    id: SharedString,
}

// Implement Modifier for Tappable so modifiers can be chained
impl<V> Modifier for Tappable<V> {}

impl<V: IntoElement + 'static> IntoElement for Tappable<V> {
    type Element = gpui::AnyElement;

    fn into_element(self) -> Self::Element {
        TappableElement { tappable: self }.into_any_element()
    }
}

#[derive(IntoElement)]
struct TappableElement<V: IntoElement + 'static> {
    tappable: Tappable<V>,
}

impl<V: IntoElement + 'static> RenderOnce for TappableElement<V> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let child = self.tappable.child.into_any_element();
        let handler = self.tappable.handler;
        let element_id = self.tappable.id;

        div()
            .id(element_id)
            .cursor_pointer()
            .on_click(move |event, window, cx| {
                handler(event, window, cx);
            })
            .child(child)
    }
}

// Implement IntoElement for Modified so it can be rendered
impl<V: IntoElement + 'static> IntoElement for Modified<V> {
    type Element = gpui::AnyElement;

    fn into_element(self) -> Self::Element {
        ModifiedElement { modified: self }.into_any_element()
    }
}

/// Internal wrapper to implement RenderOnce for Modified
#[derive(IntoElement)]
struct ModifiedElement<V: IntoElement + 'static> {
    modified: Modified<V>,
}

impl<V: IntoElement + 'static> RenderOnce for ModifiedElement<V> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let child = self.modified.child.into_any_element();
        let is_dark = cx.theme().is_dark();

        // Apply the modifier by wrapping the child in a container
        match self.modified.modifier {
            ModifierKind::Padding(padding) => {
                // Padding wrapper stretches to fill parent so flex children work
                div()
                    .flex_grow()
                    .pt(px(padding.top))
                    .pb(px(padding.bottom))
                    .pl(px(padding.leading))
                    .pr(px(padding.trailing))
                    .child(child)
            }

            ModifierKind::Foreground(color) => {
                // Resolve semantic colors based on current theme
                div()
                    .flex_grow()
                    .text_color(color.resolve(is_dark))
                    .child(child)
            }
            ModifierKind::CornerRadius(radius) => div()
                .flex_grow()
                .rounded(px(radius))
                .overflow_hidden()
                .child(child),
            ModifierKind::Border { color, width } => {
                // GPUI uses border_N() methods
                // Resolve semantic colors based on current theme
                let container = div().flex_grow().border_color(color.resolve(is_dark));
                let container = if width <= 1.0 {
                    container.border_1()
                } else if width <= 2.0 {
                    container.border_2()
                } else if width <= 4.0 {
                    container.border_4()
                } else {
                    container.border_8()
                };
                container.child(child)
            }
            ModifierKind::Shadow {
                radius,
                color: _,
                x: _,
                y: _,
            } => {
                // GPUI has shadow_sm, shadow_md, shadow_lg, etc.
                // TODO: Use color, x, y when GPUI supports custom shadows
                let container = if radius <= 2.0 {
                    div().shadow_sm()
                } else if radius <= 4.0 {
                    div().shadow_md()
                } else if radius <= 8.0 {
                    div().shadow_lg()
                } else {
                    div().shadow_xl()
                };
                container.flex_grow().child(child)
            }
            ModifierKind::Opacity(value) => div().flex_grow().opacity(value).child(child),
            ModifierKind::Frame(frame) => {
                let mut container = div();

                if let Some(w) = frame.width {
                    container = container.w(px(w));
                }
                if let Some(h) = frame.height {
                    container = container.h(px(h));
                }
                if let Some(min_w) = frame.min_width {
                    container = container.min_w(px(min_w));
                }
                if let Some(max_w) = frame.max_width {
                    container = container.max_w(px(max_w));
                }
                if let Some(min_h) = frame.min_height {
                    container = container.min_h(px(min_h));
                }
                if let Some(max_h) = frame.max_height {
                    container = container.max_h(px(max_h));
                }

                let container = container.flex();
                let container = frame.alignment.horizontal.apply_as_justify(container);
                let container = frame.alignment.vertical.apply_as_items(container);
                container.child(child)
            }
            ModifierKind::Hidden(is_hidden) => {
                if is_hidden {
                    div() // Return empty div when hidden
                } else {
                    div().child(child)
                }
            }
            ModifierKind::Disabled(_is_disabled) => {
                // NOTE: Disabled state is handled at the component level.
                // Use component-specific disabled props (e.g., TextField.disabled()).
                // This modifier exists for SwiftUI API compatibility but is a no-op.
                div().child(child)
            }
            ModifierKind::Scale(_value) => {
                // NOTE: Scale transforms are not supported by GPUI.
                // GPUI lacks CSS transform support. This modifier is a no-op.
                // Workaround: Use frame() with manually calculated dimensions.
                div().child(child)
            }
            ModifierKind::Tint(_color) => {
                // NOTE: Tint requires color blending/compositing not available in GPUI.
                // This modifier is a no-op until GPUI adds blend mode support.
                // For images, consider using pre-tinted assets.
                div().child(child)
            }
            ModifierKind::FixedSize {
                horizontal,
                vertical,
            } => {
                let mut container = div();
                // Horizontal: prevent growing and shrinking in x-axis
                if horizontal {
                    container = container.flex_none();
                }
                // Vertical: prevent shrinking to preserve natural height
                if vertical {
                    container = container.flex_shrink_0();
                }
                container.child(child)
            }
            ModifierKind::AspectRatio {
                ratio: _,
                content_mode: _,
            } => {
                // NOTE: AspectRatio requires setting Style.aspect_ratio, but GPUI's
                // StyleRefinement (used by Styled trait) doesn't expose this field.
                // Would need custom Element implementation to access raw Style.
                // For now, this is a no-op. Use explicit frame dimensions as workaround.
                div().child(child)
            }
        }
    }
}
