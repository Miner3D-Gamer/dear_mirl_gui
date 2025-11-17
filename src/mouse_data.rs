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
    pub down: bool,
    pub clicked: bool,
    pub released: bool,
}
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The current state of the mouse buttons and if they have just been pressed
pub struct MouseState {
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
}

// pub struct MousePos<T> {
//     pos: (T, T),
//     delta_pos: (T, T),
// }

// pub struct MouseData<T> {
//     buttons: MouseState,
//     pos: MousePos<T>,
// }
