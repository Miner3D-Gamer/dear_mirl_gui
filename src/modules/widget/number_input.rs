use mirl::{
    extensions::*, graphics::rgb_to_u32, math::ConstZero, misc::keybinds::{KeyBind, sort_actions}, platform::{
        CursorStyle,
        keycodes::{KeyCode, StringToKeyCodes},
    }, prelude::Buffer, render, text::position::TextPosition
};

use crate::{
    DRAW_SAFE, DearMirlGuiModule, FocusTaken, ModuleUpdateInfo,
    module_manager::InsertionMode, modules::misc::shimmer, prelude::get_formatting,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// All the keybinds the text input module uses
pub enum Actions {
    /// Remove a single character to the left
    DeleteLeft,
    /// Remove a single character to the right
    DeleteRight,
    /// Remove a "character structure" to the left
    DeleteStructureLeft,
    /// Remove a "character structure" to the right
    DeleteStructureRight,
    /// Removes the current line
    DeleteCurrentLine,
    /// Undo returns the state of the module to before the last action
    Undo,
    /// Redo returns the state of the module to before undo was pressed
    Redo,
    /// Copy what has been highlighted or the current line
    Copy,
    /// Copy and remove what has been highlighted or the current line
    Cut,
    /// Insert your clipboard data into the writing area (Notice: this happens with a frame of delay since the module needs to request the clipboard data first)
    RequestPaste,
    /// Move the caret position one line up
    MoveUp,
    /// Move the caret position one line up and highlight everything between the old and new positions
    MoveUpAndHighlight,
    /// Move the caret position one line down
    MoveDown,
    /// Move the caret position one line down and highlight everything between the old and new positions
    MoveDownAndHighlight,
    /// Move the caret position a single column to the left
    MoveLeft,
    /// Move the caret position left until the end of the determined structure
    MoveStructureLeft,
    /// Move the caret position a single column to the left and highlight everything between the old and new positions
    MoveLeftAndHighlight,
    /// Move the caret position left until the end of the determined structure and highlight everything between the old and new positions
    MoveStructureLeftAndHighlight,
    /// Move the caret position a single column to the right
    MoveRight,
    /// Move the caret position right until the end of the determined structure
    MoveStructureRight,
    /// Move the caret position a single column to the right and highlight everything between the old and new positions
    MoveRightAndHighlight,
    /// Move the caret position right until the end of the determined structure and highlight everything between the old and new positions
    MoveStructureRightAndHighlight,
    /// Swap the current line with the line below
    ToggleOverwrite,
    /// Select all text
    SelectAll,
    /// Moves the caret position to he last column in the current line
    MoveToEndOfLine,
    /// Moves the caret position to the front of the current line
    MoveToStartOfLine,
    /// Select the line the caret is positioned at
    SelectLine,
}

/// Get a keybind layout deemed good enough by me
#[must_use]
#[allow(clippy::too_many_lines)] // What am I supposed to do? Create a unique function for _every_ keybind?
pub fn default_keybind_layout() -> Vec<KeyBind<Actions>> {
    Vec::from([
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::Backspace],
            Actions::DeleteLeft,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::Delete],
            Actions::DeleteRight,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::Backspace],
            Actions::DeleteStructureLeft,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::Delete],
            Actions::DeleteStructureRight,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::Delete],
            Actions::DeleteCurrentLine,
        ),
        KeyBind::new(false, false, true, vec![KeyCode::Z], Actions::Undo),
        KeyBind::new(false, false, true, vec![KeyCode::Y], Actions::Redo),
        KeyBind::new(false, false, true, vec![KeyCode::C], Actions::Copy),
        KeyBind::new(false, false, true, vec![KeyCode::X], Actions::Cut),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::V],
            Actions::RequestPaste,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::UpArrow],
            Actions::MoveUp,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::DownArrow],
            Actions::MoveDown,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::LeftArrow],
            Actions::MoveLeft,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::RightArrow],
            Actions::MoveRight,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::RightArrow],
            Actions::MoveRightAndHighlight,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::DownArrow],
            Actions::MoveDownAndHighlight,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::UpArrow],
            Actions::MoveUpAndHighlight,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::LeftArrow],
            Actions::MoveLeftAndHighlight,
        ),
        KeyBind::new(
            true,
            false,
            true,
            vec![KeyCode::RightArrow],
            Actions::MoveStructureRightAndHighlight,
        ),
        KeyBind::new(
            true,
            false,
            true,
            vec![KeyCode::LeftArrow],
            Actions::MoveStructureLeftAndHighlight,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::RightArrow],
            Actions::MoveStructureRight,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::LeftArrow],
            Actions::MoveStructureLeft,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::Insert],
            Actions::ToggleOverwrite,
        ),
        KeyBind::new(false, false, true, vec![KeyCode::A], Actions::SelectAll),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::End],
            Actions::MoveToEndOfLine,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::Home],
            Actions::MoveToStartOfLine,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::End],
            Actions::MoveToEndOfLine,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::Home],
            Actions::MoveToStartOfLine,
        ),
        KeyBind::new(false, false, true, vec![KeyCode::D], Actions::SelectAll),
        KeyBind::new(false, false, true, vec![KeyCode::L], Actions::SelectLine),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::BrowserBack],
            Actions::Undo,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::BrowserForward],
            Actions::Redo,
        ),
    ])
}
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(clippy::doc_markdown)]
/// Default Keybinds:
/// - Backspace                     -> Delete a single character to the left
/// - Backspace + Control           -> Deletes the character structure to the left
/// - Delete                        -> Delete a single character to the right
/// - Delete + Control              -> Deletes the character structure to the right
/// - Delete + Shift                -> Delete the current line
/// - Enter / KeyPad Enter          -> Creates a new line
/// - Enter / KeyPad Enter + Shift  -> Creates a new line without moving text
/// - Tab                           -> Indent
/// - Tab + Control                 -> Indent at line start
/// - Tab + Shift                   -> Outdent at line start
/// - Tab + Control + Shift         -> Outdent
/// - Left                          -> Move caret left / to previous line
/// - Left + Control                -> Move the caret left while skipping character structures
/// - Left + Shift                  -> Move caret left and highlight
/// - Left + Shift + Control        -> Move caret left skipping structures and highlight
/// - Right                         -> Move caret right / to next line
/// - Right + Control               -> Move the caret right while skipping character structures
/// - Right + Shift                 -> Move caret right and highlight
/// - Right + Shift + Control       -> Move caret right skipping structures and highlight
/// - Up                            -> Move caret up one line
/// - Up + Alt                      -> Swap current line with the line above
/// - Up + Alt + Shift              -> Duplicate line above
/// - Up + Control                  -> Move the caret up while skipping character structures
/// - Up + Shift                    -> Move caret up and highlight
/// - Up + Shift + Control          -> Move caret up skipping structures and highlight
/// - Down                          -> Move caret down one line
/// - Down + Alt                    -> Swap current line with the line below
/// - Down + Alt + Shift            -> Duplicate line and move cursor to new line
/// - Down + Alt + Shift            -> Duplicate line below
/// - Down + Control                -> Move the caret down while skipping character structures
/// - Down + Shift                  -> Move caret down and highlight
/// - Down + Shift + Control        -> Move caret down skipping structures and highlight
/// - End + Control                 -> Move to end of document
/// - End + Shift                   -> Move to end of line
/// - Home + Control                -> Move to start of document
/// - Home + Shift                  -> Move to start of line
/// - Insert                        -> Toggle overwrite mode
/// - Y + Control                   -> Redo
/// - Z + Control                   -> Undo
/// - C + Control                   -> Copy current line(s)
/// - X + Control                   -> Cut current line(s)
/// - V + Control                   -> Paste text at current line(s)
/// - F + Control                   -> Toggle search window
/// - H + Control                   -> Toggle replace window
/// - G + Control                   -> Move to line
/// - L + Control                   -> Select line
/// - D + Control                   -> Select structure
/// - A + Control                   -> Select all
///
/// TODO:
/// - Fix Undo/Redo to me more consistent
/// - Caret blinking animation
/// - Automatic structure completion (Brackets)
/// - Ctrl + F Search
/// - Ctrl + H Replace
/// - Advanced mouse highlight selection (double/triple click)
/// - Whitespace visualization
/// - Ctrl + G go to line
/// - f3 select next occurrence of current word
/// - Overwrite Mode (Insert key, instead of inserting letters they are replaced)
/// - Regex support
/// - Generic commands -> Strip whitespace, make all lower/upper/title case, deduplicate lines
/// - Bookmarks
/// - Number editing -> Alt + Y => += 1 | Alt + X => -= 1
/// - Statistics -> Words, letters, whitespace, total size, lines
/// - Some keybind to invert the selection/highlights
/// - Multi caret support (+ Multi selection)
/// - ~~Camera/Scrolling~~
/// - ~~Carets Movement~~
/// - ~~Multiline support~~
/// - ~~Automatic mouse position to caret position~~
/// - ~~Arrow Movement~~
/// - ~~Delete key Support~~
/// - ~~Undo/Redo System~~
/// - ~~Copy Support~~
/// - ~~Paste Support~~
/// - ~~Cut Support~~
/// - ~~Read Only Option~~
/// - ~~Placeholder text when input is empty~~
/// - ~~Ctrl + Up/Down for vertical letter skipping~~
/// - ~~Automatic indent detection~~
/// - ~~Advanced caret movement [Home/End keys (beginning/end of line), Ctrl+Home/End (beginning/end of document), Bracket matching (Bracket is portal)]~~
/// - ~~Show line number~~
/// - ~~Text selection/highlighting (Shift selection, ctrl + a)~~
#[allow(clippy::struct_excessive_bools)]
pub struct NumberInput<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + 'static
        + mirl::extensions::TryFromPatch<String>
        + Clone,
