use mirl::graphics::rgb_to_u32;
use mirl::misc::keybinds::{KeyBind, sort_actions};
use mirl::misc::text_position::TextPosition;
use mirl::platform::keycodes::StringToKeyCodes;
use mirl::render::draw_text_antialiased_isize;
use mirl::{
    extensions::*,
    platform::{CursorStyle, keycodes::KeyCode},
};

use crate::{
    Buffer, DearMirlGuiModule, FocusTaken, InsertionMode, ModuleUpdateInfo,
    get_formatting, render,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// All the keybinds the text input module uses
pub enum Actions {
    /// Create a new line, when text is to the right that text will be pushed over to that new line
    NewLine,
    /// Create a new line, text to the right of the current line is ignored
    NewLineWithoutShifting,
    /// Configurable in struct, by default: Pad text to be a multiple of 4 at the caret position using spaces
    Indent,
    /// Configurable in struct, by default: Remove padding at the start of the line until the first character is at a position divisible by 4
    IndentAtLineStart,
    /// Configurable in struct, by default: Remove padding until the caret has a position divisible by 4
    Outdent,
    /// Configurable in struct, by default: Remove padding at the start of the line until the first character is at a position divisible by 4
    OutdentAtLineStart,
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
    /// Move the caret position up until the end of the determined structure
    MoveStructureUp,
    /// Move the caret position one line up and highlight everything between the old and new positions
    MoveUpAndHighlight,
    /// Move the caret position up until the end of the determined structure and highlight everything between the old and new positions
    MoveStructureUpAndHighlight,
    /// Move the caret position one line down
    MoveDown,
    /// Move the caret position down until the end of the determined structure
    MoveStructureDown,
    /// Move the caret position one line down and highlight everything between the old and new positions
    MoveDownAndHighlight,
    /// Move the caret position down until the end of the determined structure and highlight everything between the old and new positions
    MoveStructureDownAndHighlight,
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
    SwapWithLineAbove,
    /// Swap the current line with the line above
    SwapWithLineBelow,
    // /// Toggle showing the liner number on the right
    // ToggleLineNumbering,
    /// Start search
    ToggleSearchWindow,
    /// Search and replace
    ToggleReplaceWindow,
    // /// Go to the next occurrence of the current structure
    // NextOccurrence
    // /// Go to the previous occurrence of the current structure
    // PreviousOccurrence
    /// Instead of inserting letters, you're replacing them
    ToggleOverwrite,
    /// Move to the specified line number
    MoveToLine,
    /// Select all text
    SelectAll,
    /// Duplicate the current line below
    DuplicateLineBelow,
    /// Duplicate the current line above
    DuplicateToAbove,
    /// Moves caret position to the last column in the last line
    MoveToEndOfDocument,
    /// Moves the caret position to he last column in the current line
    MoveToEndOfLine,
    /// Moves the caret position to the very front of the first line
    MoveToStartOfDocument,
    /// Moves the caret position to the front of the current line
    MoveToStartOfLine,
    /// Select structure the caret is at
    SelectStructure,
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
            vec![KeyCode::Enter],
            Actions::NewLine,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::KeyPadEnter],
            Actions::NewLine,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::Enter],
            Actions::NewLineWithoutShifting,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::KeyPadEnter],
            Actions::NewLineWithoutShifting,
        ),
        KeyBind::new(false, false, false, vec![KeyCode::Tab], Actions::Indent),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::Tab],
            Actions::IndentAtLineStart,
        ),
        KeyBind::new(
            true,
            false,
            false,
            vec![KeyCode::Tab],
            Actions::OutdentAtLineStart,
        ),
        KeyBind::new(true, false, true, vec![KeyCode::Tab], Actions::Outdent),
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
            true,
            false,
            true,
            vec![KeyCode::UpArrow],
            Actions::MoveStructureUpAndHighlight,
        ),
        KeyBind::new(
            true,
            false,
            true,
            vec![KeyCode::DownArrow],
            Actions::MoveStructureDownAndHighlight,
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
            true,
            vec![KeyCode::UpArrow],
            Actions::MoveStructureUp,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::DownArrow],
            Actions::MoveStructureDown,
        ),
        KeyBind::new(
            false,
            true,
            false,
            vec![KeyCode::UpArrow],
            Actions::SwapWithLineAbove,
        ),
        KeyBind::new(
            false,
            true,
            false,
            vec![KeyCode::DownArrow],
            Actions::SwapWithLineBelow,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::F],
            Actions::ToggleSearchWindow,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::H],
            Actions::ToggleReplaceWindow,
        ),
        KeyBind::new(
            false,
            false,
            false,
            vec![KeyCode::Insert],
            Actions::ToggleOverwrite,
        ),
        KeyBind::new(false, false, true, vec![KeyCode::G], Actions::MoveToLine),
        KeyBind::new(false, false, true, vec![KeyCode::A], Actions::SelectAll),
        KeyBind::new(
            true,
            true,
            false,
            vec![KeyCode::DownArrow],
            Actions::DuplicateLineBelow,
        ),
        KeyBind::new(
            true,
            true,
            false,
            vec![KeyCode::UpArrow],
            Actions::DuplicateToAbove,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::End],
            Actions::MoveToEndOfDocument,
        ),
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::Home],
            Actions::MoveToStartOfDocument,
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
        KeyBind::new(
            false,
            false,
            true,
            vec![KeyCode::D],
            Actions::SelectStructure,
        ),
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
///
#[allow(clippy::struct_excessive_bools)]
pub struct TextInput {
    /// The width of the writeable section + line counter
    pub width: usize,
    /// The height of individual lines
    pub line_height: usize,
    /// How many lines are allowed
    pub max_lines: usize,
    /// The text the input contains
    pub text: Vec<String>,
    /// If the module needs to be redrawn
    pub needs_redraw: bool,
    /// If the button has been selected
    pub selected: usize,
    /// Used for detecting new key strokes
    pub last_keys_pressed: Vec<KeyCode>,
    /// The vertical, and horizontal position of the cursors. Yes for some reason this has multi cursor support
    pub caret: Vec<Caret>,
    // /// What text has been selected, the first item is the head, the last the tail
    // pub highlighted: (Position, Position),
    /// When at the front of a string, should pressing backspace allow you to delete what is behind? The objective answer is no, the subjective answer is 'Let it be configurable'.
    pub remove_behind: bool,
    /// Last text states allowing for ctrl+z/ctrl+y
    pub last_states: Vec<TextState>,
    /// The current state so the functions don't get confused what state is currently used
    pub current_state: usize,
    /// What text should be displayed when no text is written
    pub placeholder_text: String,
    /// If you can select the input field
    pub read_only: bool,
    /// How big an "indent" is
    pub indent_length: usize,
    /// What an indent is
    pub indent_char: char,
    /// Keybinds
    pub keybinds: Vec<KeyBind<Actions>>,
    /// Lets the caret wrap around from the start to the end/end to the start
    pub allow_caret_wrap: bool,
    /// How much space is reserved for the line number
    pub line_number_offset: usize,
    /// The camera
    pub camera: mirl::misc::ScrollableCamera,
    /// When in overwrite mode, instead of inserting characters, characters that already exist will be overwritten
    pub overwrite_mode: bool,
    /// If the whitespace at the end of a previous line should be conserved
    pub retain_indent: bool,
    /// What menu is currently open
    pub menu_open: TextInputMenu,
    /// If the line number should be shown
    pub show_line_numbers: bool,
    /// The text by default is 20% smaller
    pub text_height: f32,
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
pub struct Caret {
    /// Vertical
    pub line: usize,
    /// Horizontal
    pub column: usize,
    /// The origin of the highlight
    pub highlight_pos: TextPosition,
    /// If highlight is active
    pub highlight_enabled: bool,
    /// When moving vertically into a line with less characters, the "current" column position is forgotten. This variable is used to remember the last horizontal progress
    pub retain_column: Option<TextPosition>,
    /// Used for retaining the last largest horizontal positioning
    pub last_pos: TextPosition,
}
impl Caret {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self {
            line: (line),
            column: (column),
            highlight_pos: (TextPosition::new(0, 0)),
            highlight_enabled: (false),
            retain_column: (None),
            last_pos: (TextPosition::new(0, 0)),
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
        TextPosition::new(self.line(), self.column())
    }
    /// Set the current position to the position of the given position
    pub const fn move_to_pos(&mut self, position: TextPosition) {
        self.line = position.line;
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
    /// Get the current line
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }
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
impl TextInput {
    /// Move to the last character of the last line
    pub fn move_to_end_of_document(&mut self, idx: usize) {
        self.move_to_end_of_line(idx, self.line_count_idx());
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Sets the caret position to the end of the specified line
    pub fn move_to_end_of_line(&mut self, idx: usize, line: usize) {
        self.caret[idx].line = line;
        self.caret[idx].column = self.get_line_length(line);
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Sets the caret position to the end of the current line
    pub fn move_to_end_of_this_line(&mut self, idx: usize) {
        self.caret[idx].column = self.get_line_length(self.caret[idx].line);
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Sets the caret position to the start of the specified line
    pub fn move_to_start_of_line(&mut self, idx: usize, line: usize) {
        self.caret[idx].line = line;
        self.caret[idx].column = 0;
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Sets the caret position to the start of the current line
    pub fn move_to_start_of_this_line(&mut self, idx: usize) {
        self.caret[idx].column = 0;
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Move up a structure up
    pub fn move_up_by_structure(&mut self, idx: usize) {
        let strip = self.get_line_vertically(self.caret[idx].column());
        // println!("{}", strip.reversed());
        // println!("{}", "^".rjust(strip.chars().count() - self.line(), None));
        let skip = mirl::misc::skipping_text_type::skip_char_type(
            &strip.reversed(),
            strip.chars().count() - self.caret[idx].line(),
        );
        //println!("{skip}");
        self.caret[idx].line += skip;
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Move up a structure up
    pub fn move_down_by_structure(&mut self, idx: usize) {
        let strip = self.get_line_vertically(self.caret[idx].column());
        // println!("{}", strip);
        // println!("{}", "^".rjust(self.line() + 1, None));
        let skip = mirl::misc::skipping_text_type::skip_char_type(
            &strip,
            self.caret[idx].line(),
        );
        //println!("{skip}");
        self.caret[idx].line =
            self.clamp_to_line_count(skip + self.caret[idx].line());
    }
    /// Move the caret postion the the end of the next detected structure
    pub fn move_left_by_structure(&mut self, idx: usize) {
        if self.caret[idx].column == 0 {
            self.move_left(idx);
        }
        self.caret[idx].column =
            mirl::misc::skipping_text_type::skip_char_type_backward(
                &self.text[self.caret[idx].line],
                self.caret[idx].column,
            );
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Move the caret postion the the end of the next detected structure
    pub fn move_right_by_structure(&mut self, idx: usize) {
        if self.caret[idx].column == self.get_line_length(self.caret[idx].line)
        {
            self.move_right(idx);
        }
        self.caret[idx].column = mirl::misc::skipping_text_type::skip_char_type(
            &self.text[self.caret[idx].line],
            self.caret[idx].column,
        );
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Move the caret up if it is under a line
    pub fn move_caret_under_line_up(&mut self, line: usize) {
        if self.caret[0].line() >= line && self.caret[0].line() != 0 {
            self.move_up(0);
        }
    }

    /// Move cursor up one space safely
    pub fn move_up(&mut self, idx: usize) {
        let pos = self.caret[idx].to_position();
        let previous = self.caret[idx].line;
        self.caret[idx].line = self.caret[idx].line.saturating_sub(1);

        if previous == self.caret[idx].line {
            // We are and were at the top so just go to the start of the line
            self.move_to_start_of_this_line(idx);
        } else if !self.if_not_moved_restore_column(pos, idx) {
            self.caret[idx].retain_column = Some(self.caret[idx].to_position());
            self.caret[idx].column = self
                .clamp_to_column(self.caret[idx].line, self.caret[idx].column);
            self.caret[idx].last_pos = self.caret[idx].to_position();
        }
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
    }
    /// Move the caret one space to the right
    pub fn move_right(&mut self, idx: usize) {
        if self.caret[idx].is_highlighting() {
            let pos1 = self.caret[idx].highlight_pos;
            let pos2 = self.caret[idx].to_position();
            if pos1 < pos2 {
                self.caret[idx].move_to_pos(pos2);
            } else {
                self.caret[idx].move_to_pos(pos1);
            }
        } else if self.caret[idx].column
            == self.get_line_length(self.caret[idx].line)
        {
            if self.caret[idx].line == self.line_count_idx() {
                if self.allow_caret_wrap {
                    self.caret[idx].column = 0;
                    self.caret[idx].line = 0;
                }
            } else {
                self.caret[idx].column = 0;
                self.caret[idx].line =
                    self.clamp_to_line_count(self.caret[idx].line + 1);
            }
        } else {
            self.caret[idx].column += 1;
        }
        self.move_camera_to_move_caret_into_view(0, &get_formatting());
    }
    /// Move the caret down a line
    pub fn move_down(&mut self, idx: usize) {
        let pos = self.caret[idx].to_position();
        let previous = self.caret[idx].line;
        self.caret[idx].line =
            self.clamp_to_line_count(self.caret[idx].line + 1);

        if previous == self.caret[idx].line {
            self.move_to_end_of_this_line(idx);
        } else if !self.if_not_moved_restore_column(pos, idx) {
            self.caret[idx].retain_column = Some(self.caret[idx].to_position());
            self.caret[idx].column = self
                .clamp_to_column(self.caret[idx].line, self.caret[idx].column);
            self.caret[idx].last_pos = self.caret[idx].to_position();
        }
        self.move_camera_to_move_caret_into_view(0, &get_formatting());
    }
    /// Move the caret down if it is under a line
    pub fn move_caret_under_line_down(&mut self, line: usize) {
        if self.caret[0].line() > line && self.caret[0].line() != 0 {
            self.move_down(0);
        }
    }
    /// Move the caret one space to the left
    pub fn move_left(&mut self, idx: usize) {
        if self.caret[idx].is_highlighting() {
            let pos1 = self.caret[idx].highlight_pos;
            let pos2 = self.caret[idx].to_position();
            if pos1 > pos2 {
                self.caret[idx].move_to_pos(pos2);
            } else {
                self.caret[idx].move_to_pos(pos1);
            }
        } else if self.caret[idx].column == 0 {
            if self.caret[idx].line == 0 {
                if self.allow_caret_wrap {
                    self.caret[idx].line = self.line_count_idx();
                    self.caret[idx].column =
                        self.get_line_length(self.caret[idx].line);
                }
            } else {
                self.caret[idx].line -= 1;
                self.caret[idx].column =
                    self.get_line_length(self.caret[idx].line);
            }
        } else {
            self.caret[idx].column = self.caret[idx].column.saturating_sub(1);
        }
        self.move_camera_to_move_caret_into_view(0, &get_formatting());
    }
}
/// Selection
impl TextInput {
    /// Select all text available
    pub fn select_all(&mut self, idx: usize) {
        self.move_to_start_of_line(idx, 0);
        self.caret[idx].set_highlight_origin_to_current_pos();
        self.caret[idx].highlight_enabled = true;
        self.move_to_end_of_document(idx);
    }
    /// Select the structure around the caret
    pub fn select_structure(&mut self, idx: usize) {
        let left_char_type = mirl::misc::skipping_text_type::get_char_type(
            self.get_character(
                self.caret[idx].line(),
                self.caret[idx].column().saturating_sub(1),
            )
            .unwrap_or_default(),
        );
        let right_char_type = mirl::misc::skipping_text_type::get_char_type(
            self.get_character(
                self.caret[idx].line(),
                self.caret[idx].column(),
            )
            .unwrap_or_default(),
        );
        let left = if self.caret[idx].column() == 1
            || (self.caret[idx].column() != 0
                && left_char_type
                    == mirl::misc::skipping_text_type::get_char_type(
                        self.get_character(
                            self.caret[idx].line(),
                            self.caret[idx].column() - 2,
                        )
                        .unwrap_or_default(),
                    ))
        {
            mirl::misc::skipping_text_type::skip_char_type_backward(
                &self.text[self.caret[idx].line()],
                self.caret[idx].column(),
            )
        } else if right_char_type == left_char_type {
            self.caret[idx].column().saturating_sub(1)
        } else {
            self.caret[idx].column()
        };
        let right = if right_char_type
            == mirl::misc::skipping_text_type::get_char_type(
                self.get_character(
                    self.caret[idx].line(),
                    self.caret[idx].column() + 1,
                )
                .unwrap_or_default(),
            ) {
            mirl::misc::skipping_text_type::skip_char_type(
                &self.text[self.caret[idx].line()],
                self.caret[idx].column(),
            )
        } else if right_char_type == left_char_type {
            self.caret[idx].column() + 1
        } else {
            self.caret[idx].column()
        };
        self.caret[idx].column = left;
        self.caret[idx].enable_highlight();
        self.caret[idx].column = right;
    }
}
/// In/Out denting
impl TextInput {
    /// Indent at the current care position
    pub fn indent(&mut self, idx: usize) {
        let text_left: String = self.text[self.caret[idx].line()]
            .chars()
            .take(self.caret[idx].column())
            .collect();

        let text_right: String = self.text[self.caret[idx].line()]
            .chars()
            .skip(self.caret[idx].column())
            .collect();

        let length = text_left.chars().count() % self.indent_length;
        let repeats = self.indent_length - length;
        let insertion: String =
            self.indent_char.repeat_value(repeats).iter().collect();

        let text = text_left + &insertion + &text_right;
        self.caret[idx].column += repeats;
        self.text[self.caret[idx].line()] = text;
    }
    /// Indent at the first non white character at he start of the line
    pub fn indent_at_line_start(&mut self, idx: usize) {
        // Shut it clippy
        let offset = mirl::misc::skipping_text_type::skip_char_type(
            &self.text[self.caret[idx].line()],
            0,
        )
        .saturating_add(2);

        let length = offset % self.indent_length;
        let repeats = self.indent_length - length;
        let insertion: String =
            self.indent_char.repeat_value(repeats).iter().collect();

        let text = insertion + &self.text[self.caret[idx].line()];
        self.caret[idx].column += repeats;
        self.text[self.caret[idx].line()] = text;
    }
    /// Outdent at the current cursor position
    pub fn outdent(&mut self, idx: usize) {
        let text_left: String = self.text[self.caret[idx].line()]
            .chars()
            .take(self.caret[idx].column())
            .collect();

        if mirl::misc::skipping_text_type::get_char_type(
            text_left.chars().last().unwrap_or_default(),
        ) != mirl::misc::skipping_text_type::CharacterType::Whitespace
        {
            return;
        }

        let text_right: String = self.text[self.caret[idx].line()]
            .chars()
            .skip(self.caret[idx].column())
            .collect();
        let val = self.caret[idx].column()
            - mirl::misc::skipping_text_type::skip_char_type_backward(
                &text_left,
                self.caret[idx].column(),
            )
            .saturating_sub(1);

        let length = val % self.indent_length;
        let length = if length == 0 {
            self.indent_length
        } else {
            length
        };
        // println!("{length}");
        //let end = text_left.chars().count() - length;
        //text_left.truncate(end);

        let text_before: String =
            text_left.chars().take(self.caret[idx].column() - length).collect();

        let text = text_before + &text_right;
        self.caret[idx].column -= length;
        self.text[self.caret[idx].line()] = text;
    }
    /// Indent the start of the current line
    pub fn indent_start_of_line(&mut self, idx: usize) {
        let text_left: String = self.text[self.caret[idx].line()]
            .chars()
            .take(self.caret[idx].column())
            .collect();

        let text_right: String = self.text[self.caret[idx].line()]
            .chars()
            .skip(self.caret[idx].column())
            .collect();

        let length = mirl::misc::skipping_text_type::skip_char_type(
                        &text_left, 0,
                    ).saturating_add(2)  // Why + 2? Why. I don't get it.
                        % self.indent_length;
        let length = if length == 0 {
            self.indent_length
        } else {
            length
        };
        // println!("{length}");
        //let end = text_left.chars().count() - length;
        //text_left.truncate(end);

        let text_before: String = text_left.chars().take(length).collect();

        let text_after: String = text_left.chars().skip(length).collect();

        if !text_before.chars().any(|x| x != self.indent_char) {
            let text = text_after + &text_right;
            self.caret[idx].column -= length;
            self.text[self.caret[idx].line()] = text;
        }
    }
}

/// Text unaltering
impl TextInput {
    #[allow(clippy::missing_panics_doc)]
    /// Calculate the offset the line number will take up
    pub fn recalculate_line_number_offset(
        &mut self,
        formatting: &crate::Formatting,
    ) {
        #[allow(clippy::unwrap_used)] // Shut it clippy, this cannot error
        let line_number_offset = "1234567890"
            .chars()
            .map(|x| {
                mirl::render::get_text_width(
                    &(x).repeat_value(
                        ((self.text.len() as f64).log10().floor() + 1.0)
                            as usize,
                    )
                    .iter()
                    .collect::<String>(),
                    self.text_height,
                    &formatting.font,
                )
            })
            .reduce(f32::max)
            .unwrap() as usize;
        self.line_number_offset = line_number_offset;
    }
    #[inline(always)]
    #[must_use]
    #[allow(clippy::inline_always)]
    /// Get the length of a line
    pub fn get_line_length(&self, line: usize) -> usize {
        self.text[line].chars().count()
    }
    #[inline(always)]
    #[must_use]
    #[allow(clippy::inline_always)]
    /// Clamp a value to the line count
    pub fn clamp_to_line_count(&self, other: usize) -> usize {
        self.line_count_idx().min(other)
    }
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    #[allow(clippy::unwrap_used)]
    /// Get a full column
    pub fn get_line_vertically(&self, column: usize) -> String {
        let mut value = String::new();
        for i in 0..self.text.len() {
            value.push(
                self.get_character(i, column)
                    .unwrap_or_else(|| " ".chars().nth(0).unwrap()),
            );
        }
        value
    }
    #[inline(always)]
    #[allow(clippy::inline_always)]
    #[must_use]
    /// Get the line count
    pub const fn line_count_idx(&self) -> usize {
        self.text.len().saturating_sub(1)
    }
    #[inline(always)]
    #[must_use]
    #[allow(clippy::inline_always)]
    /// Clamp a value to the column of the specified line
    pub fn clamp_to_column(&self, line: usize, column: usize) -> usize {
        self.get_line_length(self.clamp_to_line_count(line)).min(column)
    }
    #[must_use]
    /// Get a single character from line, column
    pub fn get_character(&self, line: usize, column: usize) -> Option<char> {
        self.text[line].chars().nth(column)
    }
    #[must_use]
    /// Get the horizontal offset the text is experiencing
    pub const fn get_horizontal_text_offset(
        &self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        if self.show_line_numbers {
            ((formatting.horizontal_margin * 3) as f32 + self.camera.offset_x)
                as crate::DearMirlGuiCoordinateType
                + self.line_number_offset as crate::DearMirlGuiCoordinateType
        } else {
            formatting.horizontal_margin as crate::DearMirlGuiCoordinateType
                + self.camera.offset_x as crate::DearMirlGuiCoordinateType
        }
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
            let line_chars: Vec<char> = self.text[line_start].chars().collect();
            let selected_chars: Vec<char> =
                line_chars[front_pos..back_pos].to_vec();
            let selected_text: String = selected_chars.into_iter().collect();
            vec![selected_text]
        } else {
            let mut result = Vec::new();

            let first_line_chars: Vec<char> =
                self.text[line_start].chars().collect();
            let first_line_selected: String =
                first_line_chars[front_pos..].iter().collect();
            result.push(first_line_selected);

            for line_idx in (line_start + 1)..line_end {
                result.push(self.text[line_idx].clone());
            }

            let last_line_chars: Vec<char> =
                self.text[line_end].chars().collect();
            let last_line_selected: String =
                last_line_chars[..back_pos].iter().collect();
            result.push(last_line_selected);

            result
        }
    }
    /// Move the camera to make sure a caret is visible
    pub fn move_camera_to_move_caret_into_view(
        &mut self,
        idx: usize,
        formatting: &crate::Formatting,
    ) {
        self.view_position(self.caret[idx].to_position(), formatting);
    }
    /// Move the camera to view a position
    ///
    /// This function has to be rewritten!!!!!!!!!
    pub fn view_position(
        &mut self,
        pos: TextPosition,
        formatting: &crate::Formatting,
    ) {
        let target_y = (pos.line + 1) * self.line_height;
        let target_x = render::get_text_width(
            &self.text[pos.line].chars().take(pos.column).collect::<String>(),
            self.line_height as f32,
            &formatting.font,
        );

        let margin = self.line_height as f32;
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
        } else if target_y as f32 + self.line_height as f32
            > visible_bottom - margin
        {
            self.camera.offset_y =
                -(target_y as f32 + self.line_height as f32 + margin
                    - viewport_height);
        }
        self.camera.clamp_to_bounds();
    }

    /// What is the offset required to correctly render line X
    #[must_use]
    pub fn get_line_height(
        &self,
        formatting: &crate::Formatting,
        idx: usize,
    ) -> isize {
        ((self.line_height + formatting.vertical_margin)
            * self.caret[idx].line()) as isize
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
    /// Get the white space at the start of the line
    #[must_use]
    pub fn get_line_indent(&self, line: usize) -> usize {
        let mut text = self.text[line].clone();
        let mut amount: usize = 0;
        while text.chars().nth(0).unwrap_or_default() == self.indent_char {
            amount += 1;
            text.remove(0);
        }
        amount
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
        let mut longest = String::new();
        let mut length = 0;
        for i in &self.text {
            let l = i.chars().count();
            if l > length {
                length = l;
                longest.clone_from(i);
            }
        }
        longest
    }
    fn get_content_size(&self, font: &fontdue::Font) -> (f32, f32) {
        let width = render::get_text_width(
            &self.get_longest_line(),
            self.text_height,
            font,
        ) * 1.5; // Theoretically not "good", practically it works better than intended and at the end of the day, that's the only thing that counts:)
        let height = self.line_height * (self.text.len() + 1);
        (width, height as f32)
    }
    #[inline(always)]
    #[allow(clippy::inline_always)]
    #[must_use]
    /// Splits the line at the given position and returns both parts
    pub fn split_line(&self, line: usize, idx: usize) -> (String, String) {
        let before = self.text[line].chars().take(idx).collect();
        let after = self.text[line].chars().skip(idx).collect();
        (before, after)
    }
    /// If a previous column position has been set, respect it
    pub fn if_not_moved_restore_column(
        &mut self,
        last_pos: TextPosition,
        idx: usize,
    ) -> bool {
        let do_not_retain_column = false;
        if do_not_retain_column {
            return false;
        }
        if let Some(highest_pos) = self.caret[idx].retain_column
            && self.caret[idx].last_pos == last_pos
        {
            self.caret[idx].column = self
                .clamp_to_column(self.caret[idx].line(), highest_pos.column);
            self.caret[idx].last_pos = self.caret[idx].to_position();
            return true;
        }
        false
    }
    /// Set or hide the current window
    pub fn toggle_menu(&mut self, menu: TextInputMenu) {
        if self.menu_open == menu {
            self.menu_open = TextInputMenu::None;
        } else {
            self.menu_open = menu;
        }
    }
}

/// Potentially line adding
impl TextInput {
    /// Insert a text line and move every cursor under one down
    pub fn insert_line(
        &mut self,
        line_idx: usize,
        line_content: String,
    ) -> bool {
        if self.text.len() >= self.max_lines {
            return false;
        }
        self.text.insert(line_idx, line_content);
        self.move_caret_under_line_down(line_idx);
        true
    }
    /// Create a new line with the remainder of this line getting placed on the new one
    pub fn new_line(&mut self, idx: usize) -> bool {
        if self.text.len() >= self.max_lines {
            return false;
        }
        if self.caret[idx].is_highlighting() {
            self.delete_text_in_area(self.caret[idx].get_highlighted_area());
        }
        let (before, after) =
            self.split_line(self.caret[idx].line(), self.caret[idx].column());

        self.text[self.caret[idx].line()] = before;
        let amount = if self.retain_indent {
            self.get_line_indent(self.caret[idx].line())
        } else {
            0
        };
        let before: String =
            self.indent_char.repeat_value(amount).iter().collect();
        self.caret[idx].line += 1;
        self.caret[idx].column = amount;
        self.insert_line(self.caret[idx].line(), before + &after);

        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
        true
    }
    /// Like the new line function but doesn't copy the remaining text to the next line
    pub fn new_line_without_shifting(&mut self, idx: usize) -> bool {
        if self.text.len() >= self.max_lines {
            return false;
        }
        self.caret[idx].reset_highlighted();
        let amount = if self.retain_indent {
            self.get_line_indent(self.caret[idx].line())
        } else {
            0
        };
        let before: String =
            self.indent_char.repeat_value(amount).iter().collect();
        self.caret[idx].line += 1;
        self.caret[idx].column = amount;
        self.insert_line(self.caret[idx].line(), before);
        self.move_camera_to_move_caret_into_view(idx, &get_formatting());
        true
    }
}

#[must_use]
/// Convert normal text into text TextInput can use -> Replaces all tabs and splits at new lines
pub fn ready_text(text: &str, spaces_per_tab: usize) -> Vec<String> {
    let t = text.replace(
        '\t',
        &' '.repeat_value(spaces_per_tab).iter().collect::<String>(),
    );
    let x: Vec<&str> = t.split('\n').collect();
    x.iter().map(std::string::ToString::to_string).collect()
}

/// A List of text, the caret position, and what is highlighted
pub type TextState = (Vec<String>, Vec<Caret>);

impl TextInput {
    #[must_use]
    /// Set the placeholder text displayed when nothing is typed
    pub fn with_placeholder_text(mut self, text: &str) -> Self {
        self.placeholder_text = text.to_string();
        self
    }
}

impl TextInput {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        line_height: usize,
        width: usize,
        lines: usize,
        text: Option<Vec<String>>,
    ) -> Self {
        Self {
            width,
            line_height,
            max_lines: lines,
            text: text.clone().unwrap_or_else(|| Vec::from([String::new()])),
            needs_redraw: (true),
            selected: 0,
            last_keys_pressed: Vec::new(),
            // Yummy
            caret: vec![Caret::new(
                text.clone().unwrap_or_default().len().saturating_sub(1),
                text.unwrap_or_default()
                    .last()
                    .map_or_default(|x| x.chars().count()),
            )],
            //highlighted: std::default::Default::default(),
            remove_behind: false,
            last_states: Vec::new(),
            current_state: 0,
            read_only: false,
            placeholder_text: String::new(),
            indent_length: 4,
            indent_char: " ".chars().next().unwrap_or_default(),
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
            line_number_offset: width / 10,
            overwrite_mode: false,
            retain_indent: true,
            menu_open: TextInputMenu::None,
            show_line_numbers: true,
            text_height: line_height as f32 * 0.8,
            blacklist: Vec::new(),
            blacklist_is_whitelist: false,
        }
    }

    // /// If any selection has been made
    // pub const fn is_something_highlighted(&self) -> bool {
    //     self.highlighted.0.line != self.highlighted.1.line
    //         || self.highlighted.0.column != self.highlighted.1.column
    // }
    /// Delete all text in all area
    pub fn delete_text_in_area(&mut self, pos: (TextPosition, TextPosition)) {
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
            return;
        }

        let total_lines = line_end - line_start;

        if total_lines == 0 {
            let mut line_chars: Vec<char> =
                self.text[line_start].chars().collect();
            line_chars.drain(front_pos..back_pos);
            self.text[line_start] = line_chars.into_iter().collect();
        } else {
            let first_line_before: String =
                self.text[line_start].chars().take(front_pos).collect();
            let last_line_after: String =
                self.text[line_end].chars().skip(back_pos).collect();

            self.text[line_start] = first_line_before + &last_line_after;

            self.text.drain((line_start + 1)..=line_end);
        }
        self.caret[0].line = line_start;
        self.caret[0].column = front_pos;

        self.caret[0].reset_highlighted();
    }
    /// Delete the given lines
    pub fn delete_lines(&mut self, pos: (TextPosition, TextPosition)) {
        let (start_line, end_line) = {
            let head = pos.0;
            let tail = pos.1;
            if head.line > tail.line {
                (tail.line, head.line)
            } else {
                (head.line, tail.line)
            }
        };

        self.text.drain(start_line..=end_line);

        self.caret[0].line = start_line.min(self.text.len().saturating_sub(1));
        self.caret[0].column = 0;
        self.caret[0].reset_highlighted();
    }
    /// Delete a single character to the right
    pub fn delete_right(&mut self, idx: usize) {
        if self.caret[idx].is_highlighting() {
            self.delete_text_in_area(self.caret[idx].get_highlighted_area());
        } else {
            if self.caret[idx].is_highlighting() {
                self.delete_text_in_area(
                    self.caret[idx].get_highlighted_area(),
                );
                return;
            }
            if self.caret[idx].column()
                == self.get_line_length(self.caret[idx].line())
            {
                self.merge_lines(
                    self.caret[idx].line(),
                    self.clamp_to_line_count(self.caret[idx].line() + 1),
                );
            } else {
                self.remove_chars_from_line(
                    self.caret[idx].line(),
                    self.caret[idx].column(),
                    1,
                );
            }
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
    pub fn insert_string_into_line(
        &mut self,
        line: usize,
        idx: usize,
        string: &str,
    ) {
        let (before, after) = self.split_line(line, idx);
        self.text[line] = before + string + &after;
    }
    #[inline(always)]
    #[allow(clippy::inline_always)]
    /// Insert a string into the middle of a line
    pub fn remove_chars_from_line(
        &mut self,
        line: usize,
        idx: usize,
        amount: usize,
    ) {
        self.text[line].remove_chars_at(idx, amount);
    }
    /// Joins together this and the other line -> the other line will be consumed
    pub fn merge_lines(&mut self, line: usize, other: usize) {
        //println!("{} {}", line, other);
        if line == other {
            return;
        }
        let text = self.text[other].clone();

        self.text[line].push_str(&text);
        self.text.remove(other);
        self.move_caret_under_line_up(other);
    }
    /// Delete the whole line the caret is on
    pub fn delete_current_line(&mut self, idx: usize) {
        if self.caret[idx].is_highlighting() {
            self.delete_lines(self.caret[idx].get_highlighted_area());
            let (pos1, pos2) = self.caret[idx].get_highlighted_area();
            self.caret[idx].move_to_pos(pos1.min(pos2));
        } else {
            self.remove_line(self.caret[idx].line());
            self.move_to_start_of_this_line(idx);
        }
    }
    /// Delete the structure detected to the left
    pub fn delete_structure_left(&mut self, idx: usize) {
        let cut_point = mirl::misc::skipping_text_type::skip_char_type_backward(
            &self.text[self.caret[idx].line()],
            self.caret[idx].column(),
        );
        self.remove_chars_from_line(
            self.caret[idx].line(),
            cut_point,
            self.caret[idx].column() - cut_point,
        );
        self.caret[idx].column = cut_point;
    }
    /// Delete the structure detected to the right
    pub fn delete_structure_right(&mut self, idx: usize) {
        let cut_point = mirl::misc::skipping_text_type::skip_char_type(
            &self.text[self.caret[idx].line()],
            self.caret[idx].column(),
        );
        self.remove_chars_from_line(
            self.caret[idx].line(),
            self.caret[idx].column(),
            cut_point - self.caret[idx].column(),
        );
    }

    /// Delete a single character to the left
    pub fn delete_left(&mut self, idx: usize) {
        if self.caret[idx].is_highlighting() {
            self.delete_text_in_area(self.caret[idx].get_highlighted_area());
        } else if self.caret[idx].column() == 0 {
            if self.caret[idx].line() != 0 {
                self.caret[idx].column = self
                    .get_line_length(self.caret[idx].line.saturating_sub(1));
            }
            self.merge_lines(
                self.caret[idx].line.saturating_sub(1),
                self.caret[idx].line(),
            );
            //self.move_up(idx);
        } else {
            self.move_left(idx);
            self.remove_chars_from_line(
                self.caret[idx].line(),
                self.caret[idx].column,
                1,
            );
        }
    }
    /// Deletes a line and moves carets below
    pub fn remove_line(&mut self, line: usize) {
        self.text.remove(line);
        self.move_caret_under_line_up(line);
    }
    /// Swap the postion of the cursor with another line
    pub fn swap_caret_position(&mut self, line1: usize, line2: usize) {
        if self.caret[0].line() == line1 {
            self.caret[0].line = line2;
        } else if self.caret[0].line() == line2 {
            self.caret[0].line = line1;
        }
    }
    /// Undo the last action
    pub fn undo(&mut self) {
        // println!("{}", self.current_state);
        if !self.last_states.is_empty() {
            //println!("Set");
            self.current_state -= 1;
            (self.text, self.caret) =
                self.last_states[self.current_state].clone();
        }
    }

    /// Writes out the keycodes at the caret position
    pub fn write(&mut self, keycodes: &[KeyCode], uppercase: bool, idx: usize) {
        for keycode in keycodes {
            if self.blacklist.contains(keycode) != self.blacklist_is_whitelist {
                continue;
            }
            if let Some(value) = keycode.to_user_friendly_string() {
                let before: String = self.text[self.caret[idx].line()]
                    .chars()
                    .take(self.caret[idx].column())
                    .collect();
                let after: String = self.text[self.caret[idx].line()]
                    .chars()
                    .skip(self.caret[idx].column())
                    .collect();
                if uppercase {
                    self.text[self.caret[idx].line()] =
                        before + &value.to_uppercase() + &after;
                } else {
                    self.text[self.caret[idx].line()] =
                        before + &value.to_lowercase() + &after;
                }
                self.caret[idx].column += 1;
            }
        }
    }
    /// Redo the last undo
    pub fn redo(&mut self) {
        if self.current_state + 1 < self.last_states.len() {
            self.current_state += 1;
            (self.text, self.caret) =
                self.last_states[self.current_state].clone();
        }
    }
    /// Swap the position if 2 lines
    #[allow(clippy::assigning_clones)]
    pub fn swap_lines(&mut self, line1: usize, line2: usize) {
        self.text.swap(line1, line2);
        self.swap_caret_position(line1, line2);
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
                    self.move_right(0);
                    self.caret[0].reset_highlighted();
                }
                Actions::MoveLeft => {
                    self.move_left(0);
                    self.caret[0].reset_highlighted();
                }
                Actions::MoveUp => {
                    self.move_up(0);
                    self.caret[0].reset_highlighted();
                }
                Actions::MoveDown => {
                    self.move_down(0);
                    self.caret[0].reset_highlighted();
                }
                // Simple deletion
                Actions::DeleteLeft => {
                    //let caret = self.caret[0].clone();
                    self.delete_left(0);
                    // caret.delete_left(self);
                    // self.caret[0] = caret;
                }
                Actions::DeleteRight => {
                    self.delete_right(0);
                    // let caret = self.caret[0].clone();
                    // caret.delete_right(self);
                    // self.caret[0] = caret;
                }
                Actions::Copy => {
                    if self.caret[0].is_highlighting() {
                        new_clipboard_data = Some(mirl::platform::file_system::FileData::from_list_of_strings(&self.get_selected_area(self.caret[0].get_highlighted_area())));
                    } else {
                        new_clipboard_data = Some(
                            mirl::platform::file_system::FileData::from_string(
                                self.text[self.caret[0].line()].clone(),
                            ),
                        );
                    }
                }
                // Clipboard stuff
                Actions::Cut => {
                    if self.caret[0].is_highlighting() {
                        new_clipboard_data =Some(mirl::platform::file_system::FileData::from_list_of_strings(&self.get_selected_area(self.caret[0].get_highlighted_area())));
                        self.delete_text_in_area(
                            self.caret[0].get_highlighted_area(),
                        );
                    } else {
                        new_clipboard_data = Some(
                            mirl::platform::file_system::FileData::from_string(
                                self.text[self.caret[0].line()].clone(),
                            ),
                        );
                        self.remove_line(self.caret[0].line());
                    }
                }
                Actions::RequestPaste => {
                    request_clipboard_data = true;
                }
                // Misc
                Actions::NewLine => {
                    self.new_line(0);
                }
                Actions::NewLineWithoutShifting => {
                    self.new_line_without_shifting(0);
                }
                Actions::DeleteCurrentLine => {
                    self.delete_current_line(0);
                }
                Actions::DeleteStructureLeft => {
                    self.delete_structure_left(0);
                }
                Actions::DeleteStructureRight => {
                    self.delete_structure_right(0);
                }
                Actions::DuplicateToAbove => {
                    self.insert_line(
                        self.caret[0].line(),
                        self.text[self.caret[0].line()].clone(),
                    );
                }
                Actions::DuplicateLineBelow => {
                    self.insert_line(
                        self.caret[0].line(),
                        self.text[self.caret[0].line()].clone(),
                    );
                    self.move_caret_under_line_down(
                        self.caret[0].line().saturating_sub(1),
                    );
                }
                Actions::SwapWithLineAbove => {
                    if self.caret[0].line() > 0 {
                        self.swap_lines(
                            self.caret[0].line(),
                            self.caret[0].line() - 1,
                        );
                    }
                }
                Actions::SwapWithLineBelow => {
                    if self.caret[0].line() < self.line_count_idx() {
                        self.swap_lines(
                            self.caret[0].line(),
                            self.caret[0].line() + 1,
                        );
                    }
                }
                Actions::MoveStructureLeft => {
                    self.move_left_by_structure(0);
                    self.caret[0].reset_highlighted();
                }
                Actions::MoveStructureRight => {
                    self.move_right_by_structure(0);
                    self.caret[0].reset_highlighted();
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
                Actions::Indent => {
                    self.indent(0);
                    // let caret = self.caret[0].clone();
                    // caret.indent(self);
                    // self.caret[0] = caret;
                }
                Actions::IndentAtLineStart => {
                    self.indent_at_line_start(0);
                    // let caret = self.caret[0].clone();
                    // caret.indent_at_line_start(self);
                    // self.caret[0] = caret;
                }
                Actions::Outdent => {
                    self.outdent(0);
                    // let caret = self.caret[0].clone();
                    // caret.outdent(self);
                    // self.caret[0] = caret;
                }
                Actions::OutdentAtLineStart => {
                    self.indent_start_of_line(0);
                    // let caret = self.caret[0].clone();
                    // caret.indent_start_of_line(self);
                    // self.caret[0] = caret;
                }
                Actions::SelectAll => {
                    self.select_all(0);
                }
                Actions::MoveToEndOfDocument => {
                    self.move_to_end_of_document(0);
                }
                Actions::MoveToEndOfLine => {
                    self.move_to_end_of_this_line(0);
                }
                Actions::MoveToStartOfLine => {
                    self.move_to_start_of_this_line(0);
                }
                Actions::MoveToStartOfDocument => {
                    self.move_to_start_of_line(0, 0);
                }
                Actions::SelectLine => {
                    self.move_to_start_of_this_line(0);
                    self.caret[0].set_highlight_origin_to_current_pos();
                    self.move_to_end_of_this_line(0);
                }
                Actions::MoveDownAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_down(0);
                }
                Actions::MoveLeftAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_left(0);
                }
                Actions::MoveRightAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_right(0);
                }
                Actions::MoveUpAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_up(0);
                }
                Actions::MoveStructureLeftAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_left_by_structure(0);
                }
                Actions::MoveStructureRightAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_right_by_structure(0);
                }
                Actions::SelectStructure => {
                    self.select_structure(0);
                }
                Actions::ToggleSearchWindow => {
                    self.toggle_menu(TextInputMenu::Search);
                }
                Actions::ToggleReplaceWindow => {
                    self.toggle_menu(TextInputMenu::Replace);
                }
                Actions::MoveToLine => {
                    self.toggle_menu(TextInputMenu::SkipToLine);
                }
                Actions::MoveStructureUp => {
                    self.move_up_by_structure(0);
                }
                Actions::MoveStructureDown => {
                    self.move_down_by_structure(0);
                }
                Actions::MoveStructureDownAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_down_by_structure(0);
                }
                Actions::MoveStructureUpAndHighlight => {
                    self.caret[0].enable_highlight();
                    self.move_up_by_structure(0);
                } // _ => todo!("Keybind missing"),
            }
        }

