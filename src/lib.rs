#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::todo)]
#![warn(clippy::panic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![feature(result_option_map_or_default)]
//! A registry-based, retained-mode, modular GUI library for Mirl,
//! inspired by `Dear ImGui`.
//!
//! Or in simpler terms:
//! A debug window crate designed to work with Mirl

#[cfg(not(feature = "draw_safe"))]
const DRAW_SAFE: bool = false;
#[cfg(feature = "draw_safe")]
const DRAW_SAFE: bool = true;

#[cfg(feature = "debug-window")]
use mirl::{
    extensions::*, graphics::rgb_to_u32, platform::file_system::FileSystem,
    platform::framework_traits::Window,
};
use mirl::{
    platform::{Buffer, CursorStyle, keycodes::KeyCode},
    render::{self},
};

/// All builtin modules
pub mod modules;
// pub use modules::{
//     Button, CheckBox, Image, ProgressBar, ResetFormatting, SameLine, Selection,
//     Separator, Slider, Text, TextInput, WindowEmulator,
// };
/// The `DearMirlGui` defining file
pub mod gui;
pub use gui::DearMirlGui;

use crate::gui::ModuleContainer;

impl ButtonState {
    #[must_use]
    /// Create a new button state -> Pressed, clicked, and released are calculated
    pub const fn new(current: bool, last: bool) -> Self {
        Self {
            down: current,
            clicked: current && !last,
            released: !current && last,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The current state of a mouse buttons and if they have just been pressed
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct ButtonState {
    down: bool,
    clicked: bool,
    released: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The current state of the mouse buttons and if they have just been pressed
pub struct MouseData {
    left: ButtonState,
    middle: ButtonState,
    right: ButtonState,
}

#[derive(Debug, Clone)]
#[allow(missing_docs)]
/// The current formatting for a window/its modules
pub struct Formatting {
    pub font: fontdue::Font,
    pub main_color: u32,
    pub secondary_color: u32,
    pub text_color: u32,
    /// Currently unused
    pub misc_ui_color: u32,
    pub horizontal_margin: usize,
    pub vertical_margin: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // Once again not a state machine
/// A struct holding every bit of info the `DearMirlGui` struct and every module returns
pub struct GuiOutput {
    /// Requested cursor style
    pub new_cursor_style: Option<mirl::platform::CursorStyle>,
    /// If this window took focus
    pub focus_taken: bool,
    /// Requested new cursor position
    pub new_cursor_position: Option<(isize, isize)>,
    /// If the cursor should be hidden
    pub hide_cursor: bool,
    /// Requested data to set the clipboard to
    pub new_clipboard_data: Option<mirl::platform::file_system::FileData>,
    /// If a text input is selected (Some devices may not have a keyboard available at all times)
    pub text_input_selected: bool,
    /// If a module would like to access what is currently stored in the clipboard
    pub request_clipboard_data: bool,
}

#[derive(Debug, Clone, Copy)]
/// A struct holding all information the modules are provided with
pub struct ModuleInputs<'a> {
    /// If a previous module already took focus
    pub focus_taken: bool,
    /// The current mouse position
    pub mouse_pos: Option<(isize, isize)>,
    /// The mouse position since last frame - is (0, 0) when `mouse_pos` is None
    pub mouse_pos_delta: (isize, isize),
    /// The mouse scroll distance, (x, y)
    pub mouse_scroll: Option<(isize, isize)>,
    /// Info on what mouse buttons have been pressed
    pub mouse_info: &'a MouseData,
    /// All pressed keys
    pub pressed_keys: &'a Vec<KeyCode>,
    /// Delta time, what else to say about it?
    pub delta_time: f64,
    /// The current formatting used
    pub formatting: &'a Formatting,
    /// Clipboard data must be requested first
    pub clipboard_data: &'a Option<mirl::platform::file_system::FileData>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// The return further info that just [`Option::None`]/[`Option::Some`]
pub enum GuiReturnsModule<T: 'static> {
    /// All went well and you can use the module no problem
    AllGood(T),
    /// There was no module with this ID
    UnableToFindID(String),
    /// There was an module with this ID, however the used type was not correct
    CastingAsWrongModule {
        /// The correct type the module should be casted as
        correct: String,
        /// The incorrect type the module was requested to be casted into
        wrong: String,
        /// The id of the module
        id: String,
    },
}

/// A trait any struct can implement to be used as a `DearMirlGui` module
pub trait DearMirlGuiModule: AnyCasting + std::fmt::Debug + WhatAmI {
    /// Create an internal buffer, draw all desired info on it and return
    fn draw(&self, formatting: &crate::Formatting) -> Buffer;
    /// Gets the height of a module regardless of the returned buffer height
    fn get_height(&self, formatting: &crate::Formatting) -> isize;
    /// Gets the width of a module regardless of the returned buffer height
    fn get_width(&self, formatting: &crate::Formatting) -> isize;
    /// Update the internal state of the module with the given information
    fn update(&mut self, inputs: &crate::ModuleInputs) -> crate::GuiOutput;
    /// Get an offset for the next module
    fn get_next_offset(
        &self,
        _modules: &indexmap::IndexMap<String, ModuleContainer>,
        _current_idx: usize,
        _formatting: &crate::Formatting,
    ) -> (isize, isize) {
        (0, 0)
    }
    /// Check if the module needs a redraw
    fn need_redraw(&self) -> bool;
    /// When you have updated the formatting and wish to properly apply the change to all modules
    fn apply_new_formatting(&mut self, formatting: &crate::Formatting);
}

impl<T: 'static> GuiReturnsModule<T> {
    /// # Panics
    /// If the value is not `Self::AllGood`, it will error with the corresponding error message
    #[allow(clippy::panic)]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::AllGood(value) => value,
            Self::CastingAsWrongModule {
                wrong,
                correct,
                id,
            } => {
                panic!(
                    "Module with id '{id}' is being cast as the wrong type;\n\tRequested module type: '{wrong}', \n\tCorrect module type: '{correct}'"
                )
            }
            Self::UnableToFindID(id) => {
                panic!("Unable to find a module with the id of {id}")
            }
        }
    }
}
/// Automatic Any casting for all structs that implemented [`DearMirlGuiModule`]
pub trait AnyCasting {
    /// Return self as an Any instance
    fn as_any(&self) -> &dyn std::any::Any;
    /// Return self as a mutable Any instance
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Get the type of any struct
pub trait WhatAmI {
    /// Get location/type of used struct
    fn what_am_i(&self) -> &'static str;
}

impl<T: 'static> WhatAmI for T {
    fn what_am_i(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
// MAGIC. THIS IS SOME BATSHIT I'D EXPECT PYTHON TO ALLOW ME, NOT RUST
impl<T: DearMirlGuiModule + 'static> AnyCasting for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GuiOutput {
    #[must_use]
    /// Create a new output with everything set to None or False
    pub const fn default(took_focus: bool) -> Self {
        Self {
            new_cursor_style: None,
            focus_taken: took_focus,
            new_cursor_position: None,
            hide_cursor: false,
            new_clipboard_data: None,
            text_input_selected: false,
            request_clipboard_data: false,
        }
    }
    // pub const fn set_cursor_style(
    //     &mut self,
    //     cursor_style: Option<CursorStyle>,
    // ) -> &mut Self {
    //     self.new_cursor_style = cursor_style;
    //     self
    // }
}
// #[deprecated(note = "Please enable the  feature in mirl for mutual compatibility")]
// const fn deprecated_trigger() {}

#[test]
#[cfg(not(feature = "debug-window"))]
fn main() {
    panic!(
        "Debug window flag not set; use 'cargo test --features debug-window -- --nocapture'"
    )
}
#[test]
#[cfg(feature = "debug-window")]
#[allow(dead_code)]
fn main() {
    let output = std::thread::Builder::new()
        .stack_size(4 * mirl::constants::bytes::GB)
        .spawn(actual_main);
    let _ = output.map(|x| x.join()).unwrap();
}
#[allow(clippy::unwrap_used)]
#[cfg(feature = "debug-window")]
#[allow(dead_code)]
fn actual_main() {
    let buffer = mirl::platform::Buffer::new_empty(800, 600);
    let mut window = mirl::platform::glfw::Framework::new(
        "Rust Window",
        *mirl::platform::WindowSettings::default(&buffer)
            .set_position_to_middle_of_screen(),
    )
    .unwrap();
    let file_system =
        mirl::platform::file_system::NativeFileSystem::new(Vec::new()).unwrap();
    main_loop(&mut window, &file_system, &buffer);
}

#[allow(clippy::unwrap_used, clippy::float_cmp, clippy::too_many_lines)]
#[cfg(feature = "debug-window")]
fn main_loop<
    F: mirl::platform::framework_traits::ExtendedFramework<f64>,
    D: mirl::platform::file_system::FileSystem,
>(
    window: &mut F,
    file_system: &D,
    buffer: &Buffer,
) {
    mirl::enable_traceback();
    let font = mirl::platform::file_system::get_default_font(file_system)
        .unwrap()
        .as_font()
        .unwrap();

    let mut gui: DearMirlGui<true, false> =
        DearMirlGui::new_simple("Example Window", 100, 10, 0, 0, &font);
    gui.add_module("module1", modules::Text::new("Hello World", 20, None));
    gui.add_module(
        "module2",
        modules::Text::new("there is text now ig", 20, None),
    );

    gui.add_module(
        "slider",
        modules::Slider::<f64>::new(30, None, None, None, true),
    );
    gui.add_module(
        "progress_bar_up",
        modules::ProgressBar::new(30, None, None, false),
    );
    gui.add_module(
        "progress_bar_down",
        modules::ProgressBar::new(30, None, None, true),
    );

    gui.add_module(
        "button",
        modules::Button::new(
            "Clickn't Me!".into(),
            20,
            None,
            Some(|| {
                println!("Oh no, I've been pressed!");
            }),
            Some(&font),
        ),
    );
    gui.add_module(
        "button2",
        modules::Button::new(
            "A Button with really really long text".into(),
            20,
            None,
            None,
            None,
        ),
    );
    gui.add_module(
        "checkbox1",
        modules::CheckBox::new_3_state(20, "sample text".to_string()),
    );
    gui.add_module(
        "checkbox2",
        modules::CheckBox::new_2_state(20, "bottom text".to_string()),
    );
    // #[cfg(feature = "BAD_APPLE")]
    // gui.add_module(
    //     "bad_apple".into(),
    //     CheckBox::new_3_state(100, "Evil Apple".to_string()),
    // );
    gui.add_module("divider", modules::Separator::new(20, 300, false, None));

    gui.add_module(
        "selection",
        modules::Selection::new(
            &[
                "Option 1a".into(),
                "Option 2a".into(),
                "Option 3a".into(),
                "Option 4a".into(),
            ],
            20,
            true,
            &gui.formatting,
            None,
        ),
    );
    gui.add_module("anti_new_line", modules::SameLine::new(10));
    gui.add_module(
        "selection2",
        modules::Selection::new(
            &[
                "Option 1b".into(),
                "Option 2b".into(),
                "Option 3b".into(),
                "Option 4b".into(),
            ],
            20,
            false,
            &gui.formatting,
            None,
        ),
    );
    gui.add_module("formatting", modules::ResetFormatting::new());
    gui.add_module(
        "input",
        modules::TextInput::new(
            20,
            300,
            4,
            Some(Vec::from([
                "text.chars().take(i + 1).collect::<String>(),".into(),
                "Another creative line for thinky thinky".into(),
                "   Indent testing :/".into(),
            ])),
            Some("Click me to start writing"),
            &gui.formatting,
        ),
    );
    // let mut sub_window = DearMirlGui::<false, true>::new_simple(
    //     "Minesweeper",
    //     0,
    //     0,
    //     0,
    //     0,
    //     &font,
    // );
    // let rows = 5;
    // let columns = 2;

    // for c in 0..columns {
    //     for r in 0..rows {
    //         let idx = r + rows * c;
    //         println!("Hi {}", idx);
    //         let module = CheckBox::new_2_state(5, "".into());
    //         println!("Don't");
    //         sub_window.add_module(format!("column_{}", idx), module);
    //         println!("All good");
    //         sub_window
    //             .add_module(format!("formatting_{}", idx), SameLine::new(0));
    //         println!("Added formatter");
    //     }
    //     sub_window.add_module(format!("resetter_{}", c), ResetFormatting::new())
    // }

    // sub_window.set_size_to_see_all_modules();

    // gui.add_module("minesweeper".into(), WindowEmulator::new(sub_window));

    gui.set_size_to_see_all_modules();

    let cursor_style_manager = window
        .load_custom_cursor(
            0.into(),
            rgb_to_u32(0, 255, 200),
            rgb_to_u32(0, 100, 100),
        )
        .unwrap();
    //println!("#{}", mirl::graphics::u32_to_hex(rgb_to_u32(0, 0, 255)));
    let mut frame_start = std::time::Instant::now();
    let mut delta_time = 0.0;
    let mut fps;
    let mut fps_list = Vec::new();

    let mut delta_time_accumulation: f64 = 0.0;
    let mut request_clipboard_data = false;

    while window.is_open() {
        buffer.clear_buffer_with_color(rgb_to_u32(110, 150, 140));
        let mouse_scroll = window
            .get_mouse_scroll()
            .map(mirl::extensions::Tuple2Into::tuple_2_into);

        // Set this to true if you wanna see how the gui handles casting modules to an incorrect type
        if false {
            gui.get_module_as::<modules::Text, ()>("slider", |_| {}).unwrap();
        }

        gui.get_module_as_mut::<modules::ProgressBar, ()>(
            "progress_bar_up",
            |slider| {
                slider.progress += delta_time as f32 / 10.0;
                slider.progress = slider.progress.clamp(0.0, 1.0);
                if slider.progress == 1.0 {
                    slider.progress = 0.0;
                }
            },
        )
        .unwrap();

        gui.get_module_as_mut::<modules::Selection, ()>(
            "selection",
            |buttons| {
                if buttons.currently_selected[3] {
                    buttons.currently_selected[3] = false;
                    buttons.radio_buttons = !buttons.radio_buttons;
                }
            },
        )
        .unwrap();

        gui.get_module_as_mut::<modules::ProgressBar, ()>(
            "progress_bar_down",
            |slider| {
                let exact = delta_time_accumulation.sin().midpoint(1.0);

                slider.progress = exact as f32;
                slider.progress = slider.progress.clamp(0.0, 1.0);
                if slider.progress == 0.0 {
                    slider.progress = 1.0;
                }
            },
        )
        .unwrap();

        let mouse_pos = window.get_mouse_position();

        delta_time_accumulation += delta_time * 2.0;

        let clipboard_data: Option<mirl::platform::file_system::FileData> = {
            if request_clipboard_data {
                println!("A module requested the current clipboard value");
            }
            None
        };

        let gui_output = gui.update(
            mouse_pos,
            mouse_scroll,
            window.is_mouse_down(mirl::platform::MouseButton::Left),
            window.is_mouse_down(mirl::platform::MouseButton::Middle),
            window.is_mouse_down(mirl::platform::MouseButton::Right),
            &window.get_all_keys_down(),
            delta_time,
            &clipboard_data,
        );
        gui.draw_on_buffer(buffer);

        request_clipboard_data = gui_output.request_clipboard_data;

        if !gui_output.focus_taken {
            // Do stuff that uses mouse/keyboard
        }
        if let Some(data) = &gui_output.new_clipboard_data {
            println!(
                "New clipboard data has been provided: {}",
                data.to_printable()
            );
        }

        if let Some(_pos) = gui_output.new_cursor_position {
            // Not natively possible in mirl -> On the to do list
        }
        if let Some(_data) = gui_output.new_clipboard_data {
            // Not natively possible in mirl -> On the to do list
        }
        if gui_output.hide_cursor {
            // Not natively possible in mirl -> On the to do list
        }

        if let Some(mouse_pos) = mouse_pos
            && buffer
                .create_collision_isize::<false>(0, 0)
                .does_area_contain_point(mouse_pos)
        {
            window.set_cursor_style(
                cursor_style_manager.from_cursor_style(
                    gui_output
                        .new_cursor_style
                        .unwrap_or(mirl::platform::CursorStyle::Default),
                ),
            );
        }

        delta_time = frame_start.elapsed().as_secs_f64();
        frame_start = std::time::Instant::now();

        if delta_time == 0.0 {
            fps = f64::MAX;
        } else {
            fps = 1.0 / delta_time;
        }
        let average_fps: u64 = fps_list.average().unwrap_or_default();

        mirl::extensions::lists::add_item_to_max_sized_list(
            &mut fps_list,
            average_fps.max(1) as usize,
            fps as u64,
        );

        window.update(buffer);
        window.set_title(&format!("Rust Window {average_fps} <- {fps}"));
    }
}
