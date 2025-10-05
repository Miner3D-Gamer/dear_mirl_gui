/// When instead of skipping to the next line vertically you wanna continue horizontally
pub mod same_line;
/// Set a offset yourself
pub mod custom_offset;
/// A decorational divider
pub mod line;
/// Reset any offset formatting and return to the normal
pub mod reset_offset;

pub use same_line::SameLine;
pub use custom_offset::CustomOffset;
pub use line::Separator;
pub use reset_offset::ResetOffset;
