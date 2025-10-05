use mirl::{
    extensions::*,
    graphics::{rgba_to_u32, rgba_u32_to_u32},
};

use crate::{render, Buffer, CursorStyle, DearMirlGuiModule, FocusTaken, InsertionMode};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A slider/progress module
pub struct Slider<ProgressType: num_traits::Float> {
    #[allow(missing_docs)]
    pub width: usize,
    #[allow(missing_docs)]
    pub height: usize,
    /// From 0.0 to 1.0
    pub progress: ProgressType,
    /// Width of the slider thumb
    pub slider_width: usize,
    /// If the slider thumb is currently being dragged
    pub dragging: usize,
    /// If the slider should wrap around instead of stopping at the ends
    pub wrap: bool,
    /// A small value so progress doesn't flip between 0.0 and 1.0
    pub eps: ProgressType,
    #[allow(missing_docs)]
    needs_redraw: std::cell::Cell<bool>,
}
impl<ProgressType: num_traits::Float> Slider<ProgressType> {
    #[must_use]
    #[allow(missing_docs, clippy::unwrap_used, clippy::missing_panics_doc)]
    pub fn new(
        height: usize,
        width: Option<usize>,
        // normalized_range: bool,
        // min: Option<isize>,
        // max: Option<isize>,
        // visual_min: Option<isize>,
        // visual_max: Option<isize>,
        progress: Option<ProgressType>,
        slider_width: Option<usize>,
        wrap: bool,
    ) -> Self {
        let eps = 1e-6;
        let eps = num_traits::NumCast::from(eps).unwrap();
        Self {
            width: width.unwrap_or(height * 3),
            height,
            progress: progress.unwrap_or(eps),
            // min: min.unwrap_or_else(|| {
            //     if normalized_range {
            //         0
            //     } else {
            //         panic!("Not normalized slider but also no minimum set")
            //     }
            // }),
            // max: max.unwrap_or_else(|| {
            //     if normalized_range {
            //         0
            //     } else {
            //         panic!("Not normalized slider but also no maximum set")
            //     }
            // }),
            // normalized_range,
            slider_width: slider_width.unwrap_or(height / 5),
            dragging: 0,
            wrap,
            eps,
            needs_redraw: std::cell::Cell::new(true),
        }
    }
}

impl<
    ProgressType: std::fmt::Debug
        + num_traits::Float
        + num_traits::NumCast
        + num_traits::ConstOne
        + num_traits::ConstZero
        + 'static
        + std::marker::Send,
