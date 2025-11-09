use crate::{
    DearMirlGuiModule, get_formatting,
    modules::support::struct_editing::{
        DynSyncInspectable, Inspectable, InspectableType,
    },
};

impl InspectableType for String {
    type Inspectable = InspectString;
    fn new_from_value(value: Self) -> Self::Inspectable {
        InspectString {
            state: crate::modules::TextInput::new(
                get_formatting().height,
                get_formatting().height * 3,
                1,
                Some(crate::modules::text_input::ready_text(&value, 4)),
            ),
        }
    }
    // fn sync_from_value(&self, value: &mut Self::Inspectable) {
    //     value.state.text = crate::modules::text_input::ready_text(self, 4);
    // }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InspectString {
    state: crate::modules::TextInput,
}
impl Inspectable for InspectString {
    fn get_fields_mut(&mut self) -> Vec<(&'static str, DynSyncInspectable)> {
        vec![]
    }
    fn get_name(&mut self) -> &'static str {
        "String"
    }
    fn sync(
        &mut self,
        new: &DynSyncInspectable,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut val = new
            .lock()
            .map_err(|_| "Unable to lock".to_string())?
            .as_any_cloned();
        if let Some(inspect) = val.downcast_mut::<Self>() {
            let own_fields = self.get_fields_mut();
            let other_fields = inspect.get_fields_mut();
            if own_fields.len() != other_fields.len() {
                return Err("Fields do not match".into());
            }
            if own_fields.is_empty() {
                *self = inspect.clone();
            } else {
                for i in 0..own_fields.len() {
                    let own = &own_fields[i].1;
                    let other = &other_fields[i].1;
                    let mut val = own
                        .lock()
                        .map_err(|_| "Unable to lock sub".to_string())?;
                    val.sync(other)?;
                }
            }
        }

        Ok(())
    }
    // fn sync(
    //     &mut self,
    //     new: &DynSyncInspectable,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     let val = new.lock().map_err(|_| "Unable to lock".to_string())?;
    //     if let Some(inspect) = val.as_any_cloned().downcast_mut::<Self>() {
    //         self.state.text = inspect.state.text.clone();
    //         self.state.caret = inspect.state.caret.clone();
    //         self.state.camera = inspect.state.camera;
    //         self.set_need_redraw(Vec::from([(0, true)]));
    //     }

    //     Ok(())
    // }

    // fn draw_ui(
    //     &mut self,
    //     formatting: &crate::Formatting,
    //     info: &crate::ModuleDrawInfo,
    // ) -> mirl::Buffer {
    //     self.state.draw(formatting, info).0
    // }

    //fn draw_ui(&mut self) {
    //        println!(" = {}", self);
    //    }
}
impl DearMirlGuiModule for InspectString {
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (mirl::Buffer, crate::InsertionMode) {
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