> {
    /// The width of the writeable section + line counter
    pub width: usize,
    /// The height of individual lines
    pub height: f32,
    /// The text the input contains
    pub number: T,
    /// If the module needs to be redrawn
    pub needs_redraw: bool,
    /// If the button has been selected
    pub selected: usize,
    /// Used for detecting new key strokes
    pub last_keys_pressed: Vec<KeyCode>,
    /// The vertical, and horizontal position of the cursors.
    pub caret: Caret,
    /// When at the front of a string, should pressing backspace allow you to delete what is behind? The objective answer is no, the subjective answer is 'Let it be configurable'.
    pub remove_behind: bool,
    /// Last text states allowing for ctrl+z/ctrl+y
    pub last_states: Vec<NumberState<T>>,
    /// The current state so the functions don't get confused what state is currently used
    pub current_state: usize,
    /// If you can select the input field
    pub read_only: bool,
    /// Keybinds
    pub keybinds: Vec<KeyBind<Actions>>,
    /// Lets the caret wrap around from the start to the end/end to the start
    pub allow_caret_wrap: bool,
    /// The camera
    pub camera: mirl::misc::ScrollableCamera,
    /// When in overwrite mode, instead of inserting characters, characters that already exist will be overwritten
    pub overwrite_mode: bool,
    /// What characters are/aren't allowed
    pub blacklist: Vec<KeyCode>,
    /// If the purpose of the blacklist should be inverted
    pub blacklist_is_whitelist: bool,
}
/// Available menus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextInputMenu {
    #[default]
    /// No menu open
    None,
    /// Search for a string
    Search,
    /// Search and replace
    Replace,
    /// Go to the specified line
    SkipToLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
/// The position of a caret
///
/// This is a stripped down version of [crate::modules::text_input::Caret]
pub struct Caret {
    /// Horizontal
    pub column: usize,
    /// The origin of the highlight
    pub highlight_pos: TextPosition,
    /// If highlight is active
    pub highlight_enabled: bool,
}
impl Caret {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(column: usize) -> Self {
        Self {
            column: (column),
            highlight_pos: (TextPosition::new(0, 0)),
            highlight_enabled: (false),
        }
    }
    /// Set the highlight position to the current if there is no highlight
    pub const fn enable_highlight(&mut self) {
        if !self.highlight_enabled {
            self.highlight_enabled = true;
            self.highlight_pos = self.to_position();
        }
    }
    /// Read the function name
    pub const fn set_highlight_origin_to_current_pos(&mut self) {
        self.highlight_pos = self.to_position();
    }
    #[must_use]
    /// Get if the cursor is highlighting something
    pub const fn is_highlighting(&self) -> bool {
        self.highlight_enabled
    }
    /// Disable the highlight
    pub const fn reset_highlighted(&mut self) {
        self.highlight_enabled = false;
    }
    #[must_use]
    /// Convert the caret position into a Position position :)
    pub const fn to_position(&self) -> TextPosition {
        TextPosition::new(0, self.column())
    }
    /// Set the current position to the position of the given position
    pub const fn move_to_pos(&mut self, position: TextPosition) {
        self.column = position.column;
    }
    // /// Move cursor down one d safely
    // pub fn move_down(&mut self, module: &TextInput) {
    //     let pos = self.to_position();
    //     let previous = self.line;
    //     self.line = (module.clamp_to_line_count(self.line + 1));

