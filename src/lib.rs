#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(trivial_casts)]
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
#![allow(deprecated)]
#![allow(clippy::doc_markdown)]
#![allow(trivial_numeric_casts)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::struct_excessive_bools)]
#![feature(const_ops)]
#![feature(result_option_map_or_default)]
#![feature(const_trait_impl)]
#![feature(new_range_api)]
#![feature(const_try)]
#![feature(const_cmp)]
// #![feature(unsized_const_params)]
// #![feature(adt_const_params)]
#![allow(rustdoc::broken_intra_doc_links)]
//! A registry-based, retained-mode, modular GUI library for Mirl,
//! inspired by `Dear ImGui`.
//!
//! Or in simpler terms:
//! A debug window crate designed and tailored to work with Mirl. It can be implemented in around 20 lines of code (If you don't use fancy bracket formatting)
//!
//! # Integration:
//!
//! Further infos about every available module can be found within the docstring of that module.
//!
//! How to use:
//! ```
//! use dear_mirl_gui::module_manager::*; // Global functions for module management
//! use dear_mirl_gui::modules; // All the modules that are provided by default
//! use mirl::platform::windowing::traits::*; // Default mirl windowing traits
//! fn run_loop<W: ExtendedWindowingFramework>(
//!     buffer: &mut mirl::prelude::Buffer,
//!     window: &mut W,
//!     font: &mirl::dependencies::fontdue::Font,
//! ) {
//!     // Important! This formatting will be referenced a lot to avoid duplicate code
//!     set_formatting(dear_mirl_gui::Formatting::default(font, 20)); // 20 Being the height of text in pixels
//!     // Define your module - Module struct - Displayed text
//!     let text_display = //   v               v
//!              register_module(modules::Text::new("Hello World!"));
//!     // If you wanna use multiple guis use the DearMirlGuiManager, otherwise use the DearMirlGui directly. The .update() functions are identical.
//!     // In the ::<const FAST: bool, const USE_CACHE: bool> I've set to
//!     // - FAST: false -> Minor Visual drawbacks
//!     // - USE_CACHE: true -> This is honestly a must, it reduces redraw so much that on flamegraph, the only visible module for the test scene is a single animated widget
//!     let mut window_manager: dear_mirl_gui::DearMirlGuiManager<false, true> =
//!         dear_mirl_gui::DearMirlGuiManager::new(Vec::from([
//!             dear_mirl_gui::DearMirlGui::new_simple(
//!                 "Gui Window",
//!                 (100, 10),
//!                 &[text_display.id()],
//!             ),
//!         ]));
//!     while window.is_open() {
//!         // Clearing last frame
//!         buffer.clear();
//!         // Using the data to update all/the window(s)
//!         let gui_output = window_manager.update(
//!             &window.get_mouse_snapshot(),
//!             &window.get_all_keys_down(),
//!             0.0, // Delta time - Required for animated components
//!             &None,
//!         );
//!         if !gui_output.focus_taken {
//!             // The gui didn't take focus, use mouse/keyboard data to update your logic
//!         }
//!         // Standard drawing routine
//!         // ...
//!         // Automatic drawing, for manual drawing use window_manager.draw()
//!         window_manager.draw_on_buffer(buffer);
//!         // Update framework
//!         if let Err(error) = window.update(buffer) {
//!             println!("Error while updating window: {}", error);
//!         }
//!     }
//! }
//! ```
//!
//! ## Examples (Used for internally for testing of modules)
//! ### You can use either of these:
//! - 'cargo test -p dear_mirl_gui --features debug-window -- --nocapture'
//! - 'cargo test -p dear_mirl_gui --release --features debug-window -- --nocapture'
//!
//! ### Or if you also want to see experimental features use these:
//! - 'cargo test -p dear_mirl_gui --features experimental -- --nocapture'
//! - 'cargo test -p dear_mirl_gui --release --features experimental -- --nocapture'
//!
//!
//! ## Currently known problems:
//! Just remember the reason only the bugs are highlighted: there are _way_ more things that work than don't.
//! [{Importance 1..10}] {Problem}
//! ### Visually:
//! **[4]** Button module hover and click highlight only appears in unselected gui except when the text is moving in which case only the click highlight isn't applied
//! **[0]** Lever module is not smooth
//! **[5]** Text/Number input highlighting is wrongly offset (Accumulating fp rounding?)
//!
//! ### Functionally:
//! **[6]** Number input module automatically selects a structure when clicking after the last character (first time selection)
//! **[0]** Text input module automatically selects a structure when clicking after the last character (first time selection)
//! **[2]** Text input module selects itself through other windows
//! **[4]** Text input module 'read_only' field doesn't do anything
//! **[4]** Text input module 'overwrite_mode' field doesn't do anything
//! **[9]** Crank module rotation is slightly offset
//! **[7]** (Plugin makers only) Single insert mode overwrites the image data of other modules, use the replace all option
//! **[4]** Remove few usages of `.unwrap()` that are left, they only occur for very, very, specific circumstances yet we'd rather want to deal with another `Option<T>` than a crash, right?
//! **[2]** Module registered windows are able to be resized outside of their parent container (Fine if already dragging but should not prompt the user to start dragging)
//!
//! ### To add:
//! **[4]** Color picker missing
//! **[4]** Struct editor unfinished
//! **[7]** (Plugin makers only) Return layers not supported
//! **[3]** Text input module drag-to-select is not yet implemented
//! **[2]** Path inline support for: Image, ImageButton, Lever, NumberDisplay, NumberInput, ProgressBar, Selection, Sliders, TextEditor, Text