        if self.text.is_empty() {
            self.text.push(String::new());
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
                self.write(&text_data.to_keycodes(), shift_down, 0);
            } else if let Some(list_string) =
                clipboard_data.to_list_of_strings()
            {
                changed = true;
                for i in list_string {
                    self.write(&i.to_keycodes(), shift_down, 0);
                    self.new_line_without_shifting(0);
                }
            }
        }

        self.write(&new_keycodes, shift_down, 0);
        (changed, return_value)
    }
}

impl DearMirlGuiModule for TextInput {
    #[allow(clippy::too_many_lines)]
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        fn shimmer(buffer: &mut Buffer, xy: (usize, usize), new_color: u32) {
            let under = buffer.get_pixel_unsafe(xy);
            buffer.set_pixel_safe(
                xy,
                mirl::graphics::interpolate_color_rgb_u32_f32(
                    under, new_color, 0.5,
                ),
            );
        }
        // Settings
        let text_color = formatting.text_color;
        let text_size_mul = 0.8;
        let background_color_change = -5.0;
        let background_color = formatting.foreground_color;
        let caret_color = formatting.text_color;
        let caret_width = self.line_height / 10;
        let placeholder_color_change = -30.0;
        let line_number_padding_color_change = -30.0;

        let highlight_color = rgb_to_u32(30, 20, 200);