    //     if previous == self.line {
    //         self.move_to_end_of_this_line(module);
    //     } else if !self.if_not_moved_restore_column(pos, module) {
    //         self.retain_column = (Some(self.to_position()));
    //         self.column = (module.clamp_to_column(self.line, self.column));
    //         self.last_pos = (self.to_position());
    //     }
    // }
    // /// Select the structure the caret is over
    // pub fn select_structure(&mut self, module: &TextInput) {
    //     let left_char_type = mirl::misc::skipping_text_type::get_char_type(
    //         module
    //             .get_character(self.line(), self.column().saturating_sub(1))
    //             .unwrap_or_default(),
    //     );
    //     let right_char_type = mirl::misc::skipping_text_type::get_char_type(
    //         module
    //             .get_character(self.line(), self.column())
    //             .unwrap_or_default(),
    //     );
    //     let left = if self.column() == 1
    //         || (self.column() != 0
    //             && left_char_type
    //                 == mirl::misc::skipping_text_type::get_char_type(
    //                     module
    //                         .get_character(self.line(), self.column() - 2)
    //                         .unwrap_or_default(),
    //                 ))
    //     {
    //         mirl::misc::skipping_text_type::skip_char_type_backward(
    //             &module.text[self.line()],
    //             self.column(),
    //         )
    //     } else if right_char_type == left_char_type {
    //         self.column().saturating_sub(1)
    //     } else {
    //         self.column()
    //     };
    //     let right = if right_char_type
    //         == mirl::misc::skipping_text_type::get_char_type(
    //             module
    //                 .get_character(self.line(), self.column() + 1)
    //                 .unwrap_or_default(),
    //         ) {
    //         mirl::misc::skipping_text_type::skip_char_type(
    //             &module.text[self.line()],
    //             self.column(),
    //         )
    //     } else if right_char_type == left_char_type {
    //         self.column() + 1
    //     } else {
    //         self.column()
    //     };
    //     self.column = (left);
    //     self.enable_highlight();
    //     self.column = (right);
    // }
    /// Get the highlighted area as 2 positions
    #[must_use]
    pub const fn get_highlighted_area(&self) -> (TextPosition, TextPosition) {
        (self.highlight_pos, self.to_position())
    }
    // /// Move cursor up one space safely
    // pub fn move_up(&self, module: &TextInput) {
    //     let previous = self.line;
    //     self.line=(self.line.saturating_sub(1));

    //     if previous == self.line {
    //         self.column=(0);
    //     } else {
    //         self.column=(
    //             module.clamp_to_column(self.line, self.column),
    //         );
    //     }
    // }

    /// Get the current column
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column // I forgot to set this to column instead of line, damn you copy paste
    }
    // pub fn set_line(&self, value: usize) {
    //     self.line=(value)
    // }
    // pub fn set_column(&self, value: usize) {
    //     self.column=(value)
    // }
}
/// Movement
impl<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + 'static
        + mirl::extensions::TryFromPatch<String>
        + PartialEq
        + mirl::math::ConstZero
        + Clone,
> NumberInput<T>
{
    /// Sets the caret position to the end of the current line
    pub fn move_to_end_of_this_line(&mut self) {
        self.caret.column = self.get_line_length();
        self.move_camera_to_move_caret_into_view(&get_formatting());
    }
    /// Sets the caret position to the start of the current line
    pub fn move_to_start_of_this_line(&mut self) {
        self.caret.column = 0;
        self.move_camera_to_move_caret_into_view(&get_formatting());
    }
    /// Move the caret postion the the end of the next detected structure
    pub fn move_left_by_structure(&mut self) {
        if self.caret.column == 0 {
            self.move_left();
        }
        self.caret.column = 0;
        self.move_camera_to_move_caret_into_view(&get_formatting());
    }
    /// Move the caret postion the the end of the next detected structure
    pub fn move_right_by_structure(&mut self) {
        if self.caret.column == self.get_line_length() {
            self.move_right();
        }
        self.caret.column = self.number.to_string().chars().count();
        self.move_camera_to_move_caret_into_view(&get_formatting());
    }

    /// Move cursor up one space safely
    pub fn move_up(&mut self) {
        self.move_to_start_of_this_line();
    }
    /// Move the caret one space to the right
    pub fn move_right(&mut self) {
        if self.caret.is_highlighting() {
            let pos1 = self.caret.highlight_pos;
            let pos2 = self.caret.to_position();
            if pos1 < pos2 {
                self.caret.move_to_pos(pos2);
            } else {
                self.caret.move_to_pos(pos1);
            }
        } else if self.caret.column == self.get_line_length() {
            if self.allow_caret_wrap {
                self.caret.column = 0;
            }
        } else {
            self.caret.column += 1;
        }
        self.move_camera_to_move_caret_into_view(&get_formatting());
    }
    /// Move the caret down a line
    pub fn move_down(&mut self) {
        self.move_to_end_of_this_line();
    }
    /// Move the caret one space to the left
    pub fn move_left(&mut self) {
        if self.caret.is_highlighting() {
            let pos1 = self.caret.highlight_pos;
            let pos2 = self.caret.to_position();
            if pos1 > pos2 {
                self.caret.move_to_pos(pos2);
            } else {
                self.caret.move_to_pos(pos1);
            }
        } else if self.caret.column == 0 {
            if self.allow_caret_wrap {
                self.caret.column = self.get_line_length();
            }
        } else {
            self.caret.column = self.caret.column.saturating_sub(1);
        }
        self.move_camera_to_move_caret_into_view(&get_formatting());
    }
}
/// Selection
impl<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + PartialEq
        + 'static
        + ConstZero
        + mirl::extensions::TryFromPatch<String>
        + Clone,
