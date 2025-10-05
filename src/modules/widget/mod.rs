/// Button module
pub mod button;
/// Tick Box
pub mod check_box;
/// Display an image/buffer
pub mod image;
/// Its an image you can click, a button with fancy visuals
pub mod image_button;
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
/// A crank can be cranked to get a rotation
pub mod crank;
/// Display numbers in a seven segment display style
pub mod number_display;
/// A lever you can vertically drag that can be either on or off
pub mod lever;

pub use button::Button;
pub use check_box::CheckBox;
pub use image::Image;
pub use image_button::ImageButton;
pub use progress_bar::ProgressBar;
pub use selection::Selection;
pub use sliders::Slider;
pub use text::Text;
pub use text_input::TextInput;
pub use crank::Crank;
pub use number_display::NumberDisplay;
pub use lever::Lever;

/// Misc function the modules use
pub mod misc;

// pub mod color_picker;
// pub mod selection; // Text selection - Search up combo box AND list box for more info
// pub mod tooltip_area; // A module with "0x0 size" that doesn't take focus but displays text if the cursor has been hovering over its actual area for a while
// pub mod grid; // Imagine a #, each slot can be filled with a single module and the sizes will retain uniformity
