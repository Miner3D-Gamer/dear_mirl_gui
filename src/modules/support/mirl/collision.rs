use mirl::extensions::TryFromPatch;

use crate::DearMirlGuiModule;

impl<
    T: std::marker::Send
        + std::fmt::Debug
        + 'static
        + TryFromPatch<f32>
        + mirl::math::ConstNumbers128
        + std::ops::Mul<Output = T>
        + Copy,
    const CS: bool,
> DearMirlGuiModule for mirl::math::collision::Circle<T, CS>
where
    crate::DearMirlGuiCoordinateType: mirl::extensions::TryFromPatch<T>,
    usize: mirl::extensions::TryFromPatch<T>,
    isize: mirl::extensions::TryFromPatch<T>,
{
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (mirl::prelude::Buffer, crate::module_manager::InsertionMode) {
        let buffer_size =
            (usize::try_from_value(self.radius)).unwrap_or_default() * 2;
        let circle_size =
            isize::try_from_value(self.radius).unwrap_or_default() * 2;
        let mut buffer =
            mirl::prelude::Buffer::new_empty((buffer_size, buffer_size));
        mirl::render::draw_circle::<true, false>(
            &mut buffer,
            (circle_size, circle_size),
            circle_size,
            mirl::graphics::colors::RED,
        );
        (buffer, crate::module_manager::InsertionMode::ReplaceAll)
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        crate::DearMirlGuiCoordinateType::try_from_value(self.radius * T::CONST_2)
            .unwrap_or_default()
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        crate::DearMirlGuiCoordinateType::try_from_value(self.radius * T::CONST_2)
            .unwrap_or_default()
    }
    fn need_redraw(&mut self) -> bool {
        true
    }
    fn update(
        &mut self,
        _inputs: &crate::ModuleUpdateInfo,
    ) -> crate::GuiOutput {
        crate::GuiOutput::empty()
    }
}