> NumberInput<T>
{
    /// Select all text available
    pub fn select_all(&mut self) {
        self.move_to_start_of_this_line();
        self.caret.set_highlight_origin_to_current_pos();
        self.caret.highlight_enabled = true;
        self.move_to_end_of_this_line();
    }
    /// Select the structure around the caret
    pub fn select_structure(&mut self) {
        self.caret.column = 0;
        self.caret.enable_highlight();
        self.caret.column = self.number.to_string().chars().count();
    }
}
/// Text unaltering
impl<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + 'static
        + mirl::extensions::TryFromPatch<String>
        + PartialEq
        + ConstZero
        + Clone,
> NumberInput<T>
{
    #[inline(always)]
    #[must_use]
    #[allow(clippy::inline_always)]
    /// Get the length of a line
    pub fn get_line_length(&self) -> usize {
        self.number.to_string().chars().count()
    }
    #[inline(always)]
    #[must_use]
    #[allow(clippy::inline_always)]
    /// Clamp a value to the line count
    pub const fn clamp_to_line_count(&self, other: usize) -> usize {
        self.line_count_idx().min(other)
    }
    #[inline(always)]
    #[allow(clippy::inline_always)]
    #[must_use]
    /// Get the line count
    pub const fn line_count_idx(&self) -> usize {
        0
    }
    #[inline(always)]
    #[must_use]
    #[allow(clippy::inline_always)]
    /// Clamp a value to the column of the specified line
    pub fn clamp_to_column(&self, column: usize) -> usize {
        self.get_line_length().min(column)
    }
    #[must_use]
    /// Get a single character from line, column
    pub fn get_character(&self, column: usize) -> Option<char> {
        self.number.to_string().chars().nth(column)
    }
    #[must_use]
    /// Get the horizontal offset the text is experiencing
    pub const fn get_horizontal_text_offset(
        &self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        formatting.horizontal_margin as crate::DearMirlGuiCoordinateType
            + self.camera.offset_x as crate::DearMirlGuiCoordinateType
    }
    #[must_use]
    /// Get selected text in area as a list of strings
    pub fn get_selected_area(
        &self,
        pos: (TextPosition, TextPosition),
    ) -> Vec<String> {
        let (line_start, line_end, front_pos, back_pos) = {
            let head = pos.0;
            let tail = pos.1;
            if head.line > tail.line {
                (tail.line, head.line, tail.column, head.column)
            } else if tail.line > head.line || tail.column > head.column {
                (head.line, tail.line, head.column, tail.column)
            } else {
                (head.line, tail.line, tail.column, head.column)
            }
        };

        if line_start == line_end && front_pos == back_pos {
            return Vec::new();
        }

        let total_lines = line_end - line_start;

        if total_lines == 0 {
            let line_chars: Vec<char> =
                self.number.to_string().chars().collect();
            let selected_chars: Vec<char> =
                line_chars[front_pos..back_pos].to_vec();
            let selected_text: String = selected_chars.into_iter().collect();
            vec![selected_text]
        } else {
            let mut result = Vec::new();

            let first_line_chars: Vec<char> =
                self.number.to_string().chars().collect();
            let first_line_selected: String =
                first_line_chars[front_pos..].iter().collect();
            result.push(first_line_selected);

            let last_line_chars: Vec<char> =
                self.number.to_string().chars().collect();
            let last_line_selected: String =
                last_line_chars[..back_pos].iter().collect();
            result.push(last_line_selected);

            result
        }
    }
    /// Move the camera to make sure a caret is visible
    pub fn move_camera_to_move_caret_into_view(
        &mut self,
        formatting: &crate::Formatting,
    ) {
        self.view_position(self.caret.to_position(), formatting);
    }
    /// Move the camera to view a position
    ///
    /// This function has to be rewritten!!!!!!!!!
    pub fn view_position(
        &mut self,
        pos: TextPosition,
        formatting: &crate::Formatting,
    ) {
        let target_y = (pos.line + 1) as f32 * self.height;
        let target_x = render::get_text_width(
            &self
                .number
                .to_string()
                .chars()
                .take(pos.column)
                .collect::<String>(),
            self.height as f32,
            &formatting.font,
        );

        let margin = self.height as f32;
        let viewport_width = self.get_width(formatting) as f32;
        let viewport_height = self.get_height(formatting) as f32;

        // Calculate visible bounds (accounting for camera offset)
        let visible_left = (-self.camera.offset_x).max(0.0);
        let visible_top = (-self.camera.offset_y).max(0.0);
        let visible_right = visible_left + viewport_width;
        let visible_bottom = visible_top + viewport_height;

        // Adjust camera horizontally if needed
        if target_x < visible_left + margin {
            self.camera.offset_x = -(target_x - margin);
        } else if target_x > visible_right - margin {
            self.camera.offset_x = -(target_x + margin - viewport_width);
        }

        // Adjust camera vertically if needed
        if (target_y as f32) < visible_top + margin {
            self.camera.offset_y = -(target_y as f32 - margin);
        } else if target_y as f32 + self.height as f32 > visible_bottom - margin
        {
            self.camera.offset_y =
                -(target_y as f32 + self.height as f32 + margin
                    - viewport_height);
        }
        self.camera.clamp_to_bounds();
    }

    /// Get the horizontal offset the text is experiencing
    #[must_use]
    pub const fn get_vertical_text_offset(
        &self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        formatting.vertical_margin as crate::DearMirlGuiCoordinateType
            + self.camera.offset_y as crate::DearMirlGuiCoordinateType
    }
    /// Applies the scroll to the camera
    pub fn handle_scroll(
        &mut self,
        scroll: (f32, f32),
        switch: bool,
        formatting: &crate::Formatting,
    ) {
        self.camera.container_width = self.get_width(formatting) as f32;
        self.camera.container_height = self.get_height(formatting) as f32;
        let size = self.get_content_size(&formatting.font);
        self.camera.content_height = size.1;
        self.camera.content_width = size.0;
        self.camera.scroll(scroll, switch);
        // if switch {
        //     self.camera.0 = self.camera.0.add(scroll.1 * self.scroll_mul.1);
        //     self.camera.1 = self.camera.1.add(scroll.0 * self.scroll_mul.0);
        // } else {
        //     self.camera.0 = self.camera.0.add(scroll.0 * self.scroll_mul.0);
        //     self.camera.1 = self.camera.1.add(scroll.1 * self.scroll_mul.1);
        // }
    }
    fn get_longest_line(&self) -> String {
        self.number.to_string()
    }
    fn get_content_size(
        &self,
        font: &mirl::dependencies::fontdue::Font,
    ) -> (f32, f32) {
        let width =
            render::get_text_width(&self.get_longest_line(), self.height, font)
                * 1.5; // Theoretically not "good", practically it works better than intended and at the end of the day, that's the only thing that counts:)
        let height = self.height;
        (width, height as f32)
    }
}