> DearMirlGuiModule for Slider<ProgressType>
{
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.width as isize
    }
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.height as isize
    }
    fn set_need_redraw(&self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    #[allow(clippy::unwrap_used)] // It ain't gonna crash
    #[allow(clippy::too_many_lines)] // Really? What.
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        #[inline(always)]
        #[allow(clippy::inline_always)]
        const fn draw_or_invert(original: u32, under: u32) -> u32 {
            if under == original {
                mirl::graphics::invert_color(original)
            } else {
                original
            }
        }
        let buffer = Buffer::new_empty_with_color(
            self.width,
            self.height,
            formatting.foreground_color,
        );

        let position = mirl::math::interpolate(
            ProgressType::ZERO,
            num_traits::NumCast::from(self.width - self.slider_width).unwrap(),
            self.progress,
        );

        render::draw_rectangle::<{ crate::DRAW_SAFE }>(
            &buffer,
            num_traits::NumCast::from(position).unwrap(),
            0,
            self.slider_width as isize,
            self.height as isize,
            rgba_to_u32(255, 255, 255, 255),
        );

        let worst_case_text = "1.0000";
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
        let text = format!("{:.4?}", self.progress);

        let actual_text_width = mirl::render::get_text_width(
            &text,
            self.height as f32 * scaling,
            &formatting.font,
        );
        let coord = self.width / 2 - actual_text_width as usize / 2;

        render::draw_text_antialiased_execute_at::<false>(
            &buffer,
            &text,
            (coord, 0),
            rgba_u32_to_u32(255, 255, 255, 255),
            self.height as f32 * scaling,
            &formatting.font,
            draw_or_invert,
        );

        (buffer, InsertionMode::ReplaceAll)
    }
    #[allow(clippy::unwrap_used)] // It ain't gonna crash
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        if info.focus_taken.is_focus_taken() {
            return crate::GuiOutput::empty();
        }
        let mut took_focus = if self.dragging == info.container_id {
            FocusTaken::FunctionallyTaken
        } else {
            FocusTaken::FocusFree
        };

        let mut cursor_style: Option<CursorStyle> = None;
        if let Some(mouse_pos) = info.mouse_pos {
            let position = mirl::math::interpolate(
                ProgressType::ZERO,
                num_traits::NumCast::from(self.width).unwrap(),
                self.progress,
            );
            let slider_collision: mirl::math::collision::Rectangle<
                isize,
                false,
            > = mirl::math::collision::Rectangle::new(
                num_traits::NumCast::from(position).unwrap(),
                0,
                self.slider_width as isize,
                self.height as isize,
            );
            let mouse_collides_with_slider_handle =
                slider_collision.does_area_contain_point(mouse_pos);

            let mut new_progress = self.progress;
            if (self.dragging == info.container_id && info.mouse_info.left.down)
                || (info.mouse_info.left.clicked
                    && mouse_collides_with_slider_handle
                    && self.dragging == 0)
            {
                self.dragging = info.container_id;
                cursor_style = Some(CursorStyle::ResizeHorizontally);
                new_progress = super::misc::adjust_progress_by_mouse(
                    self.progress,
                    num_traits::NumCast::from(info.mouse_pos_delta.0).unwrap(),
                    num_traits::NumCast::from(self.width).unwrap(),
                );
            } else if mouse_collides_with_slider_handle {
                cursor_style = Some(CursorStyle::CenteredPointer);
                self.dragging = 0;
            } else {
                self.dragging = 0;
            }
            if mirl::math::collision::Rectangle::<_, false>::new(
                0,
                0,
                self.width as isize,
                self.height as isize,
            )
            .does_area_contain_point(mouse_pos)
                && let Some(mouse_scroll) = info.mouse_scroll
            {
                if mouse_scroll.0 != 0 {
                    took_focus = FocusTaken::FunctionallyTaken;
                    new_progress = new_progress
                        + num_traits::NumCast::from(
                            0.05 * mouse_scroll.0.sign() as f32,
                        )
                        .unwrap(); //(mouse_scroll.0 as f32 / 100.0)
                } else if mouse_scroll.1 != 0 {
                    took_focus = FocusTaken::FunctionallyTaken;
                    new_progress = new_progress
                        + num_traits::NumCast::from(
                            0.05 * mouse_scroll.1.sign() as f32,
                        )
                        .unwrap(); //(mouse_scroll.1 as f32 / 100.0)
                }
            }
            if self.wrap {
                let eps = self.eps;
                let p = new_progress;
                new_progress = if p <= ProgressType::ZERO {
                    p + ProgressType::ONE - eps
                } else if p >= ProgressType::ONE {
                    p - ProgressType::ONE + eps
                } else {
                    p
                };
            }
            if self.dragging > 0 {
                self.needs_redraw = std::cell::Cell::new(true);
            }
            self.progress =
                new_progress.clamp(ProgressType::ZERO, ProgressType::ONE);
        }
        crate::GuiOutput {
            new_cursor_style: cursor_style,
            new_clipboard_data: None,
            new_cursor_position: None,
            focus_taken: took_focus,
            hide_cursor: false,
            text_input_selected: false,
            request_clipboard_data: false,
        }
    }
    fn need_redraw(&self) -> bool {
        if self.needs_redraw.get() {
            self.needs_redraw.set(false);
            true
        } else {
            false
        }
    }
}
