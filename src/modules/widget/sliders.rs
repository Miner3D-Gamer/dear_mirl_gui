use mirl::{extensions::*, graphics::rgba_to_u32};

use crate::{
    Buffer, CursorStyle, DearMirlGuiModule, FocusTaken, InsertionMode, render,
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
/// A slider/progress module
pub struct Slider<
    ProgressType: num_traits::Float,
    LimitType: mirl::math::NumberWithMonotoneOps,
> {
    /// The width of the module
    pub width: usize,
    /// The height of the module
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
    /// If the current module needs to be redrawn for changes to take visual effect
    pub needs_redraw: bool,
    /// The range it is limited to
    pub range: std::ops::Range<LimitType>,
}
impl<
    ProgressType: num_traits::Float + FromPatch<LimitType>,
    LimitType: mirl::math::NumberWithMonotoneOps
        + num_traits::Bounded
        + FromPatch<ProgressType>
        + Copy,
> Slider<ProgressType, LimitType>
{
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
        range: Option<std::ops::Range<LimitType>>,
    ) -> Self {
        let eps = 1e-6;
        let eps = num_traits::NumCast::from(eps).unwrap();
        Self {
            width: width.unwrap_or(height * 3),
            height,
            progress: progress.unwrap_or(eps),
            slider_width: slider_width.unwrap_or(height / 5),
            dragging: 0,
            wrap,
            eps,
            needs_redraw: (true),
            range: range
                .unwrap_or_else(|| LimitType::min_value()..LimitType::max_value()),
        }
    }
    /// Based on the given range, get the value associated
    pub fn get_value(&self) -> LimitType {
        self.range.get_value_from_percent(self.progress)
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
    LimitType: mirl::math::NumberWithMonotoneOps
        + num_traits::ConstZero
        + std::fmt::Debug
        + std::marker::Send
        + 'static,
> DearMirlGuiModule for Slider<ProgressType, LimitType>
{
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
        self.needs_redraw = super::misc::determine_need_redraw(need_redraw);
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
        let mut buffer = Buffer::new_empty_with_color(
            (self.width, self.height),
            formatting.foreground_color,
        );

        let position = mirl::math::interpolate(
            ProgressType::ZERO,
            num_traits::NumCast::from(self.width - self.slider_width).unwrap(),
            self.progress,
        );

        render::draw_rectangle::<{ crate::DRAW_SAFE }>(
            &mut buffer,
            (num_traits::NumCast::from(position).unwrap(), 0),
            (self.slider_width as isize, self.height as isize),
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
            let slider_collision: mirl::math::collision::Rectangle<i32, false> =
                mirl::math::collision::Rectangle::new(
                    num_traits::NumCast::from(position).unwrap(),
                    0,
                    self.slider_width as i32,
                    self.height as i32,
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
                self.width as i32,
                self.height as i32,
            )
            .does_area_contain_point(mouse_pos)
                && let Some(mouse_scroll) = info.mouse_scroll
            {
                if mouse_scroll.0 != 0.0 {
                    took_focus = FocusTaken::FunctionallyTaken;
                    new_progress = new_progress
                        + num_traits::NumCast::from(
                            0.05 * mouse_scroll.0.sign(),
                        )
                        .unwrap(); //(mouse_scroll.0 as f32 / 100.0)
                } else if mouse_scroll.1 != 0.0 {
                    took_focus = FocusTaken::FunctionallyTaken;
                    new_progress = new_progress
                        + num_traits::NumCast::from(
                            0.05 * mouse_scroll.1.sign(),
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
                self.needs_redraw = true;
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
    fn need_redraw(&mut self) -> bool {
        if self.needs_redraw {
            self.needs_redraw = false;
            true
        } else {
            false
        }
    }
}
