//! List - Styled list container with sections.

use gpui::{
    div, px, AnyElement, App, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window,
};
use gpui_component::scroll::ScrollableElement;

use crate::modifier::Modifier;
use crate::style::Color;

use super::list_types::{
    EdgeInsets, EdgeInsetsExt, EdgeSet, ListConfiguration, ListSectionSpacing, RowConfiguration,
    SectionMargins,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListStyle {
    #[default]
    Automatic,
    Plain,
    InsetGrouped,
    Grouped,
    Sidebar,
}

impl ListStyle {
    pub fn automatic() -> Self {
        Self::Automatic
    }

    pub fn plain() -> Self {
        Self::Plain
    }

    pub fn inset_grouped() -> Self {
        Self::InsetGrouped
    }

    pub fn grouped() -> Self {
        Self::Grouped
    }

    pub fn sidebar() -> Self {
        Self::Sidebar
    }
}

enum ListChild {
    Section(Section),
    Element(AnyElement),
}

#[derive(IntoElement)]
pub struct List {
    id: SharedString,
    style: ListStyle,
    children: Vec<ListChild>,

    default_row_insets: Option<EdgeInsets>,
    default_row_spacing: Option<f32>,
    section_spacing: ListSectionSpacing,
    min_row_height: Option<f32>,
    min_header_height: Option<f32>,
}

impl List {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            style: ListStyle::Automatic,
            children: Vec::new(),
            default_row_insets: None,
            default_row_spacing: None,
            section_spacing: ListSectionSpacing::Default,
            min_row_height: None,
            min_header_height: None,
        }
    }

    pub fn list_style(mut self, style: ListStyle) -> Self {
        self.style = style;
        self
    }

    pub fn section(mut self, section: Section) -> Self {
        self.children.push(ListChild::Section(section));
        self
    }

    pub fn sections<I: IntoIterator<Item = Section>>(mut self, sections: I) -> Self {
        self.children
            .extend(sections.into_iter().map(ListChild::Section));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ListChild::Element(child.into_any_element()));
        self
    }

    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement,
    {
        self.children.extend(
            children
                .into_iter()
                .map(|c| ListChild::Element(c.into_any_element())),
        );
        self
    }

    pub fn list_row_insets(mut self, insets: impl Into<EdgeInsets>) -> Self {
        self.default_row_insets = Some(insets.into());
        self
    }

    pub fn list_row_spacing(mut self, spacing: f32) -> Self {
        self.default_row_spacing = Some(spacing);
        self
    }

    pub fn list_section_spacing(mut self, spacing: impl Into<ListSectionSpacing>) -> Self {
        self.section_spacing = spacing.into();
        self
    }

    pub fn min_row_height(mut self, height: f32) -> Self {
        self.min_row_height = Some(height);
        self
    }

    pub fn min_header_height(mut self, height: f32) -> Self {
        self.min_header_height = Some(height);
        self
    }

    fn build_configuration(&self) -> ListConfiguration {
        ListConfiguration {
            default_row_insets: self.default_row_insets,
            default_row_spacing: self.default_row_spacing,
            default_section_spacing: self.section_spacing,
            min_row_height: self.min_row_height,
            min_header_height: self.min_header_height,
            style: self.style,
        }
    }
}

impl Modifier for List {}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let config = self.build_configuration();
        let section_spacing = self.section_spacing.resolve(self.style);

        let mut base = div()
            .id(self.id)
            .size_full()
            .min_h_0()
            .flex()
            .flex_col()
            .gap(px(section_spacing));

        base = match self.style {
            ListStyle::Automatic | ListStyle::Plain => base,
            ListStyle::InsetGrouped | ListStyle::Grouped => base.p(px(16.0)),
            ListStyle::Sidebar => base.p(px(8.0)),
        };

        let children: Vec<AnyElement> = self
            .children
            .into_iter()
            .map(|child| match child {
                ListChild::Section(section) => {
                    section.with_list_config(config.clone()).into_any_element()
                }
                ListChild::Element(element) => element,
            })
            .collect();

        base.children(children).overflow_y_scrollbar()
    }
}

pub struct SectionRow {
    pub element: AnyElement,
    pub config: RowConfiguration,
}

#[derive(IntoElement)]
pub struct Section {
    header: Option<SharedString>,
    footer: Option<SharedString>,
    rows: Vec<SectionRow>,

    section_spacing_override: Option<ListSectionSpacing>,
    section_margins: Option<SectionMargins>,
    row_insets_override: Option<EdgeInsets>,
    row_spacing_override: Option<f32>,

    list_config: Option<ListConfiguration>,
}

impl Section {
    pub fn new() -> Self {
        Self {
            header: None,
            footer: None,
            rows: Vec::new(),
            section_spacing_override: None,
            section_margins: None,
            row_insets_override: None,
            row_spacing_override: None,
            list_config: None,
        }
    }

    pub fn header(mut self, text: impl Into<SharedString>) -> Self {
        self.header = Some(text.into());
        self
    }

