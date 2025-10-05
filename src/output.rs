use mirl::platform::CursorStyle;

use crate::FocusTaken;


#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // Once again not a state machine
/// A struct holding every bit of info the `DearMirlGui` struct and every module returns
pub struct GuiOutput {
    /// Requested cursor style
    pub new_cursor_style: Option<mirl::platform::CursorStyle>,
    /// If this window took focus
    pub focus_taken: FocusTaken,
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
impl GuiOutput {
    /// Compare two outputs and supplement missing values
    #[must_use]
    pub fn or(&self, rhs: Self) -> Self {
        Self {
            new_cursor_style: self.new_cursor_style.or(rhs.new_cursor_style),
            focus_taken: self.focus_taken | rhs.focus_taken,
            new_cursor_position: self
                .new_cursor_position
                .or(rhs.new_cursor_position),
            hide_cursor: self.hide_cursor || rhs.hide_cursor,
            new_clipboard_data: self
                .new_clipboard_data
                .clone()
                .or(rhs.new_clipboard_data),
            text_input_selected: self.text_input_selected
                || rhs.text_input_selected,
            request_clipboard_data: self.request_clipboard_data
                || rhs.request_clipboard_data,
        }
    }
}

impl std::ops::BitOr for GuiOutput {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}
impl std::ops::BitOrAssign for GuiOutput {
    fn bitor_assign(&mut self, rhs: Self) {
        let value = self.or(rhs);

        *self = value;
    }
}

impl GuiOutput {
    #[must_use]
    /// Create a new output with everything set to None or False
    pub const fn default(took_focus: FocusTaken) -> Self {
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
    #[must_use]
    /// Everything set to false
    pub const fn empty() -> Self {
        Self::default(FocusTaken::FocusFree)
    }
    // pub const fn set_cursor_style(
    //     &mut self,
    //     cursor_style: Option<CursorStyle>,
    // ) -> &mut Self {
    //     self.new_cursor_style = cursor_style;
    //     self
    // }
}
impl GuiOutput {
    #[must_use]
    /// Set the cursor style
    pub const fn with_optional_cursor(
        mut self,
        cursor_style: Option<CursorStyle>,
    ) -> Self {
        self.new_cursor_style = cursor_style;
        self
    }

    #[must_use]
    /// Set the cursor style
    pub const fn with_cursor(mut self, cursor_style: CursorStyle) -> Self {
        self.new_cursor_style = Some(cursor_style);
        self
    }
    #[must_use]
    /// Set whether the cursor is hidden
    pub const fn hide_cursor(mut self, hide: bool) -> Self {
        self.hide_cursor = hide;
        self
    }

    #[must_use]
    /// Set which focus state was taken
    pub const fn set_focus_to(mut self, took_focus: FocusTaken) -> Self {
        self.focus_taken = took_focus;
        self
    }

    #[must_use]
    /// Set the new cursor position
    pub const fn with_cursor_position(
        mut self,
        pos: Option<(isize, isize)>,
    ) -> Self {
        self.new_cursor_position = pos;
        self
    }

    #[must_use]
    /// Set new clipboard data
    pub fn with_clipboard_data(
        mut self,
        data: Option<mirl::platform::file_system::FileData>,
    ) -> Self {
        self.new_clipboard_data = data;
        self
    }

    #[must_use]
    /// Mark text input as selected or not
    pub const fn select_text_input(mut self, selected: bool) -> Self {
        self.text_input_selected = selected;
        self
    }

    #[must_use]
    /// Request clipboard data
    pub const fn request_clipboard(mut self, request: bool) -> Self {
        self.request_clipboard_data = request;
        self
    }
}