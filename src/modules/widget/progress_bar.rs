use mirl::prelude::Buffer;
use mirl::render;
use mirl::{
    //extensions::*,
    graphics::rgba_to_u32,
    math::interpolate,
};

use crate::{
    DearMirlGuiModule,
    module_manager::{InsertionMode, get_formatting},
};
#[derive(Debug, Clone, PartialEq)]
/// A Progress Bar module
pub struct ProgressBar {
    #[allow(missing_docs)]
    pub width: usize,
    #[allow(missing_docs)]
    pub height: usize,
    // From 0.0 to 1.1, 1.1 my beloved
    /// From 0.0 to 1.0
    pub progress: f32,
    #[allow(missing_docs)]
    pub previous_progress: f32,
    // This was a pain to program. The progress animation just went backwards no matter what I tried.
    // ...anyways, the solution was to also interpolate the y coordinate. I blame the time of night for this.
    /// Makes the progress animation go upwards instead of left to right
    pub progress_bar_vertical: bool,
    needs_redraw: std::cell::Cell<bool>,
}
impl ProgressBar {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new(progress: Option<f32>, progress_bar_vertical: bool) -> Self {
        let formatting = get_formatting();
        let height = formatting.height;
        Self {
            width: height * 3,
            height,
            progress: progress.unwrap_or(0.0),
            progress_bar_vertical,
            needs_redraw: std::cell::Cell::new(false),
            previous_progress: 0.0,
        }
    }
    #[must_use]
    /// An inline function for setting a custom height, use [with_width](Self::with_width) for setting the width
    pub const fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    #[must_use]
    /// An inline function for setting a custom width, use [with_height](Self::with_height) for setting the height
    pub const fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
}

impl DearMirlGuiModule for ProgressBar {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.width as crate::DearMirlGuiCoordinateType
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.height as crate::DearMirlGuiCoordinateType
    }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    //#[allow(clippy::too_many_lines)] // Really? What.
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        #[allow(clippy::inline_always)]
        #[inline(always)]
        const fn draw_or_invert(original: u32, under: u32) -> u32 {
            if under == original {
                mirl::graphics::invert_color(original)
            } else {
                original
            }
        }
        let color_change = -10.0;
        let mut buffer = Buffer::new_empty_with_color(
            (self.width, self.height),
            formatting.foreground_color,
        );

        let (width, height, y_pos) = if self.progress_bar_vertical {
            // Vertical mode
            (
                // width
                self.width as isize,
                // height
                interpolate(0.0, self.height as f32, self.progress) as isize,
                // y_pos
                interpolate(self.height as f32, 0.0, self.progress).ceil()
                    as isize,
            )
        } else {
            // Horizontal mode
            (
                // width
                interpolate(0.0, self.width as f32, self.progress) as isize,
                // height
                self.height as isize,
                // y_pos
                0,
            )
        };
        render::draw_rectangle::<{ crate::DRAW_SAFE }>(
            &mut buffer,
            (0, y_pos),
            (width, height),
            mirl::graphics::adjust_brightness_hsl_of_rgb(
                formatting.foreground_color,
                color_change,
            ),
        );
        let worst_case_text = "100.00%";
        let max_text_width = mirl::render::get_text_width(
            worst_case_text,
            self.height as f32,
            &formatting.font,
        );
        let desired_max_width = self.width as f32 / 2.0;

        let scaling = if max_text_width > desired_max_width {
            desired_max_width / max_text_width
        } else {
            1.0
        };
        let text = format!("{:.2?}%", self.progress * 100.0);

        let actual_text_width = mirl::render::get_text_width(
            &text,
            self.height as f32 * scaling,
            &formatting.font,
        );
        let coord = self.width / 2 - actual_text_width as usize / 2;

        render::draw_text_antialiased_execute_at::<false>(
            &mut buffer,
            &text,
            (coord, 0),
            rgba_to_u32(255, 255, 255, 255),
            self.height as f32 * scaling,
            &formatting.font,
            draw_or_invert,
        );

        (buffer, InsertionMode::ReplaceAll)
    }
    #[allow(clippy::float_cmp)]
    fn update(&mut self, _info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        if self.progress != self.previous_progress {
            self.needs_redraw.set(true);
            self.previous_progress = self.progress;
        }
        crate::GuiOutput::empty()
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
