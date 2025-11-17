# [Dear Mirl Gui](https://github.com/Miner3D-Gamer/dear-mirl-gui)


**A registry-based, retained-mode, modular GUI library for [Mirl](https://crates.io/crates/mirl), inspired by [`Dear ImGui`](https://github.com/ocornut/imgui).**

Or in simpler terms:
A debug window crate designed and tailored to work with [Mirl](https://crates.io/crates/mirl). It can be implemented in around 20 lines of code (If you don't use fancy bracket formatting)

It is by no means as good as Dear Imgui or its native rust ports but I have yet to see one of them run smoothly on the cpu.

> Default modules are listed at the bottom

## Integration:

Further infos about every available module can be found within the docstring of that module.

How to use:

```rust
    use mirl::platform::framework_traits::*;
    use dear_mirl_gui::module_manager::*;
    use dear_mirl_gui::modules;

    fn run_loop(buffer: &mut mirl::Buffer, window: &mut dyn mirl::platform::framework_traits::ExtendedFramework, font: &fontdue::Font){
        // Important! This formatting will be referenced a lot to avoid duplicate code
        set_formatting(dear_mirl_gui::Formatting::default(&font, 20)); // 20 Being the height in pixels

        // Define your module - Module struct - Displayed text
        let text_display = //   v               v
            register_module(modules::Text::new("Hello World!"));

        // If you wanna use multiple guis use the WindowManager, otherwise use the DearMirlGui directly. The .update() functions are identical.
        // In the ::<const FAST: bool, const USE_CACHE: bool> I've set to
        // - FAST: true -> You probably don't need this but who doesn't enjoy free frame time (at the cost of visual output)
        // - USE_CACHE: true -> This is honestly a must, it reduces redraw so much that on flamegraph, the only visible module for the test scene is a single animated widget
        let mut window_manager: dear_mirl_gui::WindowManager<true, true> = dear_mirl_gui::WindowManager::new(
            Vec::from([
                dear_mirl_gui::DearMirlGui::new_simple("Gui Window", (100, 10), &vec![text_display.id()])
                ])
            );
        while window.is_open() {
            // Clearing last frame
            buffer.clear();

            // Gathering data
            let mouse_scroll = window
                .get_mouse_scroll()
                .map(mirl::extensions::Tuple2Into::tuple_into);
            let mouse_pos = window.get_mouse_position();
            let pressed_keys = window.get_all_keys_down();

            // Using the data to update all/the window(s)
            let gui_output = window_manager.update(
                mouse_pos,
                mouse_scroll,
                window.is_mouse_down(mirl::platform::MouseButton::Left),
                window.is_mouse_down(mirl::platform::MouseButton::Middle),
                window.is_mouse_down(mirl::platform::MouseButton::Right),
                &pressed_keys,
                0.0, // Delta time - Required for animated components
                &None,
            );
            // Standard drawing routine
            // ...

            // Automatic drawing, for manual drawing use window_manager.draw()
            window_manager.draw_on_buffer(buffer);

            // Update framework
            window.update(buffer);
        }
    }
```

## Examples (Used for internally for testing of modules)

### You can use either of these:

- 'cargo test -p dear_mirl_gui --features debug-window -- --nocapture'
- 'cargo test -p dear_mirl_gui --release --features debug-window -- --nocapture'

### Or if you also want to see experimental features use these:

- 'cargo test -p dear_mirl_gui --features experimental -- --nocapture'
- 'cargo test -p dear_mirl_gui --release --features experimental -- --nocapture'

## Currently Known Problems

> Just remember the reason only the bugs are highlighted: there are way more things that work than things that don't.

### Visual Issues

- **[4]** Button hover/click highlight only appears in unselected GUI; during text motion, only click highlight fails.
- **[0]** Lever module is not smooth.
- **[6]** _(Plugin makers)_ Single insert mode overwrites image data of other modules. Use **replace all** instead.

### Functional Issues

- **[0]** Text input auto selects a structure when clicking after the last character (first-time selection).
- **[2]** Text input can select itself through other windows.
- **[6]** Crank module rotation is slightly offset.

### Planned Additions

- **[4]** Color picker missing.
- **[4]** Struct editor unfinished.
- **[7]** _(Plugin makers)_ Return layers not supported.
- **[3]** Text input drag to select not yet implemented.

## Design Philosophy

The goal of the lib is to be both as extendable as possible (using a single trait) while also providing seamless working conditions for devs. It's built with these purposes in mind:

- A lightweight registry system for modularity and plugin support
- Good performance for non GPU environments (Optional caching)
- Minimal integration effort

## Default Modules (Alphabetically sorted):

### Widgets (15-3):
- Button            => Hover and click
- Checkbox          => Click to cycle through any amount of states
- Color Picker      => Unimplemented!
- Crank             => Get rotational data
- Image button      => A button that instead of showing text, displays an image
- Image             => A static image
- Lever             => Up or Down, boolean input
- List select       => Unimplemented!
- Number display    => Text module for numbers
- Progress bar      => From 0% to 100% (Can be vertical)
- Selection         => Radio Buttons with the option of selecting multiple options
- Slider            => Slider from 0 to 1 supporting custom ranges
- Struct Editor     => Unimplemented!
- Text Input        => Virtual text box (Supports custom keybinds)
- Text              => Display text

### Decoration (4):
- Custom Offset     => Set custom offset between modules
- Line              => A simple divider
- Reset Offset      => Returns further modules back to the front of the container
- Same line         => Puts the next module on the same height as the previous ones