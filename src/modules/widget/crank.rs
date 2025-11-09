use core::f32;

use mirl::extensions::*;
use mirl::{extensions::Tuple2Into, platform::CursorStyle};

use crate::{Buffer, DearMirlGuiModule, InsertionMode, render};

#[derive(Debug, Clone, PartialEq)]
/// A simple text module
pub struct Crank {
    /// Both width and height
    pub size: usize,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// How many rotations have been made
    pub rotations: isize,
    /// The current rotation
    pub rotation: f32,
    /// If the user is currently holding the handle
    pub cranking: usize,
    /// The circle in the middle
    pub crank_circle_size: f32,
    /// The handle you select
    pub crank_handle_size: f32,
    /// The line between the handle and the middle
    pub crank_connector_size: f32,
}
impl Crank {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(size: usize, rotations: isize, rotation: f32) -> Self {
        let crank_circle_size = 1.0 / 20.0;
        let crank_handle_size = 1.0 / 15.0;
        let crank_connector_size = 1.0 / 20.0;
        Self {
            needs_redraw: std::cell::Cell::new(true),
            rotation,
            rotations,
            size,
            cranking: 0,
            crank_circle_size,
            crank_handle_size,
            crank_connector_size,
        }
    }
    fn get_position(
        &self,
    ) -> (
        crate::DearMirlGuiCoordinateType,
        crate::DearMirlGuiCoordinateType,
        crate::DearMirlGuiCoordinateType,
    ) {
        let distance = 2.5;
        let offset = (self.size / 2) as crate::DearMirlGuiCoordinateType;
        let adjusted = (self.rotation * f32::consts::PI)
            .mul_add(2.0, -(f32::consts::PI / 2.0));
        let x = adjusted.cos() * (self.size as f32 / distance);
        let y = adjusted.sin() * (self.size as f32 / distance);
        let x = offset + x as crate::DearMirlGuiCoordinateType;
        let y = offset + y as crate::DearMirlGuiCoordinateType;
        (offset, x, y)
    }
}

impl DearMirlGuiModule for Crank {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        let crank_circle_width =
            (self.size as f32 * self.crank_circle_size) as usize;
        let crank_handle_width =
            (self.size as f32 * self.crank_handle_size) as isize;
        let crank_connector_width =
            (self.size as f32 * self.crank_connector_size) as isize;

        let mut buffer = Buffer::new_empty((self.size, self.size));
        //println!("{}", self.size / 2);
        let (offset, x, y) = self.get_position();
        let offset = offset as usize;
        render::draw_circle_outline_with_thickness::<true>(
            &mut buffer,
            offset,
            offset,
            offset / 4,
            formatting.foreground_color,
            crank_circle_width,
        );
        render::draw_circle::<true, false>(
            &mut buffer,
            (x, y).tuple_into(),
            crank_handle_width,
            formatting.foreground_color,
        );
        render::draw_line::<true>(
            &mut buffer,
            (offset, offset),
            (x, y).tuple_into(),
            formatting.foreground_color,
            crank_connector_width,
        );

        // let (offset, x, y) = self.get_position();
        // let mut collision: mirl::math::collision::Circle<_, true> =
        //     mirl::math::collision::Circle::new(x, y, crank_handle_width);

        // mirl::render::draw_buffer_on_buffer_1_to_1::<true, true, true>(
        //     &buffer,
        //     &collision.draw(formatting, info).0,
        //     (x, y),
        // );

        self.needs_redraw.set(true);
        (buffer, InsertionMode::Simple)
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.size as crate::DearMirlGuiCoordinateType
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.size as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        // self.rotation += 0.001;
        self.rotation %= 1.0;
        if info.focus_taken.is_focus_taken()
            && self.cranking != info.container_id
        {
            return crate::GuiOutput::empty();
        }
        let Some(mouse_pos) = info.mouse_pos else {
            return crate::GuiOutput::empty();
        };
        let (offset, x, y) = self.get_position();
        let collision: mirl::math::collision::Circle<i32, false> =
            mirl::math::collision::Circle::new(
                x as i32,
                y as i32,
                (self.size as f32 * self.crank_handle_size) as i32,
            );

        let mouse_collides = collision.does_area_contain_point(mouse_pos);

        if (self.cranking == info.container_id && info.mouse_info.left.down)
            || (info.mouse_info.left.clicked && mouse_collides)
        {
            self.cranking = info.container_id;
            let middle: mirl::math::collision::Circle<_, false> =
                mirl::math::collision::Circle::new(
                    offset as f32,
                    offset as f32,
                    1.0,
                );
            let closest: (f32, f32) = middle
                .get_closest_point_on_edge(mouse_pos.tuple_into())
                .sub((offset, offset).tuple_into());
            let angle = closest.1.atan2(closest.0);
            let prev_rotation = self.rotation;
            self.rotation = ((angle + f32::consts::PI / 2.0)
                / (f32::consts::PI * 2.0)
                + 1.0)
                % 1.0;

            if self.rotation - prev_rotation > 0.5 {
                self.rotations += -1;
            } else if self.rotation - prev_rotation < -0.5 {
                self.rotations -= -1;
            }

            return crate::GuiOutput {
                focus_taken: crate::FocusTaken::FunctionallyTaken,
                new_clipboard_data: None,
                new_cursor_position: None,
                new_cursor_style: Some(CursorStyle::HandClosed),
                hide_cursor: false,
                text_input_selected: false,
                request_clipboard_data: false,
            };
        }
        self.cranking = 0;
        if mouse_collides {
            return crate::GuiOutput {
                focus_taken: crate::FocusTaken::VisuallyTaken,
                new_clipboard_data: None,
                new_cursor_position: None,
                new_cursor_style: Some(CursorStyle::HandOpen),
                hide_cursor: false,
                text_input_selected: false,
                request_clipboard_data: false,
            };
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
