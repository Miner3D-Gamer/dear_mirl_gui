use mirl::{
    extensions::*, graphics::rgba_to_u32, math::{Bounded, ConstOne, ConstZero}, platform::CursorStyle, prelude::Buffer, render
};

use crate::{
    DearMirlGuiModule, FocusTaken, WhatAmI,
    module_manager::{InsertionMode, get_formatting},
    modules::misc::new_buffer_error,
};
// TODO: ADD ME
// /// The type of progress
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum Progress<
//     ProgressType: num_traits::Float,
//     LimitType: mirl::math::NumberWithMonotoneOps,
// > {
//     /// 0.5 from a range of 0..100 would be 50
//     Percentual(ProgressType),
//     /// 50 from a range of 0..100 would be 0.5
//     Value(LimitType),
// }

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
/// A slider/progress module
pub struct Slider<ProgressType, LimitType: mirl::math::NumberWithMonotoneOps> {
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
    ProgressType: TryFromPatch<LimitType>
        + TryFromPatch<f64>
        + mirl::extensions::Round
        + core::ops::Mul<Output = ProgressType>
        + core::ops::Div<Output = ProgressType>
        + Copy,
    LimitType: mirl::math::NumberWithMonotoneOps
        + Bounded
        + TryFromPatch<ProgressType>
        + Copy
        + core::ops::Mul<Output = LimitType>
        + core::ops::Div<Output = LimitType>
        + mirl::extensions::Round,
> Slider<ProgressType, LimitType>
{
    #[must_use]
    #[allow(missing_docs, clippy::unwrap_used, clippy::missing_panics_doc)]
    pub fn new(
        // normalized_range: bool,
        // min: Option<isize>,
        // max: Option<isize>,
        // visual_min: Option<isize>,
        // visual_max: Option<isize>,
        progress: Option<ProgressType>,
        wrap: bool,
        range: Option<std::ops::Range<LimitType>>,
    ) -> Option<Self> {
        let eps = 1e-6;
        let eps = ProgressType::try_from_value(eps)?;
        let formatting = get_formatting();
        let height = formatting.height;
        Some(Self {
            width: height * 3,
            height,
            progress: progress.unwrap_or(eps),
            slider_width: height / 5,
            dragging: 0,
            wrap,
            eps,
            needs_redraw: (true),
            range: range.unwrap_or_else(|| {
                LimitType::min_value()..LimitType::max_value()
            }),
        })
    }
    #[must_use]
    /// An inline function for setting a custom height, use [with_width](Self::with_width) for setting the width
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self.slider_width =
            calculate_slider_width(self.width as f32, self.height as f32)
                as usize;
        self
    }
    #[must_use]
    /// An inline function for setting a custom width, use [with_height](Self::with_height) for setting the height
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self.slider_width =
            calculate_slider_width(self.width as f32, self.height as f32)
                as usize;
        self
    }
    #[must_use]
    /// An inline function for setting a custom width, use [with_height](Self::with_height) for setting the height
    pub const fn with_slider_width(mut self, width: usize) -> Self {
        self.slider_width = width;
        self
    }
}

const impl<
    ProgressType: [const] TryFromPatch<LimitType>
        + [const] TryFromPatch<f64>
        + [const] mirl::extensions::Round
        + [const] core::ops::Mul<Output = ProgressType>
        + [const] core::ops::Div<Output = ProgressType>
        + Copy,
    LimitType: [const] mirl::math::NumberWithMonotoneOps
        + [const] Bounded
        + [const] TryFromPatch<ProgressType>
        + Copy
        + [const] core::ops::Mul<Output = LimitType>
        + [const] core::ops::Div<Output = LimitType>
        + [const] mirl::extensions::Round,
> Slider<ProgressType, LimitType>
{
    #[allow(clippy::missing_const_for_fn)] // Mate, it's already const
    /// Based on the given range, get the value associated
    pub fn get_value(&self) -> Option<LimitType> {
        self.range.get_value_from_percent(self.progress)
    }
    #[allow(clippy::missing_const_for_fn)] // Mate, it's already const as well
    /// Based on the given range, get the value associated. Returns None when it fails
    pub fn set_value(&mut self, value: LimitType) -> Option<()> {
        self.progress = self.range.get_percent_from_value(value)?;
        Some(())
    }
}

