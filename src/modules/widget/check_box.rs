use mirl::{
    extensions::{Tuple2Into, TupleOps},
    platform::CursorStyle,
};

use crate::{Buffer, DearMirlGuiModule, FocusTaken, InsertionMode, render};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// A check box, what more to describe?
pub struct CheckBox {
    /// How many times this box has been clicked
    pub checked: usize,
    /// How many click states exist
    pub states: Vec<Buffer>,
    /// The text next to he check box
    pub text: String,
    #[allow(missing_docs)]
    pub height: usize,
    /// The space between the check box and the text
    pub margin: usize,
    #[allow(missing_docs)]
    pub needs_redraw: bool,
}
impl CheckBox {
    /// Set the initial state
    #[must_use]
    pub const fn with_state(mut self, state: usize) -> Self {
        self.checked = state % self.states.len();
        self
    }
}

impl CheckBox {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(
        height: usize,
        text: String,
        states: Option<Vec<Buffer>>,
        checked: Option<usize>,
    ) -> Self {
        let anti_margin = height - height / 5;
        Self {
            height,
            text,
            checked: checked.unwrap_or_default(),
            states: states.unwrap_or_else(|| {
                Vec::from([
                    Buffer::new_empty((0, 0)),
                    super::misc::draw_cross(anti_margin, 2),
                ])
            }),
            margin: height / 10,
            needs_redraw: true,
        }
    }
    #[must_use]
    /// Create a new check box with 2 possible states
    pub fn new_2_state(height: usize, text: String) -> Self {
        let anti_margin = height - height / 5;
        Self {
            height,
            text,
            checked: 0,
            states: Vec::from([
                Buffer::new_empty((0, 0)),
                super::misc::draw_cross(anti_margin, 2),
            ]),
            margin: height / 10,
            needs_redraw: true,
        }
    }
    #[must_use]
    /// Create a new check box with 3 possible states
    pub fn new_3_state(height: usize, text: String) -> Self {
        let anti_margin = height - height / 5;
        Self {
            height,
            text,
            checked: 0,
            states: Vec::from([
                Buffer::new_empty((0, 0)),
                super::misc::draw_cross(anti_margin, 2),
                super::misc::draw_blocking(
                    anti_margin,
                    mirl::graphics::colors::WHITE,
                ),
            ]),
            margin: height / 10,
            needs_redraw: true,
        }
    }
    /// Checks if the box is un-ticked and inverts the result
    #[must_use]
    pub const fn is_ticked(&self) -> bool {
        self.checked != 0
    }
}

impl DearMirlGuiModule for CheckBox {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.height as crate::DearMirlGuiCoordinateType
    }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw = super::misc::determine_need_redraw(need_redraw);
    }
    fn get_width(
        &mut self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        (self.height
            + self.margin
            + render::get_text_width(
                &self.text,
                self.height as f32,
                &formatting.font,
            ) as usize) as crate::DearMirlGuiCoordinateType
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        let mut buffer = Buffer::new_empty((
            self.get_width(formatting) as usize,
            self.height,
        ));
        render::draw_rectangle::<{ crate::DRAW_SAFE }>(
            &mut buffer,
            (0, 0),
            (self.height as isize, self.height as isize),
            formatting.foreground_color,
        );

        render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
            &mut buffer,
            &self.text,
            (self.height + self.margin, 0),
            mirl::graphics::colors::WHITE,
            self.height as f32,
            &formatting.font,
        );

        let margin = self.height / 5;
        let anti_margin = self.height - margin;

        let to_draw = &self.states[self.checked];
        if buffer.width == anti_margin && buffer.height == anti_margin {
            render::draw_buffer_on_buffer_1_to_1::<true, true, false, true>(
                &mut buffer,
                &Buffer::generate_fallback((anti_margin, anti_margin), 2),
                (margin, margin)
                    .div((2, 2))
                    .try_tuple_into()
                    .unwrap_or_default(),
            );
        } else {
            render::draw_buffer_on_buffer_1_to_1::<true, true, false, true>(
                &mut buffer,
                to_draw,
                (margin, margin)
                    .div((2, 2))
                    .try_tuple_into()
                    .unwrap_or_default(),
            );
        }

        (buffer, InsertionMode::ReplaceAll)
    }
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        if info.focus_taken.is_focus_taken() {
            return crate::GuiOutput::empty();
        }
        let collision: mirl::math::collision::Rectangle<_, true> =
            mirl::math::collision::Rectangle::new(
                0,
                0,
                self.height as i32,
                self.height as i32,
            );
        if let Some(mouse_pos) = info.mouse_pos {
            let collides = collision.does_area_contain_point(mouse_pos);
            if collides {
                let cursor_style = Some(CursorStyle::Pointer);
                let focus_taken = if info.mouse_info.left.clicked {
                    self.needs_redraw = true;
                    self.checked = (self.checked + 1) % self.states.len();
                    FocusTaken::FunctionallyTaken
                } else {
                    FocusTaken::VisuallyTaken
                };

                return crate::GuiOutput {
                    new_cursor_style: cursor_style,
                    focus_taken,
                    new_clipboard_data: None,
                    text_input_selected: false,
                    hide_cursor: false,
                    new_cursor_position: None,
                    request_clipboard_data: false,
                };
            }
        }
        crate::GuiOutput::empty()
    }
    fn need_redraw(&mut self) -> bool {
        if self.needs_redraw {
            self.needs_redraw = false;
            true
        } else {
            false
        }
    }
}
