use gpui::{
    actions, div, prelude::*, px, size, App, Application, Bounds, Context, Entity, FocusHandle,
    Subscription, Window, WindowBounds, WindowOptions,
};
use gpui_component::theme::{ActiveTheme, Theme, ThemeMode};
use gpui_component::Root;

use allui::prelude::*;

mod sidebar;
mod stories;

use stories::Story;

actions!(storybook, [Quit, CloseWindow]);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemePreference {
    #[default]
    System,
    Light,
    Dark,
}

pub struct Storybook {
    selected_story: Story,
    theme_preference: ThemePreference,
    #[allow(dead_code)]
    appearance_subscription: Subscription,
    toggle_value: bool,
    tap_count: u32,
    show_content: bool,
    selected_fruit: Option<usize>,
    focus_handle: FocusHandle,
    text_input: Entity<InputState>,
    text_input_cleanable: Entity<InputState>,
    password_input: Entity<InputState>,
    text_editor_input: Entity<InputState>,
    slider_state: Entity<SliderState>,
    slider_value: f32,
    stepper_input: Entity<InputState>,
    stepper_value: i32,
}

impl Storybook {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name..."));
        let text_input_cleanable =
            cx.new(|cx| InputState::new(window, cx).placeholder("Type here to see X button..."));
        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter password...")
                .masked(true)
        });
        let text_editor_input = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .placeholder("Enter notes here...")
        });

        let slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(50.0)
                .step(1.0)
        });

        cx.subscribe(&slider_state, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            this.slider_value = value.start();
            cx.notify();
        })
        .detach();

        let stepper_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("5")
                .placeholder("Qty")
        });

        cx.subscribe(&stepper_input, |this, _, event: &StepperEvent, cx| {
            let StepperEvent::Step(action) = event;
            match action {
                StepAction::Increment => this.stepper_value += 1,
                StepAction::Decrement => this.stepper_value -= 1,
            }
            cx.notify();
        })
        .detach();

        let entity = cx.entity().clone();
        let appearance_subscription = window.observe_window_appearance(move |window, cx| {
            entity.update(cx, |this, cx| {
                if this.theme_preference == ThemePreference::System {
                    Theme::sync_system_appearance(Some(window), cx);
                    cx.notify();
                }
            });
        });

        Self {
            selected_story: Story::default(),
            theme_preference: ThemePreference::default(),
            appearance_subscription,
            toggle_value: false,
            tap_count: 0,
            show_content: true,
            selected_fruit: Some(0),
            focus_handle: cx.focus_handle(),
            text_input,
            text_input_cleanable,
            password_input,
            text_editor_input,
            slider_state,
            slider_value: 50.0_f32,
            stepper_input,
            stepper_value: 5,
        }
    }

    pub fn cycle_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.theme_preference = self.theme_preference.next();
        match self.theme_preference {
            ThemePreference::System => {
                Theme::sync_system_appearance(Some(window), cx);
            }
            ThemePreference::Light => {
                Theme::change(ThemeMode::Light, Some(window), cx);
            }
            ThemePreference::Dark => {
                Theme::change(ThemeMode::Dark, Some(window), cx);
            }
        }
        cx.notify();
    }

    fn render_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("content-scroll")
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .p_4()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(format!("{:?}", self.selected_story)),
            )
            .child(self.render_story(cx))
    }

    fn render_story(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        use stories::*;

        match self.selected_story {
            Story::VStack => render_vstack_story().into_any_element(),
            Story::HStack => render_hstack_story().into_any_element(),
            Story::ZStack => render_zstack_story().into_any_element(),
            Story::Spacer => render_spacer_story().into_any_element(),
            Story::Text => render_text_story().into_any_element(),
            Story::Button => render_button_story().into_any_element(),
            Story::Modifiers => render_modifiers_story().into_any_element(),
            Story::Toggle => render_toggle_story(self, cx).into_any_element(),
            Story::TapGesture => render_tap_gesture_story(self, cx).into_any_element(),
            Story::TextFields => render_textfields_story(
                &self.text_input,
                &self.text_input_cleanable,
                &self.password_input,
            )
            .into_any_element(),
            Story::Sliders => {
                render_sliders_story(&self.slider_state, self.slider_value).into_any_element()
            }
            Story::MoreInputs => render_more_inputs_story(
                &self.text_editor_input,
                &self.stepper_input,
                self.stepper_value,
            )
            .into_any_element(),
            Story::DisplayComponents => render_display_components_story().into_any_element(),
            Story::ScrollView => render_scrollview_story().into_any_element(),
            Story::List => render_list_story().into_any_element(),
            Story::ListConfig => render_list_config_story().into_any_element(),
            Story::ForEach => render_foreach_story().into_any_element(),
            Story::Conditional => render_conditional_story(self, cx).into_any_element(),
            Story::Grid => render_grid_story().into_any_element(),
            Story::LazyVGrid => render_lazy_vgrid_story().into_any_element(),
            Story::LazyHGrid => render_lazy_hgrid_story().into_any_element(),
            Story::BothAxesScroll => {
                render_both_axes_scroll_story(cx.theme().secondary).into_any_element()
            }
        }
    }
}

impl Render for Storybook {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, _window, cx| {
                cx.quit();
            })
            .on_action(|_: &CloseWindow, window, _cx| {
                window.remove_window();
            })
            .child(self.render_sidebar(cx))
            .child(self.render_content(cx))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(1000.0), px(700.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.bind_keys([
                    gpui::KeyBinding::new("cmd-q", Quit, None),
                    gpui::KeyBinding::new("cmd-w", CloseWindow, None),
                ]);

                let storybook = cx.new(|cx| {
                    let storybook = Storybook::new(window, cx);
                    storybook.focus_handle.focus(window);
                    storybook
                });

                cx.new(|cx| Root::new(storybook, window, cx))
            },
        )
        .unwrap();
    });
}
