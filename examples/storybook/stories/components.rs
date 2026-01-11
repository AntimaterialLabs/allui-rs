use allui::prelude::*;
use gpui::{prelude::*, Context, Entity};

use crate::Storybook;

pub fn render_text_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Text Styles").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Large Title").font(Font::large_title()))
                .child(Text::new("Title").font(Font::title()))
                .child(Text::new("Headline").font(Font::headline()))
                .child(Text::new("Body text").font(Font::body()))
                .child(
                    Text::new("Caption")
                        .font(Font::caption())
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Font Variations").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Bold text").bold())
                .child(Text::new("Italic text").italic())
                .child(Text::new("Bold + Italic").bold().italic())
                .child(Text::new("Colored text").foreground_color(Color::blue()))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Font Design").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Default: The quick brown fox"))
                .child(
                    Text::new("Monospaced: let x = 42;")
                        .font(Font::body().monospaced()),
                )
                .child(
                    Text::new("Serif: Classical typography")
                        .font(Font::body().design(FontDesign::Serif)),
                )
                .child(
                    Text::new("Rounded: Friendly appearance")
                        .font(Font::body().design(FontDesign::Rounded)),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Text Decorations").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Normal text"))
                .child(Text::new("Strikethrough text").strikethrough(true))
                .child(
                    Text::new("Strikethrough + Red")
                        .strikethrough(true)
                        .foreground_color(Color::red()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Line Limit").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .alignment(HorizontalAlignment::Leading)
                        .child(Text::new(".line_limit(1):").foreground_color(Color::gray()))
                        .child(
                            Text::new("This is a very long text that should be truncated to a single line when line_limit is set to 1.")
                                .line_limit(1)
                                .frame_width(300.0)
                        ),
                )
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .alignment(HorizontalAlignment::Leading)
                        .child(Text::new(".line_limit(2):").foreground_color(Color::gray()))
                        .child(
                            Text::new("This is a very long text that demonstrates the line_limit modifier. It should wrap to two lines maximum and then truncate any remaining content.")
                                .line_limit(2)
                                .frame_width(300.0)
                        ),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}

pub fn render_button_story() -> impl IntoElement {
    VStack::new()
        .spacing(12.0)
        .alignment(HorizontalAlignment::Leading)
        .child(
            Button::new("Bordered Prominent", || {
                println!("Bordered Prominent clicked!");
            })
            .button_style(ButtonStyle::BorderedProminent),
        )
        .child(
            Button::new("Bordered", || {
                println!("Bordered clicked!");
            })
            .button_style(ButtonStyle::Bordered),
        )
        .child(
            Button::new("Plain", || {
                println!("Plain clicked!");
            })
            .button_style(ButtonStyle::Plain),
        )
        .child(
            Button::new("Disabled", || {})
                .button_style(ButtonStyle::BorderedProminent)
                .disabled(true),
        )
}

pub fn render_modifiers_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Modifier order matters:"))
        .child(
            HStack::new()
                .spacing(32.0)
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(Text::new(".padding().background()"))
                        .child(Text::new("Hello").padding(16.0).background(Color::red())),
                )
                .child(
                    VStack::new()
                        .spacing(8.0)
                        .child(Text::new(".background().padding()"))
                        .child(Text::new("Hello").background(Color::red()).padding(16.0)),
                ),
        )
        .child(Text::new("Chained modifiers:"))
        .child(
            Text::new("Styled Text")
                .padding(12.0)
                .background(Color::blue())
                .corner_radius(8.0)
                .padding(4.0)
                .background(Color::green())
                .corner_radius(12.0),
        )
}

pub fn render_toggle_story(storybook: &Storybook, cx: &mut Context<Storybook>) -> impl IntoElement {
    let toggle_value = storybook.toggle_value;

    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Toggle - Boolean switch component:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .child(
                    HStack::new()
                        .spacing(12.0)
                        .child(Toggle::new_with_handler(
                            "Dark Mode",
                            toggle_value,
                            cx.listener(|this: &mut Storybook, checked: &bool, _window, cx| {
                                this.toggle_value = *checked;
                                cx.notify();
                            }),
                        ))
                        .child(
                            Text::new(if toggle_value { "ON" } else { "OFF" }).foreground_color(
                                if toggle_value {
                                    Color::green()
                                } else {
                                    Color::gray()
                                },
                            ),
                        ),
                )
                .child(Toggle::new("Disabled Toggle", true, |_| {}).disabled(true))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Note: Toggle wraps gpui-component's Switch.")
                .foreground_color(Color::gray()),
        )
}