impl<
    ProgressType: std::fmt::Debug
        + ConstOne
        + ConstZero
        + 'static
        + std::marker::Send
        + mirl::extensions::TryFromPatch<usize>
        + Copy
        + core::ops::Add<Output = ProgressType>
        + core::ops::Sub<Output = ProgressType>
        + core::ops::Mul<Output = ProgressType>
        + TryIntoPatch<f32>
        + TryFromPatch<f32>
        + core::ops::Div<Output = ProgressType>
        + core::cmp::PartialOrd
        + mirl::extensions::Clamp,
    // + core::cmp::Ord,
    LimitType: mirl::math::NumberWithMonotoneOps
        + ConstZero
        + std::fmt::Debug
        + std::marker::Send
        + 'static,
> DearMirlGuiModule for Slider<ProgressType, LimitType>
where
    isize: mirl::extensions::TryFromPatch<ProgressType>,
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
        let Some(pos_end) = (self.width - self.slider_width).try_into_value()
        else {
            return new_buffer_error(&format!(
                "Unable to convert from usize to {} (Value: {})",
                self.progress.what_am_i(),
                (self.width - self.slider_width)
            ));
        };

        let position =
            mirl::math::interpolate(ProgressType::ZERO, pos_end, self.progress);

        let Some(draw_pos_start) = isize::try_from_value(position) else {
            return new_buffer_error(&format!(
                "Unable to convert from {} to isize (Value: {:?})",
                self.progress.what_am_i(),
                position
            ));
        };
        render ::draw_rectangle::<{ crate::DRAW_SAFE }>(
            &mut buffer,
            (draw_pos_start, 0),
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
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        // TODO: Put mouse dragging into its own function

        if info.focus_taken.is_focus_taken() {
            return crate::GuiOutput::empty();
        }
        let mut took_focus = if self.dragging == info.container_id {
            FocusTaken::FunctionallyTaken
        } else {
            FocusTaken::FocusFree
        };

        let mut cursor_style: Option<CursorStyle> = None;
        // Click to drag, drag, or ignore
        if let Some(mouse_pos) = info.mouse_pos {
            let position = mirl::math::interpolate(
                ProgressType::ZERO,
                (self.width).try_into_value().unwrap(),
                self.progress,
            );
            let slider_collision: mirl::math::geometry::Pos2D<
                _,
                mirl::math::collision::Rectangle<_, false>,
            > = mirl::math::geometry::Pos2D::new(
                ((position).try_into_value().unwrap(), 0.0),
                mirl::math::collision::Rectangle::new((
                    self.slider_width as f32,
                    self.height as f32,
                )),
            );
            // Condition for current mouse to move the slider cursor
            let mouse_collides_with_slider_handle =
                slider_collision.does_area_contain_point(mouse_pos);

            let already_dragging =
                self.dragging == info.container_id && info.mouse_info.left.down;

            let starting_to_drag = info.mouse_info.left.clicked
                && mouse_collides_with_slider_handle
                && self.dragging == 0;

            let mut new_progress = self.progress;

            if already_dragging || starting_to_drag {
                self.dragging = info.container_id;
                cursor_style = Some(CursorStyle::ResizeHorizontally);
                new_progress = super::misc::adjust_progress_by_mouse(
                    self.progress,
                    ProgressType::try_from_value(info.mouse_pos_delta.0)
                        .unwrap(),
                    ProgressType::try_from_value(self.width).unwrap(),
                );
            } else if mouse_collides_with_slider_handle {
                cursor_style = Some(CursorStyle::CenteredPointer);
                self.dragging = 0;
            } else {
                self.dragging = 0;
            }
            if mirl::math::geometry::Pos2D::<
                _,
                mirl::math::collision::Rectangle<_, false>,
            >::new(
                (0.0, 0.0),
                mirl::math::collision::Rectangle::new((
                    self.width as f32,
                    self.height as f32,
                )),
            )
            .does_area_contain_point(mouse_pos)
            {
                new_progress = if info.mouse_scroll.0 != 0.0 {
                    took_focus = FocusTaken::FunctionallyTaken;
                    new_progress
                        + ProgressType::try_from_value(
                            0.05 * info.mouse_scroll.0.sign(),
                        )
                        .unwrap() //(mouse_scroll.0 as f32 / 100.0)
                } else if info.mouse_scroll.1 != 0.0 {
                    took_focus = FocusTaken::FunctionallyTaken;
                    new_progress
                        + ProgressType::try_from_value(
                            0.05 * info.mouse_scroll.1.sign(),
                        )
                        .unwrap() //(mouse_scroll.1 as f32 / 100.0)
                } else {
                    new_progress
                };
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
                new_progress.clamped(ProgressType::ZERO, ProgressType::ONE);
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
#[must_use]
/// Calculate the slider arm width
pub fn calculate_slider_width(width: f32, height: f32) -> f32 {
    ((width + height).sqrt() * (width + height)) / (width + height)
}
