use mirl::{extensions::RepeatData, platform::CursorStyle, prelude::Buffer, render};

use crate::{DearMirlGuiModule, FocusTaken, module_manager::InsertionMode, prelude::get_formatting};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A simple selection module
pub struct Selection {
    /// The selections it contains
    pub text: Vec<String>,
    #[allow(missing_docs)]
    pub height: usize,
    /// The color used
    pub color: u32,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// What buttons are currently selected, can't state more than the obvious with this one
    pub currently_selected: Vec<bool>,
    /// It's wasteful to recalculate the width every frame
    pub width: usize,
    /// It's wasteful to recalculate the total height every frame
    pub total_height: usize,
    /// If only a single button is should be pressed at a time
    pub radio_buttons: bool,
}
impl Selection {
    #[must_use]
    /// For the formatting use the .formatting field from the window
    pub fn new(
        text: &[String],
        radio_buttons: bool,
        initial_states: Option<Vec<bool>>,
    ) -> Self {
        let formatting = get_formatting();
        let height = formatting.height;
        Self {
            text: text.to_vec(),
            height,
            color: mirl::graphics::colors::WHITE,
            needs_redraw: std::cell::Cell::new(true),
            currently_selected: initial_states
                .unwrap_or_else(|| false.repeat_value(text.len())),
            width: 0,
            total_height: 0,
            radio_buttons,
        }
    }
    #[must_use]
    /// An inline function for setting a custom height
    pub const fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    /// Recalculate the total height of the module
    #[must_use]
    pub fn get_total_height(&self, formatting: &crate::Formatting) -> usize {
        let mut total_height = 0;
        for _ in 0..self.text.len() {
            total_height += self.height;
            total_height += formatting.vertical_margin;
        }
        total_height
    }
    /// Recalculate the total width of the module
    #[must_use]
    pub fn get_total_width(&self, formatting: &crate::Formatting) -> usize {
        self.text
            .iter()
            .map(|x| {
                render::get_text_width(x, self.height as f32, &formatting.font)
                    as usize
            })
            .max()
            .unwrap_or(0)
            + self.height
            + formatting.horizontal_margin
    }
    /// Get the button that is currently pressed when in radio button mode, otherwise get the first button that is pressed
    #[must_use]
    pub fn radio_button_get_idx(&self) -> Option<usize> {
        self.currently_selected.iter().position(|&b| b)
    }
}

impl DearMirlGuiModule for Selection {
    fn apply_new_formatting(&mut self, formatting: &crate::Formatting) {
        self.total_height = self.get_total_height(formatting);
        self.width = self.get_total_width(formatting);
    }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        // Button alignment
        let margin_divider = 5;
        let inner_button_color = mirl::graphics::colors::WHITE;

        let mut buffer = Buffer::new_empty((self.width, self.total_height));
        let mut offset = 0;
        for (idx, i) in self.text.iter().enumerate() {
            let margin = self.height / margin_divider;
            let smaller = self.height - margin;
            let margin = margin / 2;
            if self.radio_buttons {
                let t = self.height / 2;
                render::draw_circle::<false, true>(
                    &mut buffer,
                    (t as isize, (offset + t) as isize),
                    t as isize,
                    formatting.foreground_color,
                );
                if self.currently_selected[idx] {
                    render::draw_circle::<true, true>(
                        &mut buffer,
                        (t as isize, (offset + t) as isize),
                        smaller as isize / 2,
                        inner_button_color,
                    );
                }
            } else {
                render::draw_rectangle::<{ crate::DRAW_SAFE }>(
                    &mut buffer,
                    (0, offset as isize),
                    (self.height as isize, self.height as isize),
                    formatting.foreground_color,
                );
                if self.currently_selected[idx] {
                    render::draw_rectangle::<{ crate::DRAW_SAFE }>(
                        &mut buffer,
                        (margin as isize, offset as isize + margin as isize),
                        (smaller as isize, smaller as isize),
                        inner_button_color,
                    );
                }
            }
            render::draw_text_antialiased::<{ crate::DRAW_SAFE }>(
                &mut buffer,
                i,
                (self.height + formatting.horizontal_margin, offset),
                self.color,
                self.height as f32,
                &formatting.font,
            );
            offset += self.height + formatting.vertical_margin;
        }
        (buffer, InsertionMode::ReplaceAll)
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.total_height as crate::DearMirlGuiCoordinateType
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.width as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        if info.focus_taken.is_focus_taken() {
            return crate::GuiOutput::empty();
        }
        let formatting = get_formatting();
        let mut new_cursor_style = None;
        let mut focus_taken = FocusTaken::FocusFree;
        if let Some(mouse_pos) = info.mouse_pos {
            let mut offset = 0;
            // if info.container_id == 3 {
            //     println!(">{mouse_pos:?}");
            // }
            for (idx, _) in self.text.iter().enumerate() {
                let collides = if self.radio_buttons && false {
                    // TODO: THIS IS CAUSING PROBLEMS. It could be because of how the Circle has been split into a pos and a shape
                    mirl::math::geometry::Pos2D::<
                        _,
                        mirl::math::collision::Circle<f32, false>,
                    >::new(
                        (0.0, offset as f32),
                        mirl::math::collision::Circle::new(self.height as f32),
                    )
                    .does_area_contain_point(mouse_pos)
                } else {
                    mirl::math::geometry::Pos2D::<
                        _,
                        mirl::math::collision::Rectangle<f32, false>,
                    >::new(
                        (0.0, offset as f32),
                        mirl::math::collision::Rectangle::new((
                            self.height as f32,
                            self.height as f32,
                        )),
                    )
                    .does_area_contain_point(mouse_pos)
                };
                if collides {
                    new_cursor_style = Some(CursorStyle::Pointer);
                    focus_taken = FocusTaken::VisuallyTaken;
                    if info.mouse_info.left.clicked {
                        focus_taken = FocusTaken::FunctionallyTaken;
                        let was_true = self.currently_selected[idx];
                        if self.radio_buttons {
                            self.currently_selected =
                                false.repeat_value(self.text.len());
                        }
                        self.needs_redraw.set(true);

                        self.currently_selected[idx] = !was_true;
                        break;
                    }
                }

                offset += self.height + formatting.vertical_margin;
            }
        }
        crate::GuiOutput {
            new_cursor_style,
            focus_taken,
            new_cursor_position: None,
            hide_cursor: false,
            new_clipboard_data: None,
            text_input_selected: false,
            request_clipboard_data: false,
        }
    }

    fn need_redraw(&mut self) -> bool {
        if self.needs_redraw.get() {
            self.needs_redraw.set(false);
            true
        } else {
            false
        }
    }
}