/// When modules draw pixels on the buffer unsafely, it is possible to write data out of bounds which can crash the process.
///
/// No modules should ever crash due to writing out of bounds data.
/// However, instead of blindingly trusting that I have made not a single error: If the program crashes due to an out of bounds memory write, report the crash with the given code and enable the `draw_safe` flag until a patch has been released
#[cfg(not(feature = "draw_safe"))]
pub const DRAW_SAFE: bool = false;
/// When modules draw safely to a buffer safely, it is impossible to accidentally write out of bounds. This safety comes at a performance cost however.
#[cfg(feature = "draw_safe")]
pub const DRAW_SAFE: bool = true;

#[cfg(feature = "debug-window")]
/// The tests used internally
pub mod test;

pub use mirl;
/// Add, remove, and edit modules
pub mod module_manager;

/// All builtin modules
pub mod modules;

/// The `DearMirlGui` defining file
pub mod gui;
pub use gui::DearMirlGui;

// A struct to handle having multiple guis at once
mod window_manager;
pub use window_manager::*;

/// All required components
pub mod prelude;

#[deprecated = "Please use the new name of this module: DearMirlGuiManager"]
/// A Compatibility helper pointing out that this module has changed names
pub type WindowManager<const FAST: bool, const USE_CACHE: bool> =
    DearMirlGuiManager<FAST, USE_CACHE>;

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

#[cfg(feature = "coordinate_type_f32")]
/// The currently used coordinate system value type
pub type DearMirlGuiCoordinateType = f32;
#[cfg(not(feature = "coordinate_type_f32"))]
/// The currently used coordinate system value type
pub type DearMirlGuiCoordinateType = i32;

