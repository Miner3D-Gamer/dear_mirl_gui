use mirl::render::draw_buffer_on_buffer_1_to_1;

use crate::{
    Buffer, DearMirlGuiModule, InsertionMode, WhatAmI, get_formatting,
    modules::support::struct_editing::{
        DynSyncInspectable, Inspectable, draw_inspectable,
        get_size_of_inspectable, update_inspectable,
    },
};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone)]
/// A module with which you can visually edit structs
pub struct StructEditor {
    /// If the held struct was edited
    pub struct_edited_by_editor: bool,
    /// The struct currently held
    pub storage: Option<DynSyncInspectable>,
    /// A copy that detects if the external struct has been edited
    pub compare: Option<DynSyncInspectable>,
    #[allow(missing_docs)]
    pub needs_redraw: std::cell::Cell<bool>,
    /// The cached size of the struct
    pub size: (isize, isize),
}
impl StructEditor {
    #[allow(missing_docs)]
    /// # Errors
    /// When lock was poisoned
    pub fn new<T: Inspectable + Clone + 'static>(
        item: &T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            storage: None,
            compare: None,
            needs_redraw: std::cell::Cell::new(true),
            struct_edited_by_editor: false,
            size: get_size_of_inspectable(
                &std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                    item.clone(),
                ))),
                &get_formatting(),
                None,
            )?,
        })
    }
    /// Sync the given struct with the struct in the local storage, local edits are prioritized
    ///
    /// When to call this function:
    /// - After the module has been updated
    /// - Before your code edits the struct
    ///
    /// # Errors
    /// When the given struct and the struct in storage mismatch it errors when the local struct has been edited
    /// When the MutexGuard lock has been poisoned
    pub fn sync<T: Inspectable + Clone + 'static + InspectableType>(
        &mut self,
        item: &mut T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.struct_edited_by_editor {
            if let Some(s) = &self.storage {
                self.struct_edited_by_editor = false;

                // Acquire an *owned* Box<dyn Any> while the lock is held,
                // then drop the guard and downcast the owned box into Box<T>.
                let any_box: Box<dyn std::any::Any> = {
                    let guard = s
                        .lock()
                        .map_err(|_| "Unable to access local storage")?;
                    guard.as_any_cloned()
                };

                let boxed_t = any_box.downcast::<T>().map_err(|x| {
                    self.storage = None;
                    format!(
                        "Type mismatch in storage.\nIn Storage: {}\nGiven: {}",
                        x.what_am_i(),
                        item.clone().what_am_i()
                    )
                })?;
                *item = *boxed_t;
            }
        } else {
            let t: DynSyncInspectable = std::sync::Arc::new(
                std::sync::Mutex::new(Box::new(item.clone())),
            );
            if let Some(s) = &self.storage
                && let Some(c) = &self.compare
            {
                //println!("Before locking");
                let mut storage_contents =
                    s.lock().map_err(|_| "Unable to access local storage")?;
                //println!("Lock once");
                let compare_contents =
                    c.lock().map_err(|_| "Unable to access local compare")?;
                //println!("Got here");
                if !compare_contents.is_same(item) {
                    //println!("Update!");
                    storage_contents.sync(&t)?;
                    //println!("Done");
                }
            } else {
                self.size =
                    get_size_of_inspectable(&t, &get_formatting(), None)?;

                // store an owned boxed trait object
                self.storage = Some(t);
                self.compare = Some(std::sync::Arc::new(
                    std::sync::Mutex::new(Box::new(item.clone())),
                ));
            }
        }
        Ok(())
    }
}
use crate::modules::support::struct_editing::InspectableType;

impl DearMirlGuiModule for StructEditor {
    fn apply_new_formatting(&mut self, _formatting: &crate::Formatting) {}
    fn set_need_redraw(&mut self, need_redraw: Vec<(usize, bool)>) {
        self.needs_redraw.set(super::misc::determine_need_redraw(need_redraw));
    }
    fn draw(
        &mut self,
        formatting: &crate::Formatting,
        info: &crate::ModuleDrawInfo,
    ) -> (Buffer, InsertionMode) {
        let mut buffer = Buffer::new_empty((
            self.get_width(formatting) as usize,
            self.get_height(formatting) as usize,
        ));
        if let Some(store) = &mut self.storage {
            match draw_inspectable(
                store,
                0,
                formatting,
                Some(self.size),
                None,
                info,
            ) {
                Err(error) => println!("ERROR: {error}"),
                Ok(img) => {
                    draw_buffer_on_buffer_1_to_1::<true, false, false, false>(
                        &mut buffer,
                        &img,
                        (0, 0),
                    );
                    //println!("{:?}", (img.width, img.height));
                    // render::draw_text::<true>(
                    //     &mut buffer,
                    //     "test",
                    //     (0, 0),
                    //     mirl::graphics::colors::RED,
                    //     20.0,
                    //     &formatting.font,
                    // );
                }
            }
        }

        (buffer, InsertionMode::ReplaceAll)
    }
    fn get_height(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.size.1 as crate::DearMirlGuiCoordinateType
    }
    fn get_width(
        &mut self,
        _formatting: &crate::Formatting,
    ) -> crate::DearMirlGuiCoordinateType {
        self.size.0 as crate::DearMirlGuiCoordinateType
    }
    fn update(&mut self, info: &crate::ModuleUpdateInfo) -> crate::GuiOutput {
        if let Some(store) = &mut self.storage {
            match update_inspectable(
                store,
                0,
                &get_formatting(),
                Some(self.size),
                None,
                info,
            ) {
                Err(error) => {
                    println!("ERROR: {error}");
                    crate::GuiOutput::empty()
                }
                Ok(img) => img,
            }
        } else {
            crate::GuiOutput::empty()
        }
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