pub fn render_tap_gesture_story(
    storybook: &Storybook,
    cx: &mut Context<Storybook>,
) -> impl IntoElement {
    let tap_count = storybook.tap_count;
    let entity = cx.entity().clone();

    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("on_tap_gesture - Add tap handlers to any view:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .child(
                    Text::new("Tap me!")
                        .padding(16.0)
                        .background(Color::blue())
                        .corner_radius(8.0)
                        .on_tap_gesture_with("tap-me-button", move |_event, _window, cx| {
                            entity.update(cx, |this, cx| {
                                this.tap_count += 1;
                                cx.notify();
                            });
                        }),
                )
                .child(
                    Text::new(format!("Tap count: {}", tap_count)).foreground_color(Color::green()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Any view can be made tappable with .on_tap_gesture()")
                .foreground_color(Color::gray()),
        )
}

pub fn render_textfields_story(
    text_input: &Entity<InputState>,
    text_input_cleanable: &Entity<InputState>,
    password_input: &Entity<InputState>,
) -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("TextField - Single-line text input:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Basic TextField:").foreground_color(Color::gray()))
                .child(TextField::new(text_input).frame_width(300.0))
                .child(
                    Text::new("TextField with cleanable (x button):")
                        .foreground_color(Color::gray()),
                )
                .child(
                    TextField::new(text_input_cleanable)
                        .cleanable(true)
                        .frame_width(300.0),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("SecureField - Password input (masked):"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    SecureField::new(password_input)
                        .show_toggle(true)
                        .frame_width(300.0),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Note: State is managed via Entity<InputState>")
                .foreground_color(Color::gray()),
        )
}

pub fn render_sliders_story(
    slider_state: &Entity<SliderState>,
    slider_value: f32,
) -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Slider - Range value selection:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Horizontal slider:").foreground_color(Color::gray()))
                .child(
                    HStack::new()
                        .spacing(16.0)
                        .child(Slider::new(slider_state).frame_width(200.0))
                        .child(
                            Text::new(format!("{:.0}", slider_value))
                                .foreground_color(Color::green()),
                        ),
                )
                .child(
                    Text::new(format!("Current value: {:.1}", slider_value))
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(
            Text::new("Note: Subscribe to SliderEvent for value changes")
                .foreground_color(Color::gray()),
        )
}

pub fn render_more_inputs_story(
    text_editor_input: &Entity<InputState>,
    stepper_input: &Entity<InputState>,
    stepper_value: i32,
) -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("TextEditor - Multi-line text input:"))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    TextEditor::new(text_editor_input)
                        .height(150.0)
                        .frame_width(300.0),
                )
                .child(
                    Text::new("Note: Create InputState with .multi_line(true)")
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Stepper - Increment/decrement control:"))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    HStack::new()
                        .spacing(16.0)
                        .child(Text::new(format!("Value: {}", stepper_value)))
                        .child(Stepper::new(stepper_input)),
                )
                .child(
                    Text::new("Stepper triggers increment/decrement callbacks")
                        .foreground_color(Color::gray()),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Picker - Selection from options:"))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    Text::new(
                        "Picker requires PickerState + PickerDelegate. See picker.rs for usage.",
                    )
                    .foreground_color(Color::gray()),
                )
                .child(
                    VStack::new()
                        .spacing(4.0)
                        .child(Text::new("Usage:").bold())
                        .child(
                            Text::new("let state = PickerState::new(...);")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .child(
                            Text::new("Picker::new(\"id\", &state).build(window, cx)")
                                .font(Font::caption())
                                .foreground_color(Color::tertiary_label()),
                        )
                        .padding(12.0)
                        .background(Color::secondary_system_background())
                        .corner_radius(8.0),
                )
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}

pub fn render_display_components_story() -> impl IntoElement {
    VStack::new()
        .spacing(16.0)
        .alignment(HorizontalAlignment::Leading)
        .child(Text::new("Divider").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Item above"))
                .child(Divider::new())
                .child(Text::new("Item below"))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0)
                .frame_width(300.0),
        )
        .child(Text::new("Label").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Label::new("star.fill", "Favorites"))
                .child(Label::new("folder", "Documents"))
                .child(Label::new("gear", "Settings"))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Link").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Link::new("Visit Allui", || {
                    println!("Link clicked: Visit Allui");
                }))
                .child(Link::new("Documentation", || {
                    println!("Link clicked: Documentation");
                }))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("ProgressView").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(12.0)
                .alignment(HorizontalAlignment::Leading)
                .child(Text::new("Linear (determinate):").foreground_color(Color::gray()))
                .child(
                    ProgressView::new()
                        .progress_view_style(ProgressViewStyle::Linear)
                        .value(0.65)
                        .frame_width(200.0),
                )
                .child(Text::new("Circular (indeterminate):").foreground_color(Color::gray()))
                .child(ProgressView::new().progress_view_style(ProgressViewStyle::Circular))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
        .child(Text::new("Image").font(Font::headline()))
        .child(
            VStack::new()
                .spacing(8.0)
                .alignment(HorizontalAlignment::Leading)
                .child(
                    Text::new("Note: Image currently renders placeholder text.")
                        .foreground_color(Color::gray()),
                )
                .child(Image::new("photo.jpg").frame_size(100.0, 100.0))
                .padding(16.0)
                .background(Color::tertiary_system_background())
                .corner_radius(8.0),
        )
}
