# AGENTS.md

> AI Agent Guide for Allui Development

This document provides comprehensive guidance for AI coding agents working on Allui, a SwiftUI-inspired declarative UI framework for Rust built on GPUI and gpui-component.

---

## Table of Contents

- [Project Overview](#project-overview)
- [Architecture & Design Philosophy](#architecture--design-philosophy)
- [Build & Development](#build--development)
- [Code Organization](#code-organization)
- [Critical GPUI Patterns](#critical-gpui-patterns)
- [Component Development](#component-development)
- [Modifier System](#modifier-system)
- [State Management](#state-management)
- [Testing](#testing)
- [Code Style & Conventions](#code-style--conventions)
- [Known Limitations & TODOs](#known-limitations--todos)
- [Common Pitfalls](#common-pitfalls)

---

## Project Overview

### What is Allui?

Allui brings SwiftUI's declarative, composable API to Rust desktop applications. If you know SwiftUI, you already know Allui.

**Key Stats:**
- **Lines of Code:** ~8,000+ across 46 files (39 src, 7 storybook)
- **Dependencies:** GPUI 0.2, gpui-component 0.5
- **Status:** Alpha
- **Platform:** macOS, Linux, Windows (wherever GPUI runs)

### Three-Layer Architecture

```
┌─────────────────────────────────────────┐
│        User Application Code            │
├─────────────────────────────────────────┤
│              Allui                      │
│  ┌──────────────┬───────────────────┐   │
│  │   Layout     │    Components     │   │
│  │  Primitives  │  (Display+Input)  │   │
│  ├──────────────┼───────────────────┤   │
│  │   Modifier   │   Control Flow    │   │
│  │   System     │   & Containers    │   │
│  └──────────────┴───────────────────┘   │
├─────────────────────────────────────────┤
│         gpui-component (v0.5.0)         │
│    (Stateful widgets: Switch, Input)    │
├─────────────────────────────────────────┤
│           GPUI (v0.2)                   │
│     (GPU-accelerated rendering)         │
└─────────────────────────────────────────┘
```

**Allui** provides SwiftUI API patterns and semantics
**gpui-component** provides stateful widgets (Switch, Input, Slider, etc.)
**GPUI** provides GPU-accelerated rendering and reactive state management

---

## Architecture & Design Philosophy

### Core Principles

1. **SwiftUI API Parity**
   - Names, methods, and behaviors match SwiftUI exactly
   - Layout behavior must be indistinguishable from SwiftUI
   - Modifiers have the same semantics (order matters)

2. **Presentational Components**
   - All Allui components are stateless
   - State lives in GPUI views (user's `impl Render` types)
   - Components render based on props passed to them

3. **True Modifier Wrapping**
   - Each modifier creates a wrapper `div()` container
   - `.padding().background()` ≠ `.background().padding()`
   - Matches SwiftUI's compositional model exactly

4. **Hybrid Component Strategy**
   - **Layout primitives:** Built directly on GPUI for precise control
   - **Display components:** Thin wrappers around GPUI primitives
   - **Input components:** Wrap gpui-component's stateful widgets

### Design Decisions

**Why no `@State` or `@Binding` equivalents?**
- Avoids reinventing GPUI's proven state system
- SwiftUI's property wrappers require compiler magic Rust doesn't support well
- More verbose but more explicit and debuggable
- Users already understand `Entity`, `Context`, `cx.notify()`

**Why wrapper `div()` for each modifier?**
- Exact SwiftUI semantics (order matters)
- Predictable behavior
- Matches user mental model from SwiftUI
- GPUI's GPU rendering makes nested divs efficient

**Why stable element IDs required?**
- GPUI's event system requires IDs to persist between renders
- Automatic ID generation breaks click handling
- Explicit > implicit for debugging

---

## Build & Development

### Setup

```bash
# Install dependencies
cargo build

# Run interactive component showcase
cargo run --example storybook

# Run with release optimizations (recommended for UI testing)
cargo run --example storybook --release
```

### Project Structure

```
allui/
├── src/
│   ├── lib.rs              # Crate root, public exports
│   ├── prelude.rs          # Convenient re-exports (use allui::prelude::*)
│   ├── types.rs            # Type aliases (ClickHandler)
│   ├── alignment.rs        # Alignment types with helper methods
│   ├── modifier.rs         # Modifier trait + implementations
│   ├── components/         # UI components
│   │   ├── mod.rs
│   │   ├── text.rs         # Display: Text
│   │   ├── button.rs       # Display: Button
│   │   ├── divider.rs      # Display: Divider
│   │   ├── image.rs        # Display: Image (TODO: actual loading)
│   │   ├── label.rs        # Display: Label
│   │   ├── link.rs         # Display: Link
│   │   ├── progress_view.rs # Display: ProgressView (TODO: animation)
│   │   ├── toggle.rs       # Input: Toggle
│   │   ├── text_field.rs   # Input: TextField
│   │   ├── secure_field.rs # Input: SecureField
│   │   ├── text_editor.rs  # Input: TextEditor
│   │   ├── slider.rs       # Input: Slider
│   │   ├── stepper.rs      # Input: Stepper
│   │   └── picker.rs       # Input: Picker
│   ├── layout/             # Layout primitives & containers
│   │   ├── mod.rs
│   │   ├── vstack.rs       # Vertical stack
│   │   ├── hstack.rs       # Horizontal stack
│   │   ├── zstack.rs       # Overlay stack
│   │   ├── spacer.rs       # Flexible space
│   │   ├── group.rs        # Transparent container
│   │   ├── empty_view.rs   # Empty placeholder
│   │   ├── scroll_view.rs  # Scrollable container
│   │   ├── list.rs         # Sectioned list (iOS-style)
│   │   ├── control_flow.rs # ForEach, If, IfLet
│   │   ├── lazy_stack.rs   # LazyVStack, LazyHStack (virtualized)
│   │   ├── children_macro.rs # Internal macro for child collection
│   │   ├── grid_item.rs    # GridItem, GridItemSize
│   │   ├── grid.rs         # Grid, GridRow (static 2D layout)
│   │   ├── lazy_vgrid.rs   # LazyVGrid (virtualized vertical grid)
│   │   └── lazy_hgrid.rs   # LazyHGrid (virtualized horizontal grid)
│   └── style/              # Style types
│       ├── mod.rs
│       ├── color.rs        # Color type + semantic colors
│       └── font.rs         # Font types + text styles
├── examples/
│   └── storybook/          # Interactive showcase (7 files, ~2150 lines, 17 stories)
│       ├── main.rs         # Main entry, window setup, theme management
│       ├── sidebar.rs      # Sidebar rendering and navigation
│       └── stories/        # Story modules by category
│           ├── mod.rs      # Story enum and metadata
│           ├── layout.rs   # Stack, HStack, ZStack, Spacer
│           ├── components.rs # Text, Button, Modifiers, Toggle, etc.
│           ├── containers.rs # ScrollView, List, ForEach, Conditional
│           └── grids.rs    # Grids - Grid, LazyVGrid, LazyHGrid
├── Cargo.toml
├── README.md               # User-facing documentation
└── AGENTS.md               # This file
```

### Important Files for Common Tasks

**Adding a new component:**
- Create `src/components/my_component.rs`
- Add to `src/components/mod.rs` exports
- Add to `src/prelude.rs` if commonly used
- Add story to appropriate file in `examples/storybook/stories/`

**Adding a new layout primitive:**
- Create `src/layout/my_layout.rs`
- Add to `src/layout/mod.rs` exports
- Add to `src/prelude.rs`
- Add story to `examples/storybook/stories/layout.rs`

**Adding a new modifier:**
- Add variant to `ModifierKind` enum in `src/modifier.rs`
- Add method to `Modifier` trait
- Implement rendering in `ModifiedElement::render()`
- Add demo to `examples/storybook/stories/components.rs` (Modifiers story)

---

## Code Organization

### Module Structure

```
allui::
├── layout::               # Layout primitives
│   ├── VStack, HStack, ZStack
│   ├── Spacer, EmptyView, Group
│   ├── ScrollView, List, Section
│   ├── ForEach, If, IfLet, LazyVStack, LazyHStack
│   └── Grid, GridRow, GridItem, LazyVGrid, LazyHGrid
├── components::           # UI components
│   ├── Text, Button, Divider, Image, Label, Link, ProgressView
│   └── Toggle, TextField, SecureField, TextEditor, Slider, Stepper, Picker
├── style::                # Style types
│   ├── Color, Font
│   ├── Padding, Frame, Alignment
│   └── ButtonStyle, ListStyle, etc.
├── Modifier              # Modifier trait (implemented by all view types)
└── prelude::             # Common exports
```

### Naming Conventions

**Follow SwiftUI exactly:**
- Types: `VStack`, `HStack`, `TextField` (not `VBox`, `TextInput`)
- Methods: `padding()`, `background()`, `foreground_color()` (not `set_padding()`)
- Parameters: `alignment`, `spacing`, `axes` (match SwiftUI names)

**GPUI terminology for internals:**
- `RenderOnce` trait implementation
- `IntoElement` for element conversion
- `div()` as base container
- `Entity<T>` for stateful types

**Type aliases for complex types:**
```rust
pub type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
```

---

## Critical GPUI Patterns

### 1. Element IDs Must Be Stable Across Renders

**CRITICAL:** GPUI's `on_click` handler requires that the element has the same ID between mouse-down and mouse-up events. If the element gets a new ID on each render, clicks will never register.

**Problem:**
```rust
// ❌ WRONG: ID changes every render
static COUNTER: AtomicU64 = AtomicU64::new(0);
fn render() {
    let id = COUNTER.fetch_add(1, ...);  // New ID each render!
    div().id(format!("btn-{}", id)).on_click(...)
}
```

**Symptom:** `on_mouse_down` fires but `on_click` never fires.

**Root Cause:** GPUI re-renders views frequently (on mouse move, state changes, etc.). Dynamic IDs create a "new" element each time.

**Solution:**
```rust
// ✅ CORRECT: Stable ID provided by user
Text::new("Click me")
    .on_tap_gesture("my-button-id", || {
        println!("Clicked!");
    })
```

Element IDs must be:
1. **User-provided** (recommended) - e.g., `.on_tap_gesture("my-button-id", handler)`
2. **Or derived deterministically** from content/position (not from render-time counters)

### 2. gpui-component Requires Initialization

Before using any gpui-component widgets (Toggle, TextField, Slider, etc.), you must call:

```rust
fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);  // ✅ Required!
        // ... rest of your app
    });
}
```

This sets up the theme state. **Without it, components will panic** with "no state of type Theme exists".

### 3. State Updates Require `cx.notify()`

GPUI uses a reactive model. After modifying state, you must call `cx.notify()` to trigger a re-render:

```rust
// ✅ CORRECT: Notify after state change
Button::new("Increment", cx.listener(|this, _, _, cx| {
    this.count += 1;
    cx.notify();  // Required to see the change!
}))
```

```rust
// ❌ WRONG: Missing cx.notify()
Button::new("Increment", cx.listener(|this, _, _, cx| {
    this.count += 1;
    // View won't update!
}))
```

### 4. Stateful Input Components Require `Entity<State>`

gpui-component's input widgets (Input, Slider, Select, etc.) require `Entity<State>`:

**Pattern:**
```rust
struct MyView {
    email_input: Entity<InputState>,
    volume_slider: Entity<SliderState>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Create state entities in constructor
        let email_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Email")
                .cleanable(true)
        });

        let volume_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(50.0)
        });

        // Subscribe to changes
        cx.subscribe(&volume_slider, |this, _, event: &SliderEvent, cx| {
            if let SliderEvent::Change(value) = event {
                this.volume = value.start();
                cx.notify();
            }
        });

        Self { email_input, volume_slider }
    }
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .child(TextField::new(&self.email_input))
            .child(Slider::new(&self.volume_slider))
    }
}
```

**Why:** gpui-component widgets manage internal state (cursor position, slider handle, etc.). The Entity pattern allows observation without ownership transfer.

---

## Component Development

### Component Types

**1. Display Components** (Stateless, presentational)
- Text, Button, Divider, Image, Label, Link, ProgressView
- Render based on props
- No internal state
- Examples: `src/components/text.rs`, `src/components/button.rs`

**2. Input Components** (Wrap gpui-component)
- Toggle, TextField, SecureField, TextEditor, Slider, Stepper, Picker
- Require `Entity<State>` for state management
- Emit events via subscriptions
- Examples: `src/components/toggle.rs`, `src/components/slider.rs`

**3. Layout Primitives** (Composition)
- VStack, HStack, ZStack, Spacer, Group, EmptyView
- Accept children via `.child()` or `.children()`
- Control layout behavior (spacing, alignment)
- Examples: `src/layout/vstack.rs`, `src/layout/hstack.rs`

**4. Containers** (Scrolling, lists)
- ScrollView, List, Section, LazyVStack, LazyHStack
- Manage large collections efficiently
- Examples: `src/layout/scroll_view.rs`, `src/layout/lazy_stack.rs`

**5. Grid Components** (2D layouts)
- Grid, GridRow for static table layouts
- LazyVGrid, LazyHGrid for virtualized grids
- GridItem for column/row sizing (Fixed, Flexible, Adaptive)
- Examples: `src/layout/grid.rs`, `src/layout/lazy_vgrid.rs`

### Creating a New Display Component

```rust
// src/components/my_component.rs
use gpui::prelude::*;
use crate::modifier::Modifier;

pub struct MyComponent {
    title: String,
    // ... other props
}

impl MyComponent {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    // Builder methods for configuration
    pub fn some_option(mut self, value: bool) -> Self {
        self.some_option = value;
        self
    }
}

impl RenderOnce for MyComponent {
    fn render(self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            // ... styling and children
    }
}

impl IntoElement for MyComponent {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        div().child(self)
    }
}

// Make it modifiable
impl Modifier for MyComponent {}
```

### Creating a New Layout Primitive

```rust
// src/layout/my_stack.rs
use gpui::prelude::*;
use crate::modifier::Modifier;

pub struct MyStack {
    spacing: f32,
    children: Vec<AnyElement>,
}

impl MyStack {
    pub fn new() -> Self {
        Self {
            spacing: 0.0,
            children: vec![],
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement,
    {
        self.children.extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl RenderOnce for MyStack {
    fn render(self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()  // or flex_row() for horizontal
            .gap(px(self.spacing))
            .children(self.children)
    }
}

impl IntoElement for MyStack {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        div().child(self)
    }
}

impl Modifier for MyStack {}
```

### Creating a New Input Component

```rust
// src/components/my_input.rs
use gpui::prelude::*;
use gpui_component::input::{Input, InputState};
use crate::modifier::Modifier;

pub struct MyInput {
    state: Entity<InputState>,
    label: Option<String>,
}

impl MyInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            label: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for MyInput {
    fn render(self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .when_some(self.label, |this, label| {
                this.child(div().child(label))
            })
            .child(Input::new(window, cx).state(&self.state))
    }
}

impl IntoElement for MyInput {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        div().child(self)
    }
}

impl Modifier for MyInput {}
```

---

## Modifier System

### How Modifiers Work

The modifier system is Allui's architectural cornerstone. Each modifier wraps the view in a container `div()`, making order semantically meaningful.

**Implementation:**
- `Modifier` trait provides all modifier methods
- `Modified<V>` wrapper holds child view + modifier kind
- `ModifierKind` enum has 13 variants for different modifiers
- `Tappable<V>` wrapper for tap gesture handlers

### Modifier Categories

**Layout Modifiers:**
```rust
.padding(16.0)                          // Inset content
.frame(Frame::size(100.0, 50.0))        // Fixed width and height
.frame_size(100.0, 50.0)                // Shorthand for Frame::size()
.frame_width(200.0)                     // Fixed width only
.frame_height(100.0)                    // Fixed height only
.frame(Frame::fill_width())             // Expand to fill available width
.frame(Frame::new().min_width(100.0).max_width(300.0))  // Flexible constraints
.fixed_size(true, true)                 // Prevent compression
.aspect_ratio(16.0 / 9.0)               // Maintain aspect ratio (TODO: proper implementation)
```

**Visual Modifiers:**
```rust
.background(Color::blue())      // Fill background
.foreground_color(Color::white()) // Text color
.corner_radius(8.0)             // Rounded corners
.border(Color::gray(), 1.0)     // Border
.shadow(ShadowSize::Medium)     // Drop shadow
.opacity(0.5)                   // Transparency
```

**Behavior Modifiers:**
```rust
.hidden(true)                   // Visibility
.disabled(true)                 // Interaction
.scale(1.2)                     // Scale (TODO: implementation)
.on_tap_gesture("id", || {})    // Tap handler
```

### Order Matters

```rust
// Padding INSIDE the background (blue box with internal padding)
Text::new("Hello")
    .padding(16.0)          // Applied first (inner)
    .background(Color::blue()) // Applied second (outer)

// Padding OUTSIDE the background (blue box with external spacing)
Text::new("Hello")
    .background(Color::blue()) // Applied first (inner)
    .padding(16.0)          // Applied second (outer)
```

**Visual difference:**
- First: Blue background extends to padding edge
- Second: Padding creates space around blue background

### Adding a New Modifier

1. **Add variant to `ModifierKind` enum** (`src/modifier.rs`):
```rust
pub enum ModifierKind {
    // ... existing variants
    MyModifier { value: f32 },
}
```

2. **Add method to `Modifier` trait**:
```rust
pub trait Modifier: IntoElement + Sized {
    // ... existing methods

    fn my_modifier(self, value: f32) -> Modified<Self> {
        Modified {
            child: self,
            modifier: ModifierKind::MyModifier { value },
        }
    }
}
```

3. **Implement rendering in `ModifiedElement::render()`**:
```rust
impl RenderOnce for ModifiedElement {
    fn render(self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let child = self.view.into_any_element();

        match self.modifier {
            // ... existing cases

            ModifierKind::MyModifier { value } => {
                div()
                    .flex_grow()
                    // Apply GPUI styling based on value
                    .some_gpui_method(value)
                    .child(child)
            }
        }
    }
}
```

4. **Add test to storybook** (`examples/storybook/stories/components.rs`):
```rust
// In "Modifiers" story or create new story
Text::new("Test")
    .my_modifier(42.0)
```

---

## State Management

Allui is **stateless**—all state management uses GPUI's native patterns.

### Patterns

**1. View State in Struct Fields**
```rust
struct MyView {
    count: i32,
    is_dark_mode: bool,
}
```

**2. Reactive Updates via `cx.notify()`**
```rust
impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .child(Text::new(format!("Count: {}", self.count)))
            .child(
                Button::new("+", cx.listener(|this, _, _, cx| {
                    this.count += 1;
                    cx.notify();  // ✅ Triggers re-render
                }))
            )
    }
}
```

**3. Entity State for Input Widgets**
```rust
struct MyView {
    email: Entity<InputState>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            email: cx.new(|cx| InputState::new(window, cx).placeholder("Email")),
        }
    }
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TextField::new(&self.email)
    }
}
```

**4. Subscriptions for Events**
```rust
cx.subscribe(&self.slider, |this, _, event: &SliderEvent, cx| {
    if let SliderEvent::Change(value) = event {
        this.volume = value.start();
        cx.notify();
    }
});
```

---

## Testing

### Interactive Testing

**Primary testing method:** Interactive storybook

```bash
# Run storybook
cargo run --example storybook --release
```

**Location:** `examples/storybook/` (7 files, ~2150 lines, 17 stories)

**Storybook structure:**
- `main.rs` - Entry point, window setup, theme management (System/Light/Dark modes)
- `sidebar.rs` - Navigation sidebar with story list and theme toggle
- `stories/mod.rs` - Story enum, metadata, and component organization
- `stories/layout.rs` - Stories (VStack, HStack, ZStack, Spacer)
- `stories/components.rs` - Stories (Text, Button, Modifiers, Toggle, TapGesture)
- `stories/containers.rs` - Stories (ScrollView, List, ForEach, Conditional)
- `stories/grids.rs` - Grid stories (Grid, LazyVGrid, LazyHGrid, Both Axes Scroll)

**Adding a new story:**
```rust
// In examples/storybook/stories/mod.rs - add variant to Story enum
pub enum Story {
    // ... existing variants
    MyComponent,
}

// In the appropriate stories/*.rs file - add render function
pub fn render_my_component(window: &mut Window, cx: &mut App) -> AnyElement {
    MyComponent::new("Demo").into_any_element()
}

// In examples/storybook/stories/mod.rs - add to metadata and render match
```

### Unit Testing

**Current state:** No automated tests

**Future:** Consider adding:
- Layout behavior tests (verify SwiftUI parity)
- Modifier chaining tests
- Visual regression tests (screenshot comparison)

---

## Code Style & Conventions

### Rust Style

**Follow standard Rust conventions:**
- Use `rustfmt` for formatting
- Run `clippy` and address warnings
- Prefer `impl Trait` return types for flexibility
- Use `#[must_use]` for builder methods

**Examples:**
```rust
// ✅ Good: Builder pattern with impl Trait
pub fn new(title: impl Into<String>) -> Self { ... }

// ✅ Good: Expressive method chaining
Text::new("Hello")
    .font_size(20.0)
    .foreground_color(Color::blue())
```

### Documentation

**Document public APIs:**
```rust
/// A control for toggling between on and off states.
///
/// # Example
///
/// ```rust,ignore
/// Toggle::new_with_handler("Dark Mode", is_dark_mode,
///     cx.listener(|view, checked: &bool, _, cx| {
///         view.is_dark_mode = *checked;
///         cx.notify();
///     })
/// )
/// ```
pub struct Toggle { ... }
```

**Inline comments for design decisions:**
```rust
// Use flex_grow() to propagate size through modifier chain
div().flex_grow().child(child)
```

### SwiftUI Naming Parity

**Always match SwiftUI names exactly:**
- Types: `VStack`, `TextField`, `Spacer` (not `VBox`, `TextInput`, `Flex`)
- Methods: `.padding()`, `.background()` (not `.set_padding()`, `.with_background()`)
- Parameters: `alignment`, `spacing` (match SwiftUI parameter names)
- Enums: `HorizontalAlignment::Leading` (not `Left`)

### File Organization

**One type per file:**
- `src/components/button.rs` defines `Button`
- `src/layout/vstack.rs` defines `VStack`

**Exceptions:**
- Related enums can live with their type (e.g., `ButtonStyle` in `button.rs`)
- Small helpers (e.g., `uniform_size()` in `lazy_stack.rs`)

**Module exports:**
```rust
// src/components/mod.rs
mod button;
pub use button::*;

// src/lib.rs
pub mod components;
pub mod layout;

// src/prelude.rs - common types
pub use crate::components::{Button, Text, Toggle};
pub use crate::layout::{VStack, HStack, ZStack};
```

---

## Known Limitations & TODOs

### High Priority

**1. Image Loading** (`src/components/image.rs:87`)
- **Status:** Placeholder only (renders labeled boxes)
- **Needed:** Actual image loading via GPUI's `img()` element
- **Affects:** File, URL, and System icon variants

**2. Text Enhancements** (`src/components/text.rs:116-117`)
- **Missing:** Line limit (text truncation)
- **Missing:** Strikethrough decoration

**3. ProgressView Animation** (`src/components/progress_view.rs:96`)
- **Status:** Static div
- **Needed:** Spinning animation for indeterminate state

**4. Label Icon Rendering** (`src/components/label.rs:69`)
- **Status:** Text placeholder for icon
- **Needed:** Actual icon from gpui-component

### Medium Priority

**5. Shadow Customization** (`src/modifier.rs:577`)
- **Status:** Preset sizes only (sm, md, lg, xl)
- **Needed:** Custom color, x/y offsets when GPUI supports it

**6. Scale Transform** (`src/modifier.rs:652`)
- **Status:** Passthrough (not implemented)
- **Needed:** CSS transform or custom GPUI element

**7. Aspect Ratio** (`src/modifier.rs:673`)
- **Status:** Passthrough
- **Needed:** Proper implementation when GPUI supports it

**8. ScrollView Indicators** (`src/layout/scroll_view.rs:124`)
- **Status:** Cannot hide scroll indicators
- **Blocked by:** GPUI API for scrollbar control

### Future Work

**9. State Management Abstractions**
- `@State`, `@Binding` equivalents via proc macros?
- Observable object pattern
- Environment values

**10. Navigation System**
- NavigationStack, NavigationLink
- Sheet, popover, alert modals
- Tab bar, sidebar navigation

**11. Animation System**
- Implicit animations (`.animation()` modifier)
- Transition effects
- Gesture-driven animations

---

## Common Pitfalls

### 1. Forgetting `cx.notify()` After State Changes

```rust
// ❌ WRONG: View won't update
Button::new("Click", cx.listener(|this, _, _, cx| {
    this.value = 42;
    // Missing cx.notify()!
}))

// ✅ CORRECT: Triggers re-render
Button::new("Click", cx.listener(|this, _, _, cx| {
    this.value = 42;
    cx.notify();
}))
```

### 2. Using Dynamic Element IDs

```rust
// ❌ WRONG: New ID every render
static COUNTER: AtomicU64 = AtomicU64::new(0);
Text::new("Click").on_tap_gesture(
    format!("btn-{}", COUNTER.fetch_add(1, ...)),
    || {}
)

// ✅ CORRECT: Stable ID
Text::new("Click").on_tap_gesture("my-button", || {})
```

### 3. Forgetting `gpui_component::init()`

```rust
// ❌ WRONG: Will panic when using Toggle, TextField, etc.
Application::new().run(|cx| {
    cx.open_window(WindowOptions::default(), |window, cx| {
        // Missing gpui_component::init()!
    });
})

// ✅ CORRECT: Initialize before using components
Application::new().run(|cx| {
    gpui_component::init(cx);
    cx.open_window(...);
})
```

### 4. Creating Entity State in `render()`

```rust
// ❌ WRONG: Creates new state every render
impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input = cx.new(|cx| InputState::new(window, cx));  // New entity every render!
        TextField::new(&input)
    }
}

// ✅ CORRECT: Create in constructor, store in view
struct MyView {
    input: Entity<InputState>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| InputState::new(window, cx)),
        }
    }
}
```

### 5. Incorrect Modifier Order

```rust
// These produce different layouts!

// Padding INSIDE background (blue fills to padding edge)
Text::new("Hello")
    .padding(16.0)
    .background(Color::blue())

// Padding OUTSIDE background (blue box + external space)
Text::new("Hello")
    .background(Color::blue())
    .padding(16.0)
```

### 6. Forgetting to Implement `Modifier` Trait

```rust
// ❌ WRONG: Can't chain modifiers
pub struct MyComponent { ... }
impl RenderOnce for MyComponent { ... }
impl IntoElement for MyComponent { ... }
// Missing: impl Modifier for MyComponent {}

// ✅ CORRECT: Add blanket implementation
impl Modifier for MyComponent {}
```

### 7. Using `Entity::new()` Instead of `cx.new()`

```rust
// ❌ WRONG: Won't work, Entity is opaque
let state = Entity::new(...);  // No such constructor

// ✅ CORRECT: Use context to create entities
let state = cx.new(|_| InputState::new(...));
```

---

## Quick Reference

### Essential Imports

```rust
use gpui::prelude::*;
use allui::prelude::*;
```

### Basic Application Template

```rust
use gpui::{App, Application, Context, Window, WindowOptions};
use allui::prelude::*;

struct MyView {
    count: i32,
}

impl MyView {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { count: 0 }
    }
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .spacing(16.0)
            .child(Text::new(format!("Count: {}", self.count)))
            .child(
                Button::new("+", cx.listener(|this, _, _, cx| {
                    this.count += 1;
                    cx.notify();
                }))
            )
            .padding(24.0)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);  // Required!

        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new_view(|cx| MyView::new(window, cx))
        });
    });
}
```

### Common Patterns

**VStack with children:**
```rust
VStack::new()
    .spacing(8.0)
    .child(Text::new("Title"))
    .child(Text::new("Subtitle"))
```

**ForEach iteration:**
```rust
VStack::new()
    .children(ForEach::new(&items, |item| {
        Text::new(&item.name)
    }))
```

**Conditional rendering:**
```rust
If::new(is_logged_in)
    .then(|| ProfileView::new())
    .otherwise(|| LoginView::new())
```

**Input with state:**
```rust
// In constructor:
let input = cx.new(|cx| InputState::new(window, cx).placeholder("Email"));

// In render:
TextField::new(&input).cleanable(true)
```

---

## Getting Help

**Storybook examples:** Run `cargo run --example storybook --release`
**README:** See `README.md` for user-facing documentation
**Source code:** All components are well-documented, check implementations for patterns

**Key files to reference:**
- `src/modifier.rs` - Modifier system implementation
- `src/layout/vstack.rs` - Simple layout primitive example
- `src/components/toggle.rs` - Input component example
- `examples/storybook/main.rs` - Complete usage examples
- `examples/storybook/stories/` - Story implementations by category

---

**Last Updated:** 2026-01-10
**Allui Version:** 0.1.0
**GPUI Version:** 0.2
**gpui-component Version:** 0.5.0
