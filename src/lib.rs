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
//! 
//! Currently known problems:
//! Button module hover and click highlight only appears in unselected gui except when the text is moving in which case only the click highlight isn't applied
//! Text input module automatically selects a structure when clicking after the last character
//! Crank module rotation is slightly offset
//! Lever module is not smooth

#[cfg(not(feature = "draw_safe"))]
const DRAW_SAFE: bool = false;
#[cfg(feature = "draw_safe")]
const DRAW_SAFE: bool = true;

use mirl::extensions::*;
#[cfg(feature = "debug-window")]
use mirl::{graphics::rgb_to_u32, platform::framework_traits::Window};
use mirl::{
    platform::{Buffer, CursorStyle},
    render::{self},
};
/// Add, remove, and edit modules
pub mod module_manager;
pub use module_manager::*;

/// All builtin modules
pub mod modules;

/// The `DearMirlGui` defining file
pub mod gui;
pub use gui::DearMirlGui;

use crate::gui::ModuleContainer;

// A struct to handle having multiple guis at once
mod window_manager;
pub use window_manager::*;

// How deeply a gui/module has taken user input
mod focus_taken;
pub use focus_taken::*;

// Formatting is used to assure that important data exists like a font and modules can adjust themselves to look a little less bad
mod formatting;
pub use formatting::*;

// Used for telling the compiler what module is what type
mod module_path;
pub use module_path::*;

// Gui/Module output
mod output;
pub use output::*;
// Gui/Module input
mod input;
pub use input::*;

/// Input sub section for mouse data
mod mouse_data;
pub use mouse_data::*;

