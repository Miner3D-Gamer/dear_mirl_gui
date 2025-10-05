use std::any::TypeId;

use crate::{AnyCasting, Buffer, DearMirlGuiModule, InsertionMode};
#[allow(clippy::type_complexity)]
/// A virtual `DearMirlGuiModule` - MAGIC!
#[derive(Debug, Clone, Copy)]
pub struct ModuleVTable {
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub draw: fn(
        &mut dyn DearMirlGuiModule,
        &crate::Formatting,
        &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode),
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub get_height: fn(&dyn DearMirlGuiModule, &crate::Formatting) -> isize,
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub get_width: fn(&dyn DearMirlGuiModule, &crate::Formatting) -> isize,
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub update: fn(
        &mut dyn DearMirlGuiModule,
        inputs: &crate::ModuleUpdateInfo,
    ) -> crate::GuiOutput,
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub need_redraw: fn(&mut dyn DearMirlGuiModule) -> bool,
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub modify_offset_cursor: fn(
        &mut dyn DearMirlGuiModule,
        &[ModuleContainer],
        &Vec<usize>,
        &crate::Formatting,
        (&mut isize, &mut isize),
    ),
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub apply_new_formatting:
        fn(&mut dyn DearMirlGuiModule, formatting: &crate::Formatting),
    /// See [`crate::DearMirlGuiModule`] for documentations
    pub set_need_redraw: fn(&mut dyn DearMirlGuiModule, Vec<(usize, bool)>),
}

impl ModuleVTable {
    /// See [`crate::DearMirlGuiModule`] for documentation
    #[must_use]
    pub fn new() -> Self {
        Self {
            draw: |item, formatting, info| item.draw(formatting, info),
            get_height: |item, formatting| item.get_height(formatting),
            get_width: |item, formatting| item.get_width(formatting),
            update: |item, inputs| item.update(inputs),
            need_redraw: |item| item.need_redraw(),
            modify_offset_cursor: |item,
                                   modules,
                                   current_idx,
                                   formatting,
                                   current| {
                item.modify_offset_cursor(
                    modules,
                    current_idx,
                    formatting,
                    current,
                );
            },
            apply_new_formatting: |item, modules| {
                item.apply_new_formatting(modules);
            },
            set_need_redraw: |item, redraw| {
                item.set_need_redraw(redraw);
            },
        }
    }
}
impl Default for ModuleVTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Type-erased container that preserves concrete type access with single storage
#[derive(Clone, Debug)]
pub struct ModuleContainer {
    /// Single shared storage
    pub item: std::rc::Rc<std::cell::RefCell<Box<dyn DearMirlGuiModule>>>,
    /// Virtual function table for trait methods
    pub vtable: ModuleVTable,
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
        let type_id = TypeId::of::<T>();
        let type_name = std::any::type_name::<T>();

        Self {
            item: std::rc::Rc::new(std::cell::RefCell::new(Box::new(item))),
            vtable: ModuleVTable::new(),
            type_id,
            type_name,
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
    pub fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }
}

impl ModuleContainer {
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub fn apply_new_formatting(&mut self, formatting: &crate::Formatting) {
        self.with_ref_mut(|item| item.apply_new_formatting(formatting));
    }
    #[must_use]
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub fn draw(
        &self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        self.with_ref_mut(|item| item.draw(formatting, info))
    }

    #[must_use]
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub fn get_height(&self, formatting: &crate::Formatting) -> isize {
        self.with_ref(|item| item.get_height(formatting))
    }

    #[must_use]
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub fn get_width(&self, formatting: &crate::Formatting) -> isize {
        self.with_ref(|item| item.get_width(formatting))
    }
    /// See [`crate::DearMirlGuiModule`] for documentation
    #[must_use]
    pub fn update(&self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        let mut borrowed = self.item.borrow_mut();
        (self.vtable.update)(&mut **borrowed, info)
    }
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub fn modify_offset_cursor(
        &self,
        modules: &[Self],
        used_idx: &Vec<usize>,
        formatting: &crate::Formatting,
        current: (&mut isize, &mut isize),
    ) {
        self.with_ref(|item| {
            item.modify_offset_cursor(modules, used_idx, formatting, current);
        });
    }
    /// See [`crate::DearMirlGuiModule`] for documentation
    #[must_use]
    pub fn need_redraw(&self) -> bool {
        self.with_ref(|item| item.need_redraw())
    }
    /// See [`crate::DearMirlGuiModule`] for documentation
    pub fn set_need_redraw(&self, redraw: Vec<(usize, bool)>) {
        self.with_ref(|item| item.set_need_redraw(redraw));
    }
}
