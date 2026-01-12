//! Color type for Allui.
//!
//! Colors can be static (specific HSLA values) or semantic (adapt to light/dark mode).
//! Semantic colors like `Color::label()` automatically resolve to the correct value
//! based on the current theme mode at render time.

use gpui::Hsla;

/// Semantic color variants that adapt to light/dark mode.
///
/// These correspond to SwiftUI's semantic colors that automatically
/// change based on the environment's color scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticColor {
    /// Primary label color for text
    Label,
    /// Secondary label color for subtitles
    SecondaryLabel,
    /// Tertiary label color for placeholder text
    TertiaryLabel,
    /// Primary system background
    SystemBackground,
    /// Secondary system background (grouped tables)
    SecondarySystemBackground,
    /// Tertiary system background
    TertiarySystemBackground,
    /// Separator for dividers
    Separator,
    /// Opaque separator
    OpaqueSeparator,
    /// TextField background color
    TextFieldBackground,
    /// TextField border color
    TextFieldBorder,
}

impl SemanticColor {
    /// Resolve this semantic color to a concrete HSLA value.
    fn resolve(self, dark_mode: bool) -> Hsla {
        match (self, dark_mode) {
            // Label colors
            (SemanticColor::Label, false) => Color::static_rgb(0.0, 0.0, 0.0).hsla,
            (SemanticColor::Label, true) => Color::static_rgb(1.0, 1.0, 1.0).hsla,

            (SemanticColor::SecondaryLabel, false) => {
                Color::static_rgba(0.24, 0.24, 0.26, 0.6).hsla
            }
            (SemanticColor::SecondaryLabel, true) => Color::static_rgba(0.92, 0.92, 0.96, 0.6).hsla,

            (SemanticColor::TertiaryLabel, false) => Color::static_rgba(0.24, 0.24, 0.26, 0.3).hsla,
            (SemanticColor::TertiaryLabel, true) => Color::static_rgba(0.92, 0.92, 0.96, 0.3).hsla,

            // Background colors
            (SemanticColor::SystemBackground, false) => Color::static_rgb(1.0, 1.0, 1.0).hsla,
            (SemanticColor::SystemBackground, true) => Color::static_rgb(0.0, 0.0, 0.0).hsla,

            (SemanticColor::SecondarySystemBackground, false) => {
                Color::static_rgb(0.95, 0.95, 0.97).hsla
            }
            (SemanticColor::SecondarySystemBackground, true) => Color::static_hex(0x1c1c1e).hsla,

            (SemanticColor::TertiarySystemBackground, false) => {
                Color::static_rgb(1.0, 1.0, 1.0).hsla
            }
            (SemanticColor::TertiarySystemBackground, true) => Color::static_hex(0x2c2c2e).hsla,

            // Separator colors
            (SemanticColor::Separator, false) => Color::static_rgba(0.24, 0.24, 0.26, 0.29).hsla,
            (SemanticColor::Separator, true) => Color::static_rgba(0.33, 0.33, 0.35, 0.6).hsla,

            (SemanticColor::OpaqueSeparator, false) => Color::static_rgb(0.78, 0.78, 0.8).hsla,
            (SemanticColor::OpaqueSeparator, true) => Color::static_hex(0x38383a).hsla,

            (SemanticColor::TextFieldBackground, false) => Color::static_rgb(1.0, 1.0, 1.0).hsla,
            (SemanticColor::TextFieldBackground, true) => {
                Color::static_rgb(0.118, 0.118, 0.118).hsla
            }

            (SemanticColor::TextFieldBorder, false) => Color::static_rgb(0.85, 0.85, 0.85).hsla,
            (SemanticColor::TextFieldBorder, true) => Color::static_rgb(0.247, 0.247, 0.247).hsla,
        }
    }
}