        // Code
        let mut buffer = Buffer::new_empty_with_color(
            (self.width, self.get_height(formatting) as usize),
            mirl::graphics::adjust_brightness_hsl_of_rgb(
                background_color,
                background_color_change,
            ),
        );

        for (idx, text) in self.text.iter().enumerate() {
            let y = (idx * (self.line_height + formatting.vertical_margin)
                + formatting.vertical_margin)
                as crate::DearMirlGuiCoordinateType
                + self.camera.offset_y as crate::DearMirlGuiCoordinateType;
            // Text line
            render::draw_text_antialiased_isize::<{ crate::DRAW_SAFE }>(
                &mut buffer,
                text,
                (self.get_horizontal_text_offset(formatting), y)
                    .try_tuple_into()
                    .unwrap_or_default(),
                text_color,
                self.line_height as f32 * text_size_mul,
                &formatting.font,
            );
        }
        if self.caret[0].is_highlighting()
            && let Some((line_start, line_end, front_pos, back_pos)) = {
                let head = self.caret[0].highlight_pos;
                let tail = self.caret[0].to_position();
                if head == tail {
                    None
                } else if head.line > tail.line {
                    Some((tail.line, head.line, tail.column, head.column))
                } else if tail.line > head.line || tail.column > head.column {
                    Some((head.line, tail.line, head.column, tail.column))
                } else {
                    Some((head.line, tail.line, tail.column, head.column))
                }
            }
        {
            let total_lines = line_end - line_start;
            let text_until = self.text[line_start]
                .chars()
                .take(front_pos)
                .collect::<String>();
            let first_line_offset = render::get_text_width(
                &text_until,
                self.text_height,
                &formatting.font,
            );

            drop(text_until);
            if total_lines == 0 && front_pos != back_pos {
                // Single line selection - this part looks correct
                let text_between = self.text[line_start]
                    .chars()
                    .skip(front_pos)
                    .take(back_pos - front_pos)
                    .collect::<String>();
                let first_line_width = render::get_text_width(
                    &text_between,
                    self.text_height,
                    &formatting.font,
                );
                render::execute_at_rectangle::<true>(
                    &mut buffer,
                    (
                        first_line_offset as isize
                            + self.get_horizontal_text_offset(formatting)
                                as isize,
                        (line_start
                            * (self.line_height + formatting.vertical_margin)
                            + formatting.vertical_margin)
                            as isize,
                    ),
                    (first_line_width as isize, self.line_height as isize),
                    highlight_color,
                    shimmer,
                );
            } else {
                // Multi-line selection
                // First line - highlight from front_pos to end of line
                let first_line_length = mirl::render::get_text_width(
                    &self.text[line_start],
                    self.text_height,
                    &formatting.font,
                ) - first_line_offset;
                render::execute_at_rectangle::<true>(
                    &mut buffer,
                    (
                        first_line_offset as isize
                            + self.get_horizontal_text_offset(formatting)
                                as isize,
                        (line_start
                            * (self.line_height + formatting.vertical_margin)
                            + formatting.vertical_margin)
                            as isize,
                    ),
                    (first_line_length as isize, self.line_height as isize),
                    highlight_color,
                    shimmer,
                );

                // Middle lines - highlight entire lines
                for i in 1..total_lines as isize {
                    render::execute_at_rectangle::<true>(
                        &mut buffer,
                        (
                            self.get_horizontal_text_offset(formatting)
                                as isize,
                            ((line_start.saturating_add_signed(i))
                                * (formatting.vertical_margin
                                    + self.line_height)
                                + formatting.vertical_margin)
                                as isize,
                        ),
                        (
                            mirl::render::get_text_width(
                                &self.text[line_start.saturating_add_signed(i)],
                                self.text_height,
                                &formatting.font,
                            ) as isize,
                            self.line_height as isize,
                        ),
                        highlight_color,
                        shimmer,
                    );
                }

                // Last line - highlight from start of line to back_pos
                let text_until_end = self.text[line_end]
                    .chars()
                    .take(back_pos) // <- Fixed: was front_pos
                    .collect::<String>();
                let last_line_length = render::get_text_width(
                    &text_until_end,
                    self.text_height,
                    &formatting.font,
                );
                render::execute_at_rectangle::<true>(
                    &mut buffer,
                    (
                        self.get_horizontal_text_offset(formatting) as isize, // <- Fixed: start from beginning of line
                        (line_end
                            * (self.line_height + formatting.vertical_margin)
                            + formatting.vertical_margin)
                            as isize,
                    ),
                    (last_line_length as isize, self.line_height as isize), // <- Fixed: just the length
                    highlight_color,
                    shimmer,
                );
            }
        }

