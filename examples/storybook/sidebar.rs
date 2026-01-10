use gpui::{div, prelude::*, px, Context, SharedString};
use gpui_component::scroll::ScrollableElement;
use gpui_component::theme::ActiveTheme;
use allui::prelude::*;

use crate::stories::{component_stories, container_stories, grid_stories, layout_stories, Story};
use crate::{Storybook, ThemePreference};

impl Storybook {
    pub fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let sidebar_bg = theme.sidebar;
        let border_color = theme.sidebar_border;
        let muted_fg = theme.muted_foreground;

        div()
            .flex()
            .flex_col()
            .w(px(200.0))
            .h_full()
            .min_h_0()
            .bg(sidebar_bg)
            .border_r_1()
            .border_color(border_color)
            .child(
                div()
                    .p_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Allui Storybook"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .pt_2()
                            .pb_2()
                            .mb_2()
                            .border_b_1()
                            .border_color(border_color)
                            .child(
                                Text::new("Theme:")
                                    .font(Font::caption())
                                    .foreground_color(Color::secondary_label()),
                            )
                            .child(self.render_theme_toggle(cx)),
                    ),
            )
            .child(
                div()
                    .id("sidebar-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .p_2()
                    .pt_0()
                    .gap_1()
                    .flex()
                    .flex_col()
                    .child(self.render_section_header("Layout", muted_fg))
                    .children(
                        layout_stories()
                            .iter()
                            .map(|info| self.render_sidebar_item(info.name, info.story, cx)),
                    )
                    .child(self.render_section_header("Components", muted_fg))
                    .children(
                        component_stories()
                            .iter()
                            .map(|info| self.render_sidebar_item(info.name, info.story, cx)),
                    )
                    .child(self.render_section_header("Containers", muted_fg))
                    .children(
                        container_stories()
                            .iter()
                            .map(|info| self.render_sidebar_item(info.name, info.story, cx)),
                    )
                    .child(self.render_section_header("Grids", muted_fg))
                    .children(
                        grid_stories()
                            .iter()
                            .map(|info| self.render_sidebar_item(info.name, info.story, cx)),
                    ),
            )
    }

    fn render_theme_toggle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        let secondary_bg = cx.theme().secondary;
        let secondary_hover = cx.theme().secondary_hover;

        div()
            .id("theme-toggle")
            .cursor_pointer()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(secondary_bg)
            .hover(|d| d.bg(secondary_hover))
            .child(Text::new(self.theme_preference.label()).font(Font::caption()))
            .on_click(move |_, window, cx| {
                entity.update(cx, |this, cx| {
                    this.cycle_theme(window, cx);
                });
            })
    }

    fn render_section_header(&self, title: &'static str, muted_fg: gpui::Hsla) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(muted_fg)
            .mt_3()
            .mb_1()
            .px_2()
            .child(title.to_uppercase())
    }

    fn render_sidebar_item(
        &self,
        name: &'static str,
        story: Story,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.selected_story == story;
        let theme = cx.theme();
        let selection_bg = theme.selection;
        let hover_bg = theme.list_hover;

        div()
            .id(SharedString::from(name))
            .cursor_pointer()
            .px_3()
            .py_1()
            .rounded_md()
            .when(is_selected, |d| d.bg(selection_bg))
            .hover(|d| d.bg(hover_bg))
            .on_click(cx.listener(move |this, _, _, _cx| {
                this.selected_story = story;
            }))
            .child(name)
    }
}

impl ThemePreference {
    pub fn next(self) -> Self {
        match self {
            Self::System => Self::Light,
            Self::Light => Self::Dark,
            Self::Dark => Self::System,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }
}