/// A List of text, the caret position, and what is highlighted
pub type NumberState<T> = (T, Caret);

impl<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + 'static
        + mirl::extensions::TryFromPatch<String>
        + Clone,
> NumberInput<T>
{
    /// Set the current number of the input
    pub fn set_number(&mut self, number: T) {
        self.needs_redraw = true;
        self.caret.highlight_enabled = false;
        self.caret.column =
            self.caret.column.min(number.to_string().chars().count());
        self.number = number;
    }
    #[must_use]
    /// Set the current width, use [with_height](Self::with_height) for setting the height
    pub const fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    #[must_use]
    /// Set the current width, use [with_width](Self::with_width) for setting the width
    pub const fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + 'static
        + ConstZero
        + core::cmp::PartialEq
        + mirl::extensions::TryFromPatch<String>
        + Clone,
> NumberInput<T>
{
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(number: T) -> Self {
        let formatting = get_formatting();
        let height = formatting.height as f32;
        Self {
            width: {
                mirl::render::get_text_width(
                    &number.to_string(),
                    height,
                    &get_formatting().font,
                ) as usize
            },
            height,
            // Yummy
            caret: Caret::new(100),
            //line_number_offset: number.to_string().chars().count() / 10,
            number,
            needs_redraw: (true),
            selected: 0,
            last_keys_pressed: Vec::new(),
            //highlighted: std::default::Default::default(),
            remove_behind: false,
            last_states: Vec::new(),
            read_only: false,
            keybinds: default_keybind_layout(),
            allow_caret_wrap: false,
            camera: mirl::misc::ScrollableCamera {
                container_width: 0.0,
                container_height: 0.0,
                content_width: 0.0,
                content_height: 0.0,
                offset_x: 0.0,
                offset_y: 0.0,
                scroll_multiplier_x: 1.0,
                scroll_multiplier_y: 1.0,
                horizontal_context_switch_multipliers: true,
                allow_free_scroll: false,
            },
            overwrite_mode: false,
            blacklist: Vec::new(),
            blacklist_is_whitelist: false,
            current_state: 0,
        }
    }

    // /// If any selection has been made
    // pub const fn is_something_highlighted(&self) -> bool {
    //     self.highlighted.0.line != self.highlighted.1.line
    //         || self.highlighted.0.column != self.highlighted.1.column
    // }
    /// Delete all text in all area
    pub fn delete_text_in_area(
        &mut self,
        pos: (TextPosition, TextPosition),
    ) -> Option<()> {
        let (front_pos, back_pos) = {
            let head = pos.0;
            let tail = pos.1;
            if tail.column > head.column {
                (head.column, tail.column)
            } else {
                (tail.column, head.column)
            }
        };

        if front_pos == back_pos {
            return Some(());
        }

        let mut line_chars: Vec<char> =
            self.number.to_string().chars().collect();
        line_chars.drain(front_pos..back_pos);
        self.number = T::try_from_value(line_chars.into_iter().collect())?;
        self.caret.column = front_pos;

        self.caret.reset_highlighted();
        Some(())
    }
    /// Delete a single character to the right
    pub fn delete_right(&mut self) {
        if self.caret.is_highlighting() {
            self.delete_text_in_area(self.caret.get_highlighted_area());
        } else {
            if self.caret.is_highlighting() {
                self.delete_text_in_area(self.caret.get_highlighted_area());
                return;
            }
            self.remove_chars_from_line(self.caret.column(), 1);
        }
    }
    // pub fn move_caret_in_area

    // /// This function adds/subtracts highlighted sections
    // pub fn toggle_highlight(&mut self, from: Position, to: Position) {
    //     if self.is_something_highlighted() {
    //         if from == self.highlighted.1 {
    //             self.highlighted.1 = to;
    //         } else if to == self.highlighted.1 {
    //             self.highlighted.1 = from;
    //         }
    //     } else {
    //         self.highlighted = (from, to);
    //     }
    // }

    // #[inline(always)]
    // #[allow(clippy::inline_always)]
    // /// Sets the highlighted region into an invalid state
    // pub fn reset_highlighted(&mut self) {
    //     self.highlighted = std::default::Default::default();
    // }
    #[inline(always)]
    #[allow(clippy::inline_always)]
    /// Insert a string into the middle of a line
    pub fn remove_chars_from_line(
        &mut self,
        at: usize,
        amount: usize,
    ) -> Option<()> {
        let t = self.number.to_string();
        let before = t.chars().count();

        let (l, r) = t.split_at(at);
        let mut l = l.to_string();
        if l.is_empty() {
            l = "0".to_string();
        }
        if r.chars().count() <= amount {
            self.number = T::try_from_value(l)?;
        } else {
            self.number = T::try_from_value(
                l + &r.chars().skip(amount).collect::<String>(),
            )?;
        }
        let after = self.number.to_string().chars().count();
        let dif = after as isize - before as isize;
        //dif.println_self();
        self.move_cursor(dif);
        Some(())
    }
    /// Move the cursor left or right by X amounts
    pub fn move_cursor(&mut self, by: isize) {
        if by == 0 {
            return;
        }
        if by > 0 {
            self.caret.column = self.caret.column.saturating_sub(by as usize);
            // for _ in 0..by as usize {
            //     self.move_left();
            // }
        } else {
            self.caret.column = self
                .caret
                .column
                .saturating_add(by as usize)
                .min(self.number.to_string().chars().count());
            // for _ in 0..by as usize {
            //     self.move_right();
            // }
        }
        //println!("What");
    }
    #[inline(always)]
    #[allow(clippy::inline_always)]
    /// Insert a string into the middle of a line
    pub fn insert_chars_from_line(
        &mut self,
        at: usize,
        chars: &str,
    ) -> Option<()> {
        let t = self.number.to_string();
        let (l, r) = t.split_at(at);
        self.number = T::try_from_value(l.to_string() + chars + r)?;
        Some(())
    }
    /// Delete the whole line the caret is on
    pub fn delete_current_line(&mut self) {
        self.number = T::ZERO;
        self.caret.column = 0;
    }
    /// Delete the structure detected to the left
    pub fn delete_structure_left(&mut self) {
        self.remove_chars_from_line(0, self.caret.column());
        self.caret.column = 0;
    }
    /// Delete the structure detected to the right
    pub fn delete_structure_right(&mut self) {
        self.remove_chars_from_line(
            self.caret.column(),
            self.get_line_length() - self.caret.column(),
        );
    }

    /// Delete a single character to the left
    pub fn delete_left(&mut self) {
        if self.caret.is_highlighting() {
            self.delete_text_in_area(self.caret.get_highlighted_area());
        } else if self.caret.column() == 0 {
        } else {
            //self.move_left();
            self.remove_chars_from_line(self.caret.column.saturating_sub(1), 1);
        }
    }
    /// Undo the last action
    pub fn undo(&mut self) {
        // println!("{}", self.current_state);
        if !self.last_states.is_empty() {
            //println!("Set");
            self.current_state -= 1;
            (self.number, self.caret) =
                self.last_states[self.current_state].clone();
        }
    }

    /// Writes out the keycodes at the caret position
    pub fn write(&mut self, keycodes: &[KeyCode]) -> Option<()> {
        if self.read_only {
            return Some(());
        }
        for keycode in keycodes {
            if self.blacklist.contains(keycode) != self.blacklist_is_whitelist {
                continue;
            }
            if !keycode.is_number() {
                continue;
            }
            if let Some(value) = keycode.to_user_friendly_string() {
                let before: String = self
                    .number
                    .to_string()
                    .chars()
                    .take(self.caret.column())
                    .collect();
                let after: String = self
                    .number
                    .to_string()
                    .chars()
                    .skip(self.caret.column())
                    .collect();
                let text = before + value + &after;
                self.number = T::try_from_value(text.clone())?; // TODO: REMOVE CLONE
                let after = self.number.to_string();
                let add = after.chars().count() as isize + 1
                    - text.chars().count() as isize;
                self.caret.column.set_add_sign(add);
            }
        }
        Some(())
    }
    /// Redo the last undo
    pub fn redo(&mut self) {
        if self.current_state + 1 < self.last_states.len() {
            self.current_state += 1;
            (self.number, self.caret) =
                self.last_states[self.current_state].clone();
        }
    }

    #[allow(clippy::too_many_lines)]
    fn handle_keybinds(
        &mut self,
        keybinds: &Vec<KeyBind<Actions>>,
    ) -> (bool, bool, Option<mirl::platform::file_system::FileData>) {
        let mut do_not_save_new_state = false;
        let mut request_clipboard_data = false;
        let mut new_clipboard_data = None;
        for i in keybinds {
            match i.action {
                // Simple Movement
                Actions::MoveRight => {
                    self.move_right();
                    self.caret.reset_highlighted();
                }
                Actions::MoveLeft => {
                    self.move_left();
                    self.caret.reset_highlighted();
                }
                Actions::MoveUp => {
                    self.move_up();
                    self.caret.reset_highlighted();
                }
                Actions::MoveDown => {
                    self.move_down();
                    self.caret.reset_highlighted();
                }
                // Simple deletion
                Actions::DeleteLeft => {
                    //let caret = self.caret.clone();
                    self.delete_left();
                    // caret.delete_left(self);
                    // self.caret = caret;
                }
                Actions::DeleteRight => {
                    self.delete_right();
                    // let caret = self.caret.clone();
                    // caret.delete_right(self);
                    // self.caret = caret;
                }
                Actions::Copy => {
                    if self.caret.is_highlighting() {
                        new_clipboard_data = Some(mirl::platform::file_system::FileData::from_list_of_strings(&self.get_selected_area(self.caret.get_highlighted_area())));
                    } else {
                        new_clipboard_data = Some(
                            mirl::platform::file_system::FileData::from_string(
                                self.number.to_string(),
                            ),
                        );
                    }
                }
                // Clipboard stuff
                Actions::Cut => {
                    if self.caret.is_highlighting() {
                        new_clipboard_data =Some(mirl::platform::file_system::FileData::from_list_of_strings(&self.get_selected_area(self.caret.get_highlighted_area())));
                        self.delete_text_in_area(
                            self.caret.get_highlighted_area(),
                        );
                    } else {
                        new_clipboard_data = Some(
                            mirl::platform::file_system::FileData::from_string(
                                self.number.to_string().clone(),
                            ),
                        );
                        self.number = T::ZERO;
                        self.caret.column = 0;
                    }
                }
                Actions::RequestPaste => {
                    request_clipboard_data = true;
                }
                // Misc
                Actions::DeleteCurrentLine => {
                    self.delete_current_line();
                }
                Actions::DeleteStructureLeft => {
                    self.delete_structure_left();
                }
                Actions::DeleteStructureRight => {
                    self.delete_structure_right();
                }
                Actions::MoveStructureLeft => {
                    self.move_left_by_structure();
                    self.caret.reset_highlighted();
                }
                Actions::MoveStructureRight => {
                    self.move_right_by_structure();
                    self.caret.reset_highlighted();
                }
                Actions::Undo => {
                    do_not_save_new_state = true;
                    self.undo();
                }
                Actions::Redo => {
                    do_not_save_new_state = true;
                    self.redo();
                }
                Actions::ToggleOverwrite => {
                    self.overwrite_mode = !self.overwrite_mode;
                }
                Actions::SelectAll => {
                    self.select_all();
                }
                Actions::MoveToEndOfLine => {
                    self.move_to_end_of_this_line();
                }
                Actions::MoveToStartOfLine => {
                    self.move_to_start_of_this_line();
                }
                Actions::SelectLine => {
                    self.move_to_start_of_this_line();
                    self.caret.set_highlight_origin_to_current_pos();
                    self.move_to_end_of_this_line();
                }
                Actions::MoveDownAndHighlight => {
                    self.caret.enable_highlight();
                    self.move_down();
                }
                Actions::MoveLeftAndHighlight => {
                    self.caret.enable_highlight();
                    self.move_left();
                }
                Actions::MoveRightAndHighlight => {
                    self.caret.enable_highlight();
                    self.move_right();
                }
                Actions::MoveUpAndHighlight => {
                    self.caret.enable_highlight();
                    self.move_up();
                }
                Actions::MoveStructureLeftAndHighlight => {
                    self.caret.enable_highlight();
                    self.move_left_by_structure();
                }
                Actions::MoveStructureRightAndHighlight => {
                    self.caret.enable_highlight();
                    self.move_right_by_structure();
                }
            }
        }

        (do_not_save_new_state, request_clipboard_data, new_clipboard_data)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)] // Idc if it looks 'complex', it mostly works so I'm not changing it
    /// Takes in the keycodes and writes/deleted/moves cursor
    pub fn handle_keycodes(
        &mut self,
        new_keycodes: &[KeyCode],
        held_keycodes: &[KeyCode],
        info: &ModuleUpdateInfo,
    ) -> (bool, (bool, bool, Option<mirl::platform::file_system::FileData>))
    {
        let mut new_keycodes = new_keycodes.to_vec();

        //let mut simplify_highlighted = false;
        let control_down = held_keycodes.contains(&KeyCode::LeftControl)
            || held_keycodes.contains(&KeyCode::RightControl);
        let alt_down = held_keycodes.contains(&KeyCode::LeftAlt)
            || held_keycodes.contains(&KeyCode::RightAlt);
        let shift_down = held_keycodes.contains(&KeyCode::LeftShift)
            || held_keycodes.contains(&KeyCode::RightShift);

        let mut active_keybinds = Vec::new();

        for keybind in self.keybinds.clone() {
            //let k = keybind.keybind();
            let active = keybind.is_keybind_activated(
                &new_keycodes,
                shift_down,
                alt_down,
                control_down,
            );
            if active {
                active_keybinds.push(keybind);
            }
        }
        let new_actions = sort_actions(active_keybinds);

        for i in &new_actions {
            i.remove_required_keys(&mut new_keycodes);
        }
        let mut changed = !new_actions.is_empty() || !new_keycodes.is_empty();

        let return_value = self.handle_keybinds(&new_actions);

        if let Some(clipboard_data) = info.clipboard_data {
            if let Ok(text_data) = clipboard_data.to_string() {
                changed = true;
                self.write(&text_data.to_keycodes());
            } else if let Some(list_string) =
                clipboard_data.to_list_of_strings()
            {
                changed = true;
                for i in list_string {
                    self.write(&i.to_keycodes());
                }
            }
        }

        self.write(&new_keycodes);
        (changed, return_value)
    }
}