        if self.selected == info.container_id {
            let before = self.text[self.caret[0].line()]
                .chars()
                .take(self.caret[0].column())
                .collect::<String>();

            // The normal x position of the cursor
            let offset = render::get_text_width(
                &before,
                self.line_height as f32 * text_size_mul,
                &formatting.font,
            );
            render::execute_at_rectangle::<true>(
                &mut buffer,
                (
                    offset as isize
                        + self.get_horizontal_text_offset(formatting) as isize,
                    self.get_line_height(formatting, 0)
                        + self.get_vertical_text_offset(formatting) as isize,
                ),
                (caret_width as isize, self.line_height as isize),
                caret_color,
                mirl::misc::invert_color_if_same,
            );
        }
        if self.text.len() == 1 && self.text[0].chars().count() == 0 {
            render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
                &mut buffer,
                &self.placeholder_text,
                (
                    self.get_horizontal_text_offset(formatting) as usize,
                    self.get_vertical_text_offset(formatting) as usize,
                ),
                mirl::graphics::adjust_brightness_hsl_of_rgb(
                    text_color,
                    placeholder_color_change,
                ),
                self.line_height as f32 * text_size_mul,
                &formatting.font,
            );
        }
        if self.show_line_numbers {
            // Line number background
            render::draw_rectangle::<{ crate::DRAW_SAFE }>(
                &mut buffer,
                (0, 0),
                (
                    self.line_number_offset as isize
                        + formatting.horizontal_margin as isize * 2,
                    self.get_height(formatting) as isize,
                ),
                mirl::graphics::adjust_brightness_hsl_of_rgb(
                    background_color,
                    line_number_padding_color_change,
                ),
            );
            for idx in 0..self.text.len() {
                let y = (idx * (self.line_height + formatting.vertical_margin)
                    + formatting.vertical_margin)
                    as crate::DearMirlGuiCoordinateType
                    + self.camera.offset_y as crate::DearMirlGuiCoordinateType;
                // Text line number

                draw_text_antialiased_isize::<{ crate::DRAW_SAFE }>(
                    &mut buffer,
                    &(idx + 1).to_string(),
                    (formatting.horizontal_margin as isize, y as isize),
                    text_color,
                    self.line_height as f32 * text_size_mul,
                    &formatting.font,
                );
            }
        }

        // render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
        //   &buffer,
        //   &after,
        //   x as usize + offset as usize,
        //   0,
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
        ((self.line_height + formatting.vertical_margin) * self.max_lines
            + formatting.vertical_margin * 2)
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
        let collision = mirl::math::collision::Rectangle::<_, false>::new(
            0,
            0,
            self.get_width(formatting) as i32,
            self.get_height(formatting) as i32,
        );
        if info.focus_taken == FocusTaken::FunctionallyTaken
            && info.container_id == self.selected
        {
            self.selected = 0;
            self.caret[0].reset_highlighted();
            self.caret[0].restore_default();
        }
        let mut took_functional_focus = false;
        //println!("{} {}", self.selected, info.container_id);

        if let Some(mouse_position) = info.mouse_pos {
            let collides = collision.does_area_contain_point(mouse_position);
            if collides {
                cursor_style = Some(CursorStyle::Text);

                if info.container_id == self.selected
                    && let Some(scroll) = info.mouse_scroll
                {
                    let shift_pressed =
                        info.pressed_keys.contains(&KeyCode::LeftShift)
                            || info.pressed_keys.contains(&KeyCode::RightShift);
                    if scroll != (0.0, 0.0) {
                        took_functional_focus = true;
                        self.handle_scroll(scroll, !shift_pressed, formatting);
                    }
                }
                if info.mouse_info.left.clicked
                    && (self.selected == 0
                        || self.selected == info.container_id)
                {
                    took_functional_focus = true;
                    self.selected = info.container_id;
                    if mouse_position.0 > self.line_number_offset as i32 {
                        let vertical = ((mouse_position.1 as f32
                            - self.camera.offset_y)
                            as usize
                            / self.line_height)
                            .min(self.text.len() - 1);

                        let horizontal =
                            super::misc::get_closest_char_pos_to_mouse_pos(
                                &self.text[vertical],
                                self.line_height as f32,
                                &formatting.font,
                                (mouse_position.0
                                    - self
                                        .get_horizontal_text_offset(formatting)
                                        as i32)
                                    as f32,
                            );
                        let new_caret = Caret::new(vertical, horizontal);
                        if self.caret[0] == new_caret {
                            self.select_structure(0);
                        } else {
                            self.caret[0] = new_caret;
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
            cursor_style = None;
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

            let previous_state = self.text.clone();
            if self.last_states.is_empty() {
                self.last_states
                    .push((previous_state.clone(), self.caret.clone()));
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

            if !do_not_save_state && self.text != previous_state {
                if self.current_state < self.last_states.len().saturating_sub(1)
                {
                    self.last_states.truncate(self.current_state);
                }
                self.last_states.push((self.text.clone(), self.caret.clone()));
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
            text_input_selected: self.selected > 0,
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
    fn apply_new_formatting(&mut self, formatting: &crate::Formatting) {
        self.recalculate_line_number_offset(formatting);
    }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw = super::misc::determine_need_redraw(need_redraw);
    }
}
