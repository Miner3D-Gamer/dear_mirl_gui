use crate::DearMirlGuiModule;

impl<
    T: std::marker::Send + std::fmt::Debug + 'static + num_traits::ToPrimitive,
    const CS: bool,
> DearMirlGuiModule for mirl::math::collision::Circle<T, CS>
{
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn draw(
        &mut self,
        _formatting: &crate::Formatting,
        _info: &crate::ModuleDrawInfo,
    ) -> (mirl::Buffer, crate::InsertionMode) {
        let buffer = mirl::Buffer::new_empty(
            self.radius.to_usize().unwrap_or_default() * 2,
            self.radius.to_usize().unwrap_or_default() * 2,
        );
        mirl::render::draw_circle::<true, false>(
            &buffer,
            self.radius.to_isize().unwrap_or_default(),
            self.radius.to_isize().unwrap_or_default(),
            self.radius.to_isize().unwrap_or_default(),
            mirl::graphics::color_presets::PURE_RED,
        );
        (buffer, crate::InsertionMode::ReplaceAll)
    }
    fn get_height(&self, _formatting: &crate::Formatting) -> isize {
        self.radius.to_isize().unwrap_or_default() * 2
    }
    fn get_width(&self, _formatting: &crate::Formatting) -> isize {
        self.radius.to_isize().unwrap_or_default() * 2
    }
    fn need_redraw(&self) -> bool {
        true
    }
    fn update(&mut self, _inputs: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        crate::GuiOutput::empty()
    }
}