/// A trait any struct can implement to be used as a `DearMirlGui` module
pub trait DearMirlGuiModule:
    AnyCasting + std::fmt::Debug + WhatAmI + std::marker::Send
{
    /// Create an internal buffer, draw all desired info on it and return
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode);
    /// Gets the height of a module regardless of the returned buffer height
    fn get_height(&self, formatting: &crate::Formatting) -> isize;
    /// Gets the width of a module regardless of the returned buffer height
    fn get_width(&self, formatting: &crate::Formatting) -> isize;
    /// Update the internal state of the module with the given information
    fn update(&mut self, inputs: &crate::ModuleUpdateInfo) -> crate::GuiOutput;
    /// Get an offset for the next module
    fn modify_offset_cursor(
        &self,
        _modules: &[ModuleContainer],
        _used_idx: &Vec<usize>,
        _formatting: &crate::Formatting,
        _current: (&mut isize, &mut isize),
    ) {
    }
    /// Manually setting wether a module needs a redraw -> Useful when intentionally using corrupting data
    fn set_need_redraw(&self, _need_redraw: Vec<(usize, bool)>) {}
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
            Self::Misc(stuff) => {
                panic!("An error occurred: {stuff}")
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

// ###################################################################################
// End of library - Tests ############################################################
// ###################################################################################

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
    actual_main();
    // let output = std::thread::Builder::new()
    //     .stack_size(4 * mirl::constants::bytes::GB)
    //     .spawn(actual_main);
    // let _ = output.map(|x| x.join()).unwrap();
}
#[allow(clippy::unwrap_used)]
#[cfg(feature = "debug-window")]
#[allow(dead_code)]
fn actual_main() {
    use mirl::platform::file_system::FileSystemNew;

    let buffer = mirl::platform::Buffer::new_empty(800, 600);
    let mut window = mirl::platform::minifb::Framework::new(
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

    set_formatting(Formatting::default(&font));

    let m1 =
        register_module("module1", modules::Text::new("Hello World", 20, None));
    let m2 = register_module(
        "module2",
        modules::Text::new("there is text now ig", 20, None),
    );

    let slider = register_module(
        "slider",
        modules::Slider::<f64>::new(30, None, None, None, true),
    );
    let progress_bar_up = register_module(
        "progress_bar_up",
        modules::ProgressBar::new(30, None, None, false),
    );
    let progress_bar_down = register_module(
        "progress_bar_down",
        modules::ProgressBar::new(30, None, None, true),
    );

    let button = register_module(
        "button",
        modules::Button::new(
            "Clickn't Me!".into(),
            20,
            None,
            Some(&font),
            Some(|| {
                println!("Oh no, I've been pressed!");
            }),
            None,
            None,
        ),
    );
    let button2 = register_module(
        "button2",
        modules::Button::new(
            //"A Button".into(),
            "A Button with really really long text".into(),
            20,
            None,
            None,
            None,
            None,
            None,
        ),
    );
    let checkbox1 = register_module(
        "checkbox1",
        modules::CheckBox::new_3_state(20, "sample text".to_string()),
    );
    let checkbox2 = register_module(
        "checkbox2",
        modules::CheckBox::new_2_state(20, "bottom text".to_string()),
    );
    // #[cfg(feature = "BAD_APPLE")]
    // add_module(
    //     "bad_apple".into(),
    //     CheckBox::new_3_state(100, "Evil Apple".to_string()),
    // );
    let _ = register_module(
        "divider",
        modules::Separator::new(20, 300, false, None),
    );

    let selection = register_module(
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
            None,
        ),
    );
    let same_line =
        register_module("anti_new_line", modules::SameLine::new(10));
    let selection2 = register_module(
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
            None,
        ),
    );
    let resetter = register_module("formatting", modules::ResetOffset::new());
    let text_input = register_module(
        "input",
        modules::TextInput::new(
            20,
            400,
            4,
            Some(Vec::from([
                "text.chars().take(i + 1).collect::<String>(),".into(),
                "Another creative line for thinky thinky".into(),
                "   Indent testing :/".into(),
            ])),
            Some("Click me to start writing"),
        ),
    );
    let crank = register_module("crank", modules::Crank::new(60, 0, 0.0));
    let lever = register_module("lever", modules::Lever::new(40, 80));
    let lever2 = register_module("lever2", modules::Lever::new(40, 80));
    let lever3 = register_module("lever3", modules::Lever::new(40, 80));

    // let display =
    //     register_module("display", modules::NumberDisplay::new(0, 3, 20.0));

    let crank_info =
        register_module("crank_info", modules::Text::new("0", 20, None));
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

    // add_module("minesweeper".into(), WindowEmulator::new(sub_window));

    // gui.set_size_to_see_all_modules();
    // gui2.set_size_to_see_all_modules();
    let mut window_manager: WindowManager<true, true> =
        WindowManager::new(Vec::from([
            DearMirlGui::new_simple(
                "Example Window",
                100,
                10,
                0,
                0,
                &font,
                &[
                    m1.id(),
                    m2.id(),
                    button.id(),
                    button2.id(),
                    checkbox2.id(),
                    // register_module(
                    //     "sub_gui",
                    //     DearMirlGui::<true, true>::new_simple(
                    //         "Hello",
                    //         0,
                    //         0,
                    //         400,
                    //         100,
                    //         &font,
                    //         &Vec::from([text_input.id()]),
                    //     ),
                    // )
                    // .id(),
                    checkbox1.id(),
                    //display.id(),
                    crank_info.id(),
                    crank.id(),
                    text_input.id(),
                ],
            ),
            DearMirlGui::new_simple(
                "Another Window",
                200,
                300,
                0,
                0,
                &font,
                &[
                    m1.id(),
                    checkbox1.id(),
                    selection.id(),
                    same_line.id(),
                    selection2.id(),
                    resetter.id(),
                    button.id(),
                    button2.id(),
                    checkbox2.id(),
                    lever.id(),
                    same_line.id(),
                    lever2.id(),
                    same_line.id(),
                    lever3.id(),
                ],
            ),
        ]));
    window_manager.set_size_to_see_all_modules();

    let cursor_style_manager = window
        .load_custom_cursors(
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
            get_module_as::<_, ()>(&slider, |_| {}).unwrap();
        }

        get_module_as_mut::<_, ()>(&progress_bar_up, |slider| {
            slider.progress += delta_time as f32 / 10.0;
            slider.progress = slider.progress.clamp(0.0, 1.0);
            if slider.progress == 1.0 {
                slider.progress = 0.0;
            }
        })
        .unwrap();
        get_module_as_mut::<_, ()>(&crank_info, |crank_info| {
            let value =
                get_module_as_mut::<_, isize>(&crank, |crank| crank.rotations)
                    .unwrap();
            crank_info.set_text(format!("Rotations: {value}"));
        })
        .unwrap();

        get_module_as_mut::<modules::Selection, ()>(&selection, |buttons| {
            if buttons.currently_selected[3] {
                buttons.currently_selected[3] = false;
                buttons.radio_buttons = !buttons.radio_buttons;
                buttons.needs_redraw.set(true);
            }
        })
        .unwrap();

        get_module_as_mut::<modules::ProgressBar, ()>(
            &progress_bar_down,
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
        let pressed_keys = window.get_all_keys_down();

        let gui_output = window_manager.update(
            mouse_pos,
            mouse_scroll,
            window.is_mouse_down(mirl::platform::MouseButton::Left),
            window.is_mouse_down(mirl::platform::MouseButton::Middle),
            window.is_mouse_down(mirl::platform::MouseButton::Right),
            &pressed_keys,
            delta_time,
            &clipboard_data,
        );

        // let gui_output = gui.update(
        //     mouse_pos,
        //     mouse_scroll,
        //     window.is_mouse_down(mirl::platform::MouseButton::Left),
        //     window.is_mouse_down(mirl::platform::MouseButton::Middle),
        //     window.is_mouse_down(mirl::platform::MouseButton::Right),
        //     &pressed_keys,
        //     delta_time,
        //     &clipboard_data,
        // );

        // let gui_output = gui_output
        //     | gui2.update(
        //         mouse_pos,
        //         mouse_scroll,
        //         window.is_mouse_down(mirl::platform::MouseButton::Left),
        //         window.is_mouse_down(mirl::platform::MouseButton::Middle),
        //         window.is_mouse_down(mirl::platform::MouseButton::Right),
        //         &pressed_keys,
        //         delta_time,
        //         &clipboard_data,
        //     );
        // gui.draw_on_buffer(buffer);
        // gui2.draw_on_buffer(buffer);
        window_manager.draw_on_buffer(buffer);

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