/// A color value that can be used for backgrounds, foregrounds, borders, etc.
///
/// Colors can be either:
/// - **Static**: A specific HSLA value that doesn't change
/// - **Semantic**: An adaptive color that resolves differently in light/dark mode
///
/// # Example
///
/// ```rust,ignore
/// // Static color - always red
/// let red = Color::red();
///
/// // Semantic color - adapts to light/dark mode
/// let label = Color::label();
///
/// // Resolve semantic color at render time
/// let resolved = label.resolve(is_dark_mode);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Color {
    /// The underlying GPUI color representation (for static colors or default).
    pub(crate) hsla: Hsla,
    /// If Some, this is a semantic color that adapts to dark mode.
    pub(crate) semantic: Option<SemanticColor>,
}

impl Color {
    // ========================================================================
    // Static color constructors (internal helpers)
    // ========================================================================

    /// Internal: Create a static color from RGB (no semantic).
    fn static_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::static_rgba(r, g, b, 1.0)
    }

    /// Internal: Create a static color from RGBA (no semantic).
    fn static_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            hsla: gpui::rgba(
                (r * 255.0) as u32 * 0x1000000
                    + (g * 255.0) as u32 * 0x10000
                    + (b * 255.0) as u32 * 0x100
                    + (a * 255.0) as u32,
            )
            .into(),
            semantic: None,
        }
    }

    /// Internal: Create a static color from hex (no semantic).
    fn static_hex(hex: u32) -> Self {
        Self {
            hsla: gpui::rgb(hex).into(),
            semantic: None,
        }
    }

    // ========================================================================
    // Public color constructors
    // ========================================================================

    /// Create a color from RGBA values (0.0-1.0).
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::static_rgba(r, g, b, a)
    }

    /// Create a color from RGB values (0.0-1.0) with full opacity.
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Create a color from a hex value (e.g., 0xFF0000 for red).
    pub fn hex(hex: u32) -> Self {
        Self::static_hex(hex)
    }

    /// Create a color from HSLA values.
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self {
            hsla: Hsla { h, s, l, a },
            semantic: None,
        }
    }

    /// Modify the alpha/opacity of this color.
    ///
    /// Note: This converts semantic colors to static colors with the opacity applied.
    pub fn opacity(self, alpha: f32) -> Self {
        // Resolve semantic first, then apply opacity
        let base_hsla = if let Some(semantic) = self.semantic {
            // Use light mode as base when opacity is applied
            semantic.resolve(false)
        } else {
            self.hsla
        };

        Self {
            hsla: Hsla {
                a: alpha,
                ..base_hsla
            },
            semantic: None, // Opacity converts to static
        }
    }

    // ========================================================================
    // Resolution
    // ========================================================================

    /// Resolve this color to a concrete HSLA value.
    ///
    /// For static colors, returns the stored HSLA.
    /// For semantic colors, resolves based on the dark mode flag.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let label = Color::label();
    /// let resolved = label.resolve(cx.theme().is_dark());
    /// ```
    pub fn resolve(self, dark_mode: bool) -> Hsla {
        if let Some(semantic) = self.semantic {
            semantic.resolve(dark_mode)
        } else {
            self.hsla
        }
    }

    /// Check if this is a semantic (adaptive) color.
    pub fn is_semantic(&self) -> bool {
        self.semantic.is_some()
    }

    // ========================================================================
    // Predefined static colors (matching SwiftUI)
    // ========================================================================

    pub fn clear() -> Self {
        Self::rgba(0.0, 0.0, 0.0, 0.0)
    }

    pub fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }

    pub fn gray() -> Self {
        Self::rgb(0.5, 0.5, 0.5)
    }

    pub fn red() -> Self {
        Self::rgb(1.0, 0.23, 0.19)
    }

    pub fn orange() -> Self {
        Self::rgb(1.0, 0.58, 0.0)
    }

    pub fn yellow() -> Self {
        Self::rgb(1.0, 0.8, 0.0)
    }

    pub fn green() -> Self {
        Self::rgb(0.2, 0.78, 0.35)
    }

    pub fn mint() -> Self {
        Self::rgb(0.0, 0.78, 0.75)
    }

    pub fn teal() -> Self {
        Self::rgb(0.19, 0.69, 0.78)
    }

    pub fn cyan() -> Self {
        Self::rgb(0.39, 0.82, 1.0)
    }

    pub fn blue() -> Self {
        Self::rgb(0.0, 0.48, 1.0)
    }

    pub fn indigo() -> Self {
        Self::rgb(0.35, 0.34, 0.84)
    }

    pub fn purple() -> Self {
        Self::rgb(0.69, 0.32, 0.87)
    }

    pub fn pink() -> Self {
        Self::rgb(1.0, 0.18, 0.33)
    }

    pub fn brown() -> Self {
        Self::rgb(0.64, 0.52, 0.37)
    }

    // ========================================================================
    // Semantic alias colors (non-adaptive)
    // ========================================================================

    /// Primary accent color (currently blue).
    pub fn primary() -> Self {
        Self::blue()
    }

    /// Secondary accent color (currently gray).
    pub fn secondary() -> Self {
        Self::gray()
    }

    // ========================================================================
    // Adaptive semantic colors (light/dark mode aware)
    // ========================================================================

    /// Primary label color for text.
    ///
    /// Adapts automatically: black in light mode, white in dark mode.
    pub fn label() -> Self {
        Self {
            hsla: SemanticColor::Label.resolve(false), // Default to light
            semantic: Some(SemanticColor::Label),
        }
    }

    /// Secondary label color for subtitles and descriptions.
    ///
    /// Adapts automatically based on color scheme.
    pub fn secondary_label() -> Self {
        Self {
            hsla: SemanticColor::SecondaryLabel.resolve(false),
            semantic: Some(SemanticColor::SecondaryLabel),
        }
    }

    /// Tertiary label color for placeholder text.
    ///
    /// Adapts automatically based on color scheme.
    pub fn tertiary_label() -> Self {
        Self {
            hsla: SemanticColor::TertiaryLabel.resolve(false),
            semantic: Some(SemanticColor::TertiaryLabel),
        }
    }

    /// System background color.
    ///
    /// Adapts automatically: white in light mode, black in dark mode.
    pub fn system_background() -> Self {
        Self {
            hsla: SemanticColor::SystemBackground.resolve(false),
            semantic: Some(SemanticColor::SystemBackground),
        }
    }

    /// Secondary system background for grouped table views.
    ///
    /// Adapts automatically based on color scheme.
    pub fn secondary_system_background() -> Self {
        Self {
            hsla: SemanticColor::SecondarySystemBackground.resolve(false),
            semantic: Some(SemanticColor::SecondarySystemBackground),
        }
    }

    /// Tertiary system background.
    ///
    /// Adapts automatically based on color scheme.
    pub fn tertiary_system_background() -> Self {
        Self {
            hsla: SemanticColor::TertiarySystemBackground.resolve(false),
            semantic: Some(SemanticColor::TertiarySystemBackground),
        }
    }

    /// Separator color for dividers.
    ///
    /// Adapts automatically based on color scheme.
    pub fn separator() -> Self {
        Self {
            hsla: SemanticColor::Separator.resolve(false),
            semantic: Some(SemanticColor::Separator),
        }
    }

    /// Opaque separator color.
    ///
    /// Adapts automatically based on color scheme.
    pub fn opaque_separator() -> Self {
        Self {
            hsla: SemanticColor::OpaqueSeparator.resolve(false),
            semantic: Some(SemanticColor::OpaqueSeparator),
        }
    }

    pub fn text_field_background() -> Self {
        Self {
            hsla: SemanticColor::TextFieldBackground.resolve(false),
            semantic: Some(SemanticColor::TextFieldBackground),
        }
    }

    pub fn text_field_border() -> Self {
        Self {
            hsla: SemanticColor::TextFieldBorder.resolve(false),
            semantic: Some(SemanticColor::TextFieldBorder),
        }
    }

    // ========================================================================
    // Deprecated explicit light/dark variants
    // ========================================================================

    /// Primary label color for text (light mode).
    #[deprecated(since = "0.2.0", note = "Use Color::label() which auto-adapts")]
    pub fn label_light() -> Self {
        Self::static_rgb(0.0, 0.0, 0.0)
    }

    /// Primary label color for text (dark mode).
    #[deprecated(since = "0.2.0", note = "Use Color::label() which auto-adapts")]
    pub fn label_dark() -> Self {
        Self::static_rgb(1.0, 1.0, 1.0)
    }

    /// Secondary label color (dark mode).
    #[deprecated(
        since = "0.2.0",
        note = "Use Color::secondary_label() which auto-adapts"
    )]
    pub fn secondary_label_dark() -> Self {
        Self::static_rgba(0.92, 0.92, 0.96, 0.6)
    }

    /// Tertiary label color (dark mode).
    #[deprecated(
        since = "0.2.0",
        note = "Use Color::tertiary_label() which auto-adapts"
    )]
    pub fn tertiary_label_dark() -> Self {
        Self::static_rgba(0.92, 0.92, 0.96, 0.3)
    }

    /// System background color (dark mode).
    #[deprecated(
        since = "0.2.0",
        note = "Use Color::system_background() which auto-adapts"
    )]
    pub fn system_background_dark() -> Self {
        Self::static_rgb(0.0, 0.0, 0.0)
    }

    /// Secondary system background (dark mode).
    #[deprecated(
        since = "0.2.0",
        note = "Use Color::secondary_system_background() which auto-adapts"
    )]
    pub fn secondary_system_background_dark() -> Self {
        Self::static_hex(0x1c1c1e)
    }

    /// Tertiary system background (dark mode).
    #[deprecated(
        since = "0.2.0",
        note = "Use Color::tertiary_system_background() which auto-adapts"
    )]
    pub fn tertiary_system_background_dark() -> Self {
        Self::static_hex(0x2c2c2e)
    }

    /// Separator color (dark mode).
    #[deprecated(since = "0.2.0", note = "Use Color::separator() which auto-adapts")]
    pub fn separator_dark() -> Self {
        Self::static_rgba(0.33, 0.33, 0.35, 0.6)
    }

    /// Opaque separator color (dark mode).
    #[deprecated(
        since = "0.2.0",
        note = "Use Color::opaque_separator() which auto-adapts"
    )]
    pub fn opaque_separator_dark() -> Self {
        Self::static_hex(0x38383a)
    }

    // ========================================================================
    // Conversion
    // ========================================================================

    /// Convert to GPUI's Hsla type.
    ///
    /// Note: For semantic colors, this returns the light mode value.
    /// Use `resolve(dark_mode)` for proper theme-aware conversion.
    pub fn to_hsla(self) -> Hsla {
        self.hsla
    }
}

impl From<Hsla> for Color {
    fn from(hsla: Hsla) -> Self {
        Self {
            hsla,
            semantic: None,
        }
    }
}

impl From<Color> for Hsla {
    fn from(color: Color) -> Self {
        color.hsla
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opacity_resolves_semantic_to_static() {
        let semantic = Color::label();
        assert!(semantic.is_semantic());

        let with_opacity = semantic.opacity(0.5);
        assert!(
            !with_opacity.is_semantic(),
            "applying opacity should resolve semantic color to static"
        );
    }

    #[test]
    fn semantic_label_adapts_to_color_scheme() {
        let label = Color::label();

        let light = label.resolve(false).l;
        let dark = label.resolve(true).l;

        assert!(light < 0.1, "label should be dark in light mode");
        assert!(dark > 0.9, "label should be light in dark mode");
    }

    #[test]
    fn semantic_background_adapts_to_color_scheme() {
        let bg = Color::system_background();

        let light = bg.resolve(false).l;
        let dark = bg.resolve(true).l;

        assert!(light > 0.9, "background should be light in light mode");
        assert!(dark < 0.1, "background should be dark in dark mode");
    }
}
