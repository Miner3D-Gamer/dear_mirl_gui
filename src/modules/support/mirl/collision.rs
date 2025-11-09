use mirl::extensions::FromPatch;

use crate::DearMirlGuiModule;

impl<
    T: std::marker::Send
        + std::fmt::Debug
        + 'static
        + num_traits::ToPrimitive
        + mirl::math::ConstTwoTillTen
        + std::ops::Mul<Output = T>
        + Copy,
    const CS: bool,
> DearMirlGuiModule for mirl::math::collision::Circle<T, CS>
where
    crate::DearMirlGuiCoordinateType: mirl::extensions::FromPatch<T>,
{
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (mirl::Buffer, crate::InsertionMode) {
        let buffer_size = self.radius.to_usize().unwrap_or_default() * 2;
        let circle_size = self.radius.to_isize().unwrap_or_default() * 2;
        let mut buffer = mirl::Buffer::new_empty((buffer_size, buffer_size));
        mirl::render::draw_circle::<true, false>(
            &mut buffer,
            (circle_size, circle_size),
            circle_size,
            mirl::graphics::color_presets::RED,
        );
        (buffer, crate::InsertionMode::ReplaceAll)
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        crate::DearMirlGuiCoordinateType::from_value(self.radius * T::TWO)
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        crate::DearMirlGuiCoordinateType::from_value(self.radius * T::TWO)
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