/// A trait any struct can implement to be used as a `DearMirlGui` module
pub trait DearMirlGuiModule:
    AnyCasting + std::fmt::Debug + WhatAmI + std::marker::Send
{
    /// Create an internal buffer, draw all desired info on it and return
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (mirl::prelude::Buffer, crate::module_manager::InsertionMode);
    /// Gets the height of a module regardless of the returned buffer height
    fn get_height(
        &mut self,
        formatting: &crate::Formatting,
    ) -> DearMirlGuiCoordinateType;
    /// Gets the width of a module regardless of the returned buffer height
    fn get_width(
        &mut self,
        formatting: &crate::Formatting,
    ) -> DearMirlGuiCoordinateType;
    /// Update the internal state of the module with the given information
    fn update(&mut self, inputs: &crate::ModuleUpdateInfo) -> crate::GuiOutput;
    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    /// Get an offset for the next module
    fn modify_offset_cursor(
        &mut self,
        modules: &[crate::gui::ModuleContainer],
        used_idx: &Vec<usize>,
        formatting: &crate::Formatting,
        current: (
            &mut DearMirlGuiCoordinateType,
            &mut DearMirlGuiCoordinateType,
        ),
    ) {
    }
    #[allow(unused_variables)]
    /// Manually setting wether a module needs a redraw -> Useful when intentionally using corrupting data
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {}
    /// Check if the module needs a redraw
    fn need_redraw(&mut self) -> bool;
    #[allow(unused_variables)]
    /// When you have updated the formatting and wish to properly apply the change to all modules
    fn apply_new_formatting(&mut self, formatting: &crate::Formatting) {}
    #[allow(unused_variables)]
    /// When a element has been added to a container, this will also be called when moving a module from one container to another
    fn added(&mut self, container_id: usize) {}
    #[allow(unused_variables)]
    /// When a module has been removed from a container, this will also be called when moving a module from one container to another
    fn removed(&mut self, container_id: usize) {}
}
// impl<T: 'static + std::fmt::Debug> GuiReturnModuleError<T> {
//     /// # Panics
//     /// If the value is not `Self::AllGood`, it will error with the corresponding error message
//     #[allow(clippy::panic)]
//     #[track_caller]
//     pub fn unwrap(self) -> T {
//         match self {
//             Self::AllGood(value) => value,
//             Self::CastingAsWrongModule {
//                 wrong,
//                 correct,
//                 id,
//             } => {
//                 panic!(
//                     "Module with id '{id}' is being cast as the wrong type;\n\tRequested module type: '{wrong}', \n\tCorrect module type: '{correct}'"
//                 )
//             }
//             Self::UnableToFindID(id, object) => {
//                 panic!(
//                     "Unable to find a module of type {object:?} with the id of {id}"
//                 )
//             }
//             Self::Misc(stuff) => {
//                 panic!("An error occurred: {stuff}")
//             }
//         }
//     }
// }
/// Automatic Any casting for all structs that are 'static
pub trait AnyCasting {
    /// Return self as an Any instance
    fn as_any(&self) -> &dyn std::any::Any;
    /// Return self as a mutable Any instance
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

// Below was the first time I found out you can implement for T
// MAGIC. THIS IS SOME BATSHIT I'D EXPECT PYTHON TO ALLOW ME, NOT RUST
impl<T: 'static> AnyCasting for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
/// Automatic Any cloning for all structs that are 'static
pub trait AnyCloning {
    /// Return self as an Any instance
    fn as_any_cloned(&self) -> Box<dyn std::any::Any>;
}

impl<T: 'static + Clone> AnyCloning for T {
    fn as_any_cloned(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
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
// ###################################################################################
// End of library - Tests ############################################################
// ###################################################################################

#[cfg(feature = "experimental")]
mod struct_editor_test;

#[test]
#[cfg(not(feature = "debug-window"))]
fn main() {
    panic!(
        "The debug-window flag has not been set. You can use either of these:\n - 'cargo test -p dear_mirl_gui --features debug-window -- --nocapture'\n - 'cargo test -p dear_mirl_gui --release --features debug-window -- --nocapture'\n\nOr if you also want to see experimental features use these:\n - 'cargo test -p dear_mirl_gui --features experimental -- --nocapture'\n - 'cargo test -p dear_mirl_gui --release --features experimental -- --nocapture'"
    )
}
#[test]
#[cfg(feature = "debug-window")]
#[allow(dead_code)]
fn main() {
    test::actual_main();
    // let output = std::thread::Builder::new()
    //     .stack_size(4 * mirl::constants::bytes::GB)
    //     .spawn(actual_main);
    // let _ = output.map(|x| x.join()).unwrap();
}