    pub fn footer(mut self, text: impl Into<SharedString>) -> Self {
        self.footer = Some(text.into());
        self
    }

    pub fn row(mut self, element: impl IntoElement) -> Self {
        self.rows.push(SectionRow {
            element: element.into_any_element(),
            config: RowConfiguration::default(),
        });
        self
    }

    pub fn row_with_config(mut self, element: impl IntoElement, config: RowConfiguration) -> Self {
        self.rows.push(SectionRow {
            element: element.into_any_element(),
            config,
        });
        self
    }

    pub fn rows<I, E>(mut self, elements: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement,
    {
        self.rows.extend(elements.into_iter().map(|e| SectionRow {
            element: e.into_any_element(),
            config: RowConfiguration::default(),
        }));
        self
    }

    pub fn list_section_spacing(mut self, spacing: impl Into<ListSectionSpacing>) -> Self {
        self.section_spacing_override = Some(spacing.into());
        self
    }

    pub fn list_section_margins(mut self, edges: EdgeSet, length: Option<f32>) -> Self {
        self.section_margins = Some(SectionMargins::new(edges, length));
        self
    }

    pub fn list_section_margins_all(mut self, length: f32) -> Self {
        self.section_margins = Some(SectionMargins::all(length));
        self
    }

    pub fn list_row_insets(mut self, insets: impl Into<EdgeInsets>) -> Self {
        self.row_insets_override = Some(insets.into());
        self
    }

    pub fn list_row_spacing(mut self, spacing: f32) -> Self {
        self.row_spacing_override = Some(spacing);
        self
    }

    pub(crate) fn with_list_config(mut self, config: ListConfiguration) -> Self {
        self.list_config = Some(config);
        self
    }

    fn effective_row_insets(&self) -> EdgeInsets {
        self.row_insets_override
            .or(self.list_config.as_ref().and_then(|c| c.default_row_insets))
            .unwrap_or_else(|| EdgeInsets::init(0.0, 16.0, 0.0, 16.0))
    }

    fn effective_row_spacing(&self) -> f32 {
        self.row_spacing_override
            .or(self
                .list_config
                .as_ref()
                .and_then(|c| c.default_row_spacing))
            .unwrap_or(0.0)
    }

    fn effective_min_row_height(&self) -> f32 {
        self.list_config
            .as_ref()
            .and_then(|c| c.min_row_height)
            .unwrap_or(44.0)
    }

    fn effective_min_header_height(&self) -> Option<f32> {
        self.list_config.as_ref().and_then(|c| c.min_header_height)
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

impl Modifier for Section {}

impl RenderOnce for Section {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        use gpui_component::ActiveTheme;

        let is_dark = cx.theme().is_dark();

        let default_row_insets = self.effective_row_insets();
        let row_spacing = self.effective_row_spacing();
        let min_row_height = self.effective_min_row_height();
        let min_header_height = self.effective_min_header_height();

        let label_color = Color::secondary_label().resolve(is_dark);
        let bg_color = Color::tertiary_system_background().resolve(is_dark);
        let separator_color = Color::separator().resolve(is_dark);

        let mut section = div().flex().flex_col().w_full();

        if let Some(margins) = self.section_margins {
            if let Some(length) = margins.length {
                if margins.edges.top {
                    section = section.pt(px(length));
                }
                if margins.edges.bottom {
                    section = section.pb(px(length));
                }
                if margins.edges.leading {
                    section = section.pl(px(length));
                }
                if margins.edges.trailing {
                    section = section.pr(px(length));
                }
            }
        }

        if let Some(header_text) = self.header {
            let mut header = div()
                .text_xs()
                .text_color(label_color)
                .px(px(16.0))
                .pb(px(8.0))
                .child(header_text.to_uppercase());

            if let Some(min_h) = min_header_height {
                header = header.min_h(px(min_h));
            }

            section = section.child(header);
        }

        let mut content = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(bg_color)
            .rounded(px(10.0))
            .overflow_hidden();

        if row_spacing > 0.0 {
            content = content.gap(px(row_spacing));
        }

        let row_count = self.rows.len();
        let row_elements: Vec<_> = self
            .rows
            .into_iter()
            .enumerate()
            .map(|(index, row)| {
                let is_last = index == row_count - 1;
                let effective_insets = row.config.insets.unwrap_or(default_row_insets);

                let row_div = div()
                    .flex()
                    .items_center()
                    .min_h(px(min_row_height))
                    .pt(px(effective_insets.top))
                    .pb(px(effective_insets.bottom))
                    .pl(px(effective_insets.leading))
                    .pr(px(effective_insets.trailing))
                    .child(row.element);

                if !is_last && row_spacing == 0.0 {
                    row_div.border_b_1().border_color(separator_color)
                } else {
                    row_div
                }
            })
            .collect();

        section = section.child(content.children(row_elements));

        if let Some(footer_text) = self.footer {
            section = section.child(
                div()
                    .text_xs()
                    .text_color(label_color)
                    .px(px(16.0))
                    .pt(px(8.0))
                    .child(footer_text),
            );
        }

        section
    }
}
