use mirl::{
    graphics::rgba_u32_to_u32,
    math::{get_center_position_of_object_for_object, interpolate},
};

use crate::{Buffer, DearMirlGuiModule, InsertionMode, get_formatting, render};

#[derive(Debug, Clone, PartialEq)]
/// A simple lever module
pub struct Lever {
    /// The progress -> 0.0 is down 1.0 is up
    pub elevation: f32,
    #[allow(missing_docs)]
    pub width: usize,
    #[allow(missing_docs)]
    pub height: usize,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// The container from which the user is dragging the handle
    pub selected: usize,
    /// How big the hole should be compared to the base width
    pub hole_size: f32,
    /// How big the stick should be compared to the hole with
    pub stick_size: f32,
    /// How wide the hole should be compared to the base width
    pub handle_size_width: f32,
    /// How tall the hole should be compared to the handle width
    pub handle_size_height: f32,
    /// How big the base should be to the configured width
    pub base_size: f32,
}
impl Lever {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(width: usize, height: usize) -> Self {
        let hole_size = 0.6;
        let stick_size = 0.9;
        let handle_size_width = 1.2;
        let handle_size_height = 0.5;
        let base_size = 1.0 / 2.0;
        Self {
            width,
            height,
            elevation: 0.0,
            needs_redraw: std::cell::Cell::new(true),
            selected: 0,
            hole_size,
            stick_size,
            handle_size_width,
            handle_size_height,
            base_size,
        }
    }
}

