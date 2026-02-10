use crate::{
    DearMirlGuiModule,
    modules::support::struct_editing::{
        DynSyncInspectable, Inspectable, InspectableType,
    },
    prelude::*,
};

impl InspectableType for bool {
    type Inspectable = InspectBool;
    fn new_from_value(value: Self) -> Option<Self::Inspectable> {
        Some(InspectBool {
            state: crate::modules::CheckBox::new_2_state(
                get_formatting().height,
                String::new(),
            )
            .with_state(usize::from(value)),
        })
    }
    // fn sync_from_value(&self, value: &mut Self::Inspectable) {
    //     value.state.checked = usize::from(*self);
    // }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct InspectBool {
    state: crate::modules::CheckBox,
}
impl Inspectable for InspectBool {
    fn get_fields_mut(&mut self) -> Vec<(&'static str, DynSyncInspectable)> {
        vec![]
    }
    fn get_name(&mut self) -> &'static str {
        "bool"
    }

    fn sync(
        &mut self,
        new: &DynSyncInspectable,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut val = new
            .lock()
            .map_err(|_| "Unable to lock".to_string())?
            .as_any_cloned();
        if let Some(inspect_bool) = val.downcast_mut::<Self>() {
            self.state.checked = inspect_bool.state.checked;
            self.set_need_redraw(Vec::from([(0, true)]));
        }

        Ok(())
    }

    //fn draw_ui(&mut self) {
    //        println!(" = {}", self);
    //    }
}
impl DearMirlGuiModule for InspectBool {
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (mirl::prelude::Buffer, crate::module_manager::InsertionMode) {
        self.state.draw(formatting, info)
    }

    fn get_height(
        &mut self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.state.get_height(formatting)
    }

    fn get_width(
        &mut self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.state.get_width(formatting)
    }

    fn update(&mut self, inputs: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        self.state.update(inputs)
    }

    fn need_redraw(&mut self) -> bool {
        self.state.need_redraw()
    }
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.state.set_need_redraw(need_redraw);
    }
    fn added(&mut self, container_id: usize) {
        self.state.added(container_id);
    }
    fn removed(&mut self, container_id: usize) {
        self.state.removed(container_id);
    }
    fn apply_new_formatting(&mut self, formatting: &crate::Formatting) {
        self.state.apply_new_formatting(formatting);
    }
    fn modify_offset_cursor(
        &mut self,
        modules: &[crate::gui::ModuleContainer],
        used_idx: &Vec<usize>,
        formatting: &crate::Formatting,
        current: (
            &mut crate::DearMirlGuiCoordinateType,
            &mut crate::DearMirlGuiCoordinateType,
        ),
    ) {
        self.state.modify_offset_cursor(modules, used_idx, formatting, current);
    }
}
