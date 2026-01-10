//! Font types for Allui.

/// Font configuration for text rendering.
#[derive(Clone, Debug)]
pub struct Font {
    /// Font size in points.
    pub size: Option<f32>,
    /// Font weight.
    pub weight: FontWeight,
    /// Font design (default, serif, monospace, rounded).
    pub design: FontDesign,
    /// Whether the font is italic.
    pub italic: bool,
}

impl Font {
    /// Create a new font with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a font with a specific size.
    pub fn size(size: f32) -> Self {
        Self {
            size: Some(size),
            ..Default::default()
        }
    }

    // SwiftUI text styles

    /// Large title style (~34pt).
    pub fn large_title() -> Self {
        Self {
            size: Some(34.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Title style (~28pt).
    pub fn title() -> Self {
        Self {
            size: Some(28.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Title 2 style (~22pt).
    pub fn title2() -> Self {
        Self {
            size: Some(22.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Title 3 style (~20pt).
    pub fn title3() -> Self {
        Self {
            size: Some(20.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Headline style (~17pt semibold).
    pub fn headline() -> Self {
        Self {
            size: Some(17.0),
            weight: FontWeight::Semibold,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Subheadline style (~15pt).
    pub fn subheadline() -> Self {
        Self {
            size: Some(15.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Body style (~17pt).
    pub fn body() -> Self {
        Self {
            size: Some(17.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Callout style (~16pt).
    pub fn callout() -> Self {
        Self {
            size: Some(16.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Footnote style (~13pt).
    pub fn footnote() -> Self {
        Self {
            size: Some(13.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Caption style (~12pt).
    pub fn caption() -> Self {
        Self {
            size: Some(12.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    /// Caption 2 style (~11pt).
    pub fn caption2() -> Self {
        Self {
            size: Some(11.0),
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }

    // Modifiers

    /// Set the font weight.
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Set the font design.
    pub fn design(mut self, design: FontDesign) -> Self {
        self.design = design;
        self
    }

    /// Make the font bold.
    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::Bold;
        self
    }

    /// Make the font italic.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Make the font monospaced.
    pub fn monospaced(mut self) -> Self {
        self.design = FontDesign::Monospaced;
        self
    }
}

impl Default for Font {
    fn default() -> Self {
        Self {
            size: None, // Use system default
            weight: FontWeight::Regular,
            design: FontDesign::Default,
            italic: false,
        }
    }
}

/// Font weight options.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontWeight {
    UltraLight,
    Thin,
    Light,
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
    Heavy,
    Black,
}

impl FontWeight {
    /// Convert to GPUI font weight value.
    pub fn to_gpui(self) -> gpui::FontWeight {
        match self {
            FontWeight::UltraLight => gpui::FontWeight::EXTRA_LIGHT,
            FontWeight::Thin => gpui::FontWeight::THIN,
            FontWeight::Light => gpui::FontWeight::LIGHT,
            FontWeight::Regular => gpui::FontWeight::NORMAL,
            FontWeight::Medium => gpui::FontWeight::MEDIUM,
            FontWeight::Semibold => gpui::FontWeight::SEMIBOLD,
            FontWeight::Bold => gpui::FontWeight::BOLD,
            FontWeight::Heavy => gpui::FontWeight::EXTRA_BOLD,
            FontWeight::Black => gpui::FontWeight::BLACK,
        }
    }
}

/// Font design options.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontDesign {
    /// The default system font design.
    #[default]
    Default,
    /// A serif font design.
    Serif,
    /// A monospaced font design.
    Monospaced,
    /// A rounded font design.
    Rounded,
}

impl FontDesign {
    /// Returns the platform-specific font family name for this design.
    pub fn font_family(&self) -> Option<&'static str> {
        match self {
            FontDesign::Default => None,
            FontDesign::Monospaced => Some(Self::monospace_family()),
            FontDesign::Serif => Some(Self::serif_family()),
            FontDesign::Rounded => Some(Self::rounded_family()),
        }
    }

    #[cfg(target_os = "macos")]
    fn monospace_family() -> &'static str {
        "SF Mono"
    }
    #[cfg(target_os = "macos")]
    fn serif_family() -> &'static str {
        "New York"
    }
    #[cfg(target_os = "macos")]
    fn rounded_family() -> &'static str {
        "SF Pro Rounded"
    }

    #[cfg(target_os = "windows")]
    fn monospace_family() -> &'static str {
        "Cascadia Mono"
    }
    #[cfg(target_os = "windows")]
    fn serif_family() -> &'static str {
        "Georgia"
    }
    #[cfg(target_os = "windows")]
    fn rounded_family() -> &'static str {
        "Segoe UI"
    }

    #[cfg(target_os = "linux")]
    fn monospace_family() -> &'static str {
        "monospace"
    }
    #[cfg(target_os = "linux")]
    fn serif_family() -> &'static str {
        "serif"
    }
    #[cfg(target_os = "linux")]
    fn rounded_family() -> &'static str {
        "sans-serif"
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    fn monospace_family() -> &'static str {
        "monospace"
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    fn serif_family() -> &'static str {
        "serif"
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    fn rounded_family() -> &'static str {
        "sans-serif"
    }
}
