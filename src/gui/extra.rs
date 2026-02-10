use std::{any::TypeId, cell::RefCell, rc::Rc};

use mirl::render::Buffer;

use crate::{AnyCasting, DearMirlGuiModule, module_manager::InsertionMode};

/// Type-erased container that preserves concrete type access with single storage
#[derive(Clone, Debug)]
pub struct ModuleContainer {
    /// Single shared storage
    pub item: Rc<RefCell<Box<dyn DearMirlGuiModule>>>,
    /// Type information for safe downcasting
    pub type_id: TypeId,
    /// The type name for debug purposes
    pub type_name: &'static str,
}

unsafe impl std::marker::Sync for ModuleContainer {}
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl std::marker::Send for ModuleContainer {}

impl ModuleContainer {
    /// Create a new module container
    pub fn new<T: DearMirlGuiModule + 'static + AnyCasting>(item: T) -> Self {
        Self {
            item: Rc::new(RefCell::new(Box::new(item))),
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>(),
        }
    }

    /// Safe downcasting to concrete type with closure (immutable)
    pub fn with_ref<R>(
        &self,
        f: impl FnOnce(&dyn DearMirlGuiModule) -> R,
    ) -> R {
        let borrow = self.item.borrow();
        f(borrow.as_ref())
    }

    /// Safe downcasting to concrete type with closure (mutable)
    pub fn with_ref_mut<R>(
        &self,
        f: impl FnOnce(&mut dyn DearMirlGuiModule) -> R,
    ) -> R {
        let mut borrow = self.item.borrow_mut();
        f(borrow.as_mut())
    }

    /// Get type information
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// Check if container holds a specific type
    #[must_use]
    pub const fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }

    /// See [crate::DearMirlGuiModule] for documentation
    pub fn apply_new_formatting(&mut self, formatting: &crate::Formatting) {
        self.with_ref_mut(|item| item.apply_new_formatting(formatting));
    }
    /// See [crate::DearMirlGuiModule] for documentation
    #[must_use]
    pub fn draw(
        &self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        self.with_ref_mut(|item| item.draw(formatting, info))
    }
    /// See [crate::DearMirlGuiModule] for documentation
    #[must_use]
    pub fn get_height(
        &self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.with_ref_mut(|item| item.get_height(formatting))
    }
    /// See [crate::DearMirlGuiModule] for documentation
    #[must_use]
    pub fn get_width(
        &self,
        formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.with_ref_mut(|item| item.get_width(formatting))
    }
    /// See [crate::DearMirlGuiModule] for documentation
    #[must_use]
    pub fn update(&self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        self.with_ref_mut(|item| item.update(info))
    }
    /// See [crate::DearMirlGuiModule] for documentation
    pub fn modify_offset_cursor(
        &self,
        modules: &[Self],
        used_idx: &Vec<usize>,
        formatting: &crate::Formatting,
        current: (
            &mut crate::DearMirlGuiCoordinateType,
            &mut crate::DearMirlGuiCoordinateType,
        ),
    ) {
        self.with_ref_mut(|item| {
            item.modify_offset_cursor(modules, used_idx, formatting, current);
        });
    }
    /// See [crate::DearMirlGuiModule] for documentation
    #[must_use]
    pub fn need_redraw(&self) -> bool {
        self.with_ref_mut(|item| item.need_redraw())
    }
    /// See [crate::DearMirlGuiModule] for documentation
    pub fn set_need_redraw(&self, redraw: Vec<(usize, bool)>) {
        self.with_ref_mut(|item| item.set_need_redraw(redraw));
    }
}