impl DearMirlGuiModule for Lever {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn set_need_redraw(&self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    #[allow(clippy::too_many_lines)]
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        self.needs_redraw.set(false);
        let buffer = Buffer::new_empty(self.width, self.height);
        let handle_color = formatting.foreground_color;
        let base_color = mirl::graphics::interpolate_color_rgb_u32_f32(
            formatting.background_color,
            formatting.foreground_color,
            0.5,
        );
        let hole_color = mirl::graphics::color_presets::BLACK;
        let stick_color = rgba_u32_to_u32(100, 60, 30, 255);

        let base_width = ((self.width as f32) * self.base_size) as usize;
        let base_height =
            self.height - formatting.vertical_margin * 2 - base_width;
        let handle_width =
            (base_width as f32 * self.handle_size_width) as usize;
        let handle_height =
            (handle_width as f32 * self.handle_size_height) as usize;

        let margin = ((base_width as f32) * (1.0 - self.hole_size)) / 2.0;
        let hole_width = 2.0f32.mul_add(-margin, base_width as f32) as usize;
        let hole_height = 2.0f32.mul_add(-margin, base_height as f32) as usize;

        let hole_offset = get_center_position_of_object_for_object(
            hole_width,
            hole_height,
            base_width,
            base_height,
        );
        let base_offset = get_center_position_of_object_for_object(
            base_width,
            base_height,
            self.width,
            self.height,
        );
        let handle_offset = get_center_position_of_object_for_object(
            handle_width,
            handle_height,
            self.width,
            self.height,
        );

        render::draw_rectangle::<true>(
            &buffer,
            base_offset.0 as isize,
            base_offset.1 as isize,
            base_width as isize,
            base_height as isize,
            base_color,
        );

        let hole_y = base_offset.1 + hole_offset.1;
        render::draw_rectangle::<true>(
            &buffer,
            (base_offset.0 + hole_offset.0) as isize,
            hole_y as isize,
            hole_width as isize,
            hole_height as isize,
            hole_color,
        );

        let hole_top = hole_y;
        let hole_bottom = hole_y + hole_height;
        let handle_half_height = handle_height / 2;

        let handle_min_pos = hole_top as isize - handle_half_height as isize;
        let handle_max_pos = hole_bottom as isize - handle_half_height as isize;

        let handle_pos = interpolate(
            handle_min_pos as f32,
            handle_max_pos as f32,
            self.elevation,
        ) as isize;

        let hole_center_x =
            (base_offset.0 + hole_offset.0 + hole_width / 2) as isize;
        let hole_center_y = (hole_y + hole_height / 2) as isize;
        //let handle_center_x = (handle_offset.0 + handle_width / 2) as isize;
        let handle_center_y = handle_pos + (handle_height as isize / 2);

        let stick_width =
            std::cmp::max(2, (hole_width as f32 * self.stick_size) as isize);
        let stick_x = hole_center_x - stick_width / 2;

        let (stick_y, stick_height) = if handle_center_y < hole_center_y {
            (handle_center_y, hole_center_y - handle_center_y)
        } else {
            (hole_center_y, handle_center_y - hole_center_y)
        };

        if stick_height > 0 {
            render::execute_at_rectangle::<true>(
                &buffer,
                (stick_x, stick_y),
                (stick_width, stick_height),
                handle_color,
                |internal_buffer, pos, _| {
                    let progress = if handle_center_y < hole_center_y {
                        (pos.1 as isize - stick_y) as f32 / stick_height as f32
                    } else {
                        1.0 - ((pos.1 as isize - stick_y) as f32
                            / stick_height as f32)
                    };
                    // println!(
                    //     "{} {} {}",
                    //     pos.1,
                    //     1.0 - ((pos.1 as isize - stick_y) as f32
                    //         / stick_height as f32),
                    //     0
                    // );

                    let new_color =
                        mirl::graphics::interpolate_color_rgb_u32_f32(
                            stick_color,
                            hole_color,
                            progress,
                        );
                    internal_buffer.set_pixel_unsafe(pos, new_color);
                },
            );
        }

        render::draw_rectangle::<true>(
            &buffer,
            handle_offset.0 as isize,
            handle_pos,
            handle_width as isize,
            handle_height as isize,
            handle_color,
        );

        (buffer, InsertionMode::ReplaceAll)
    }
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.height as isize
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.width as isize
    }
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        if info.focus_taken.is_focus_taken()
            && self.selected != info.container_id
        {
            return crate::GuiOutput::empty();
        }
        let Some(mouse_pos) = info.mouse_pos else {
            return crate::GuiOutput::empty();
        };

        let base_width = ((self.width as f32) * self.base_size) as usize;
        let base_height =
            self.height - get_formatting().vertical_margin * 2 - base_width;

        let handle_width =
            (base_width as f32 * self.handle_size_width) as usize;
        let handle_height =
            (handle_width as f32 * self.handle_size_height) as usize;

        let base_offset = get_center_position_of_object_for_object(
            base_width,
            base_height,
            self.width,
            self.height,
        );
        let handle_offset = get_center_position_of_object_for_object(
            handle_width,
            handle_height,
            self.width,
            self.height,
        );

        let margin = ((base_width as f32) * (1.0 - self.hole_size)) / 2.0;
        let hole_width = 2.0f32.mul_add(-margin, base_width as f32) as usize;
        let hole_height = 2.0f32.mul_add(-margin, base_height as f32) as usize;

        let hole_offset = get_center_position_of_object_for_object(
            hole_width,
            hole_height,
            base_width,
            base_height,
        );

        let hole_y = base_offset.1 + hole_offset.1;
        let hole_top = hole_y;
        let hole_bottom = hole_y + hole_height;
        let handle_half_height = handle_height / 2;

        let handle_min_pos = hole_top as isize - handle_half_height as isize;
        let handle_max_pos = hole_bottom as isize - handle_half_height as isize;

        let handle_pos = interpolate(
            handle_min_pos as f32,
            handle_max_pos as f32,
            self.elevation,
        ) as isize;

        let handle_collision: mirl::math::collision::Rectangle<_, true> =
            mirl::math::collision::Rectangle::new(
                handle_offset.0 as isize,
                handle_pos,
                handle_width as isize,
                handle_height as isize,
            );
        let collides = handle_collision.does_area_contain_point(mouse_pos);

        if (self.selected == info.container_id && info.mouse_info.left.down)
            || (info.mouse_info.left.clicked && collides)
        {
            self.selected = info.container_id;

            let target = if false {
                let bottom = handle_height as isize;
                let top = (handle_offset.1 + hole_offset.1) as isize;

                let adjusted_mouse_pos = mouse_pos.1 - bottom;
                adjusted_mouse_pos as f32 / top as f32
            } else if mouse_pos.1
                > (handle_half_height + handle_offset.1) as isize
            {
                1.0
                // return crate::GuiOutput::default(
                //     crate::FocusTaken::FunctionallyTaken,
                // )
                // .with_cursor(mirl::platform::CursorStyle::ArrowDown);
            } else {
                0.0
                // return crate::GuiOutput::default(
                //     crate::FocusTaken::FunctionallyTaken,
                // )
                // .with_cursor(mirl::platform::CursorStyle::ArrowUp);
            };

            let error_margin = 0.000_000_001;
            if (self.elevation - target).abs() > error_margin {
                self.elevation = target;
                self.needs_redraw.set(true);
            }
            return crate::GuiOutput::default(
                crate::FocusTaken::FunctionallyTaken,
            )
            .with_cursor(mirl::platform::CursorStyle::HandClosed);
        }
        self.selected = 0;

        if collides {
            return crate::GuiOutput::default(crate::FocusTaken::VisuallyTaken)
                .with_cursor(mirl::platform::CursorStyle::HandOpen);
        }

        crate::GuiOutput::empty()
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
