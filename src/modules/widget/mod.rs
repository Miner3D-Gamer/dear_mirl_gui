/// Button module
pub mod button;
/// Tick Box
pub mod check_box;
/// Display an image/buffer
pub mod image;
/// Progress bar
pub mod progress_bar;
/// Button Selection module including a radio button option
pub mod selection;
/// Slider module
pub mod sliders;
/// Text module
pub mod text;
/// A simple text input module
/// 
/// This is currently in a very basic state and will be rewritten in the future
pub mod text_input;
/// A sub window inside your window, idk why you would want this recursion but it's dynamic enough to support it so why not?
/// 
/// Can be used as a collapsable header as well as scrollable container
pub mod window_emulator;

pub use button::Button;
pub use check_box::CheckBox;
pub use image::Image;
pub use progress_bar::ProgressBar;
pub use selection::Selection;
pub use sliders::Slider;
pub use text::Text;
pub use text_input::TextInput;
pub use window_emulator::WindowEmulator;

/// Misc function the modules use
pub mod misc;

// pub mod color_picker;
// pub mod selection; // Text selection - Search up combo box AND list box for more info
// pub mod tooltip_area; // A module with "0x0 size" that doesn't take focus but displays text if the cursor has been hovering over its actual area for a while
// pub mod grid; // Imagine a #, each slot can be filled with a single module and the sizes will retain uniformity