impl<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + 'static
        + mirl::extensions::TryFromPatch<String>
        + ConstZero
        + PartialEq
        + Clone,
> DearMirlGuiModule for NumberInput<T>
{
    #[allow(clippy::too_many_lines)]
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        // Settings
        let text_color = formatting.text_color;
        let text_size_mul = 0.8;
        let background_color_change = -5.0;
        let background_color = formatting.foreground_color;
        let caret_color = formatting.text_color;
        let caret_width = self.height / 10.0;

        let highlight_color = rgb_to_u32(30, 20, 200);

        // Code
        let mut buffer = Buffer::new_empty_with_color(
            (self.width, self.get_height(formatting) as usize),
            mirl::graphics::adjust_brightness_hsl_of_rgb(
                background_color,
                background_color_change,
            ),
        );
        let text = self.number.to_string();

        let y = formatting.vertical_margin as crate::DearMirlGuiCoordinateType
            + self.camera.offset_y as crate::DearMirlGuiCoordinateType;

        // Text line
        render::draw_text_antialiased_isize::<{ crate::DRAW_SAFE }>(
            &mut buffer,
            &self.number.to_string(),
            (self.get_horizontal_text_offset(formatting), y)
                .try_tuple_into()
                .unwrap_or_default(),
            text_color,
            self.height as f32 * text_size_mul,
            &formatting.font,
        );

        if self.caret.is_highlighting()
            && let Some((front_pos, back_pos)) = {
                let head = self.caret.highlight_pos;
                let tail = self.caret.to_position();
                if head == tail {
                    None
                } else if tail.column > head.column {
                    Some((head.column, tail.column))
                } else {
                    Some((tail.column, head.column))
                }
            }
        {
            let text_until = text.chars().take(front_pos).collect::<String>();
            let first_line_offset = render::get_text_width(
                &text_until,
                self.height,
                &formatting.font,
            );

            drop(text_until);
            // Single line selection - this part looks correct
            let text_between = text
                .chars()
                .skip(front_pos)
                .take(back_pos - front_pos)
                .collect::<String>();
            let first_line_width = render::get_text_width(
                &text_between,
                self.height,
                &formatting.font,
            );
            render::execute_at_rectangle::<true>(
                &mut buffer,
                (
                    first_line_offset as isize
                        + self.get_horizontal_text_offset(formatting) as isize,
                    formatting.vertical_margin as isize,
                ),
                (first_line_width as isize, self.height as isize),
                highlight_color,
                shimmer,
            );
        }

        if self.selected == info.container_id {
            let before =
                text.chars().take(self.caret.column()).collect::<String>();

            // The normal x position of the cursor
            let offset = render::get_text_width(
                &before,
                self.height as f32 * text_size_mul,
                &formatting.font,
            );
            render::execute_at_rectangle::<true>(
                &mut buffer,
                (
                    offset as isize
                        + self.get_horizontal_text_offset(formatting) as isize,
                    self.get_vertical_text_offset(formatting) as isize,
                ),
                (caret_width as isize, self.height as isize),
                caret_color,
                mirl::prelude::Buffer::invert_color_if_same::<{ DRAW_SAFE }>,
            );
        }
        // if self.number == T::ZERO {
        //     render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
        //         &mut buffer,
        //         &self.placeholder_text.to_string(),
        //         (
        //             self.get_horizontal_text_offset(formatting) as usize,
        //             self.get_vertical_text_offset(formatting) as usize,
        //         ),
        //         mirl::graphics::adjust_brightness_hsl_of_rgb(
        //             text_color,
        //             placeholder_color_change,
        //         ),
        //         self.height as f32 * text_size_mul,
        //         &formatting.font,
        //     );
        // }
        // render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
        //   &buffer,
        //   &after,
        //   x as usize + offset as usize,
        //
        //   text_color,
        //   self.height as f32 * text_size_mul,
        //   &formatting.font,
        // );

        (buffer, InsertionMode::Simple)
    }
    fn get_height(
        &mut self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        (formatting.vertical_margin as f32)
            .mul_add(2.0, self.height + formatting.vertical_margin as f32)
            as crate::DearMirlGuiCoordinateType
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.width as crate::DearMirlGuiCoordinateType
    }
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        let mut cursor_style = None;
        let formatting = &get_formatting();
        let collision = mirl::math::geometry::Pos2D::<
            _,
            mirl::math::collision::Rectangle<_, false>,
        >::new(
            (0.0, 0.0),
            mirl::math::collision::Rectangle::new((
                self.get_width(formatting) as f32,
                self.get_height(formatting) as f32,
            )),
        );
        //println!("{:?}", self.get_width(formatting) as f32);
        if info.focus_taken == FocusTaken::FunctionallyTaken
            && info.container_id == self.selected
        {
            self.selected = 0;
            self.caret.reset_highlighted();
            self.caret.restore_default();
        }
        let mut took_functional_focus = false;
        //println!("{} {}", self.selected, info.container_id);

        if let Some(mouse_position) = info.mouse_pos {
            let collides = collision.does_area_contain_point(mouse_position);
            // println!(
            //     "\n\n\n\n{:?}\n{:?} ({})",
            //     collision, mouse_position, collides
            // );
            if collides {
                cursor_style = Some(CursorStyle::Text);

                if info.container_id == self.selected {
                    let shift_pressed =
                        info.pressed_keys.contains(&KeyCode::LeftShift)
                            || info.pressed_keys.contains(&KeyCode::RightShift);
                    if info.mouse_scroll != (0.0, 0.0) {
                        took_functional_focus = true;
                        self.handle_scroll(
                            info.mouse_scroll,
                            !shift_pressed,
                            formatting,
                        );
                    }
                }
                if info.mouse_info.left.clicked
                    && (self.selected == 0
                        || self.selected == info.container_id)
                {
                    took_functional_focus = true;
                    self.selected = info.container_id;
                    if true {
                        // println!("Hi");
                        let horizontal =
                            super::misc::get_closest_char_pos_to_mouse_pos(
                                &self.number.to_string(),
                                self.height as f32,
                                &formatting.font,
                                (mouse_position.0
                                    - self
                                        .get_horizontal_text_offset(formatting)
                                        as f32)
                                    as f32,
                            );
                        let new_caret = Caret::new(horizontal);
                        if self.caret == new_caret {
                            self.select_structure();
                        } else {
                            self.caret = new_caret;
                        }
                    }
                }
            } else if info.mouse_info.left.clicked
                && self.selected == info.container_id
            {
                self.selected = 0;
            }
        }
        if self.read_only {
            self.selected = 0;
            cursor_style = Some(CursorStyle::NotAllowed);
        }

        let mut request_clipboard_data = false;
        let mut new_clipboard_data = None;
        if self.selected == info.container_id {
            self.needs_redraw = true;
            let new_keys: Vec<KeyCode> = self
                .last_keys_pressed
                .get_old_items(info.pressed_keys)
                .iter()
                .map(|x| **x)
                .collect();

            let previous_state = self.number.clone();
            if self.last_states.is_empty() {
                self.last_states.push((previous_state.clone(), self.caret));
            }
            let (
                something_changed,
                (
                    do_not_save_state,
                    request_clipboard_data_local,
                    new_clipboard_data_local,
                ),
            ) = self.handle_keycodes(&new_keys, info.pressed_keys, info);
            took_functional_focus = took_functional_focus || something_changed;
            request_clipboard_data = request_clipboard_data_local;
            new_clipboard_data = new_clipboard_data_local;

            if !do_not_save_state && self.number != previous_state {
                if self.current_state < self.last_states.len().saturating_sub(1)
                {
                    self.last_states.truncate(self.current_state);
                }
                self.last_states.push((self.number.clone(), self.caret));
                self.current_state = self.last_states.len() - 1;
                //println!("Added {}", self.current_state);
            }

            self.last_keys_pressed.clone_from(info.pressed_keys);
        }
        let focus_taken: FocusTaken = if took_functional_focus {
            FocusTaken::FunctionallyTaken
        } else if self.selected == info.container_id {
            FocusTaken::VisuallyTaken
        } else {
            FocusTaken::FocusFree
        };

        crate::GuiOutput {
            new_clipboard_data,
            new_cursor_position: None,
            focus_taken,
            hide_cursor: false,
            text_input_selected: self.selected == info.container_id, // TODO: IS THIS CORRECT?
            new_cursor_style: cursor_style,
            request_clipboard_data,
        }
    }
    fn need_redraw(&mut self) -> bool {
        if self.needs_redraw {
            self.needs_redraw = false;
            true
        } else {
            false
        }
    }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw = super::misc::determine_need_redraw(need_redraw);
    }
}

/// Builder functions
impl<
    T: core::fmt::Display
        + std::marker::Send
        + std::marker::Sync
        + core::fmt::Debug
        + 'static
        + mirl::extensions::TryFromPatch<String>
        + Clone,
> NumberInput<T>
{
    /// When at the front of a string, should pressing backspace allow you to delete what is behind?
    #[must_use]
    pub const fn delete_behind(mut self, delete_behind: bool) -> Self {
        self.remove_behind = delete_behind;
        self
    }
    /// When at the start/end of the file, should the carrot be able to wrap to the other side?
    #[must_use]
    pub const fn allow_caret_wrap(mut self, allow_caret_wrap: bool) -> Self {
        self.allow_caret_wrap = allow_caret_wrap;
        self
    }
    /// Set the characters in the current black/white list
    #[must_use]
    pub fn blacklist(mut self, blacklist: Vec<KeyCode>) -> Self {
        self.blacklist = blacklist;
        self
    }
    /// If the blacklist should be treated as a whitelist
    #[must_use]
    pub const fn blacklist_is_whitelist(
        mut self,
        blacklist_is_whitelist: bool,
    ) -> Self {
        self.blacklist_is_whitelist = blacklist_is_whitelist;
        self
    }
}
