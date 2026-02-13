# [Dear Mirl Gui](https://github.com/Miner3D-Gamer/dear_mirl_gui)

**A registry-based, retained-mode, modular GUI library for [Mirl](https://crates.io/crates/mirl), inspired by [`Dear ImGui`](https://github.com/ocornut/imgui).**

Or in simpler terms:
A debug window crate designed and tailored to work with [Mirl](https://crates.io/crates/mirl). It can be inserted into a standart drawing routine in less than 20 lines of code

It is by no means as good as Dear Imgui or its native rust ports but I have yet to see one of them run smoothly on the cpu.

> Default modules are listed at the bottom

## Integration:

Further infos about every available module can be found within the docstring of that module.

How to use:

```rust
use dear_mirl_gui::module_manager::*; // Global functions for module management
use dear_mirl_gui::modules; // All the modules that are provided by default
use mirl::platform::windowing::traits::*; // Default mirl windowing traits
// You can also use these two lines
// use mirl::prelude::*;
// use dear_mirl_gui::prelude::*;

fn run_loop<W: ExtendedWindowingFramework>(
    buffer: &mut mirl::preludes::Buffer,
    window: &mut W,
    font: &mirl::dependencies::fontdue::Font,
) {
    // Important! This formatting will be referenced a lot to avoid duplicate code
    set_formatting(dear_mirl_gui::Formatting::default(font, 20)); // 20 Being the height of text in pixels

    // Define your module - Module struct - Displayed text
    let text_display = //   v               v
             register_module(modules::Text::new("Hello World!"));

    // If you wanna use multiple guis use the DearMirlGuiManager, otherwise use the DearMirlGui directly. The .update() functions are identical.
    // In the ::<const FAST: bool, const USE_CACHE: bool> I've set to
    // - FAST: false -> Minor Visual drawbacks
    // - USE_CACHE: true -> This is honestly a must, it reduces redraw so much that on flamegraph, the only visible module for the test scene is a single animated widget
    let mut window_manager: dear_mirl_gui::DearMirlGuiManager<false, true> =
        dear_mirl_gui::DearMirlGuiManager::new(Vec::from([
            dear_mirl_gui::DearMirlGui::new_simple(
                "Gui Window",
                (100, 10),
                &[text_display.id()],
            ),
        ]));
    while window.is_open() {
        // Clearing last frame
        buffer.clear();

        // Using the data to update all/the window(s)
        let gui_output = window_manager.update(
            &window.get_mouse_snapshot(),
            &window.get_all_keys_down(),
            0.0, // Delta time - Required for animated components
            &None,
        );
        if !gui_output.focus_taken {
            // The gui didn't take focus, use mouse/keyboard data to update your logic
        }
        // Standard drawing routine
        // ...

        // Automatic drawing, for manual drawing use window_manager.draw()
        window_manager.draw_on_buffer(buffer);

        // Update framework
        if let Err(error) = window.update(buffer) {
            println!("Error while updating window: {}", error);
        }
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

> Currently known issues are listed within the docstring of [lib.rs](src/lib.rs) to avoid error-syncing issues

## Design Philosophy

The goal of the lib is to be both as extendable as possible (using a single trait) while also providing seamless working conditions for devs. It's built with these purposes in mind:

For users:

- Minimal integration efforts due to being optimized for mirl
- Good performance for non GPU environments (Optional caching)

For plugin devs (Api details):

- Every module has freedom over only themselves by default, no worrying about other modules messing up values
- Every window updates containing modules, providing relative mouse positions and container id
- Module can choose wether it wants to look the same on all windows or have unique looks/behaviour with automatic caching.
- Module can define custom offsets when default formatting is insufficient

## Default Modules (Alphabetically sorted):

### Widgets (16-3):

- Button => Hover and click
- Checkbox => Click to cycle through any amount of states
- Crank => Get rotational data
- Image button => A button that instead of showing text, displays an image
- Image => A static image
- Lever => Up or Down, boolean input
- Number display => Text module specifically for numbers
- Number Input => A virtual textbox that only accepts numbers (No float support yet)
- Progress bar => From 0% to 100% (Can be vertical)
- Selection => Radio Buttons with the option of selecting multiple options
- Slider => Slider from 0 to 1 supporting custom ranges
- Text Input => Virtual text box (Supports custom keybinds)
- Text Display => Display text

## To be added (3):

- Color Picker => Pick a rgb(a) color
- List select => Select a value from a list of strings
- Struct Editor => Visually edit a struct

### Decoration (4-1):

- Custom Offset => Set custom offset between modules
- Line => A simple divider
- Reset Offset => Returns further modules back to the front of the container
- Same line => Puts the next module on the same height as the previous ones

## To be added (1):

- Tooltip => Show a piece of text when hovering over an area for a while
