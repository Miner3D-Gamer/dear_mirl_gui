# Version 3.0.0:

- Updated to work with mirl 8.0.0
- Added `DearMirlGuiModule` to `dear_mirl_gui::WindowManager`
- Renamed `dear_mirl_gui::module_manager::get_module` to `dear_mirl_gui::module_manager::get_module_raw`
- Renamed `dear_mirl_gui::module_manager::get_module_mut` to `dear_mirl_gui::module_manager::get_module_raw_mut`
- Renamed `WindowManager` to `DearMirlGuiManager`
- The inputs of `WindowManager` and `DearMirlGuiManager` have been unified using a struct `GuiInputs`
- Removed `num-traits` and `indexmap` as dependencies
- Added number input module
- Added `prelude` file
- Removed a bunch of optional size inputs of `new` functions, instead using the given values from the globally set formatting
- Added `NumberInput` Module
- Changed the outputs of get_module_as(\_mut) to return `Result<T, ModuleReturnError>` instead of `ModuleReturnError` having a `AllGood` variant
- Added traits for path inline functions: Instead of retrieving the module manually, you can use the path itself to do simple tasks
- Renamed `Text` module to `TextDisplay`

# Version 2.2.0:

- Updated to work with mirl 7.1.0
- Added a few helper build functions for the TextInput module

---

# Version 2.1.0:

Updated to work with mirl 6.0.0

---

# Version 2.0.0:

Complete overhaul.

- Multiple window support
- Modules are no longer physically inside containers meaning nesting is a lot more viable
- More modules, smarter modules
- Better caching system
- Overhauled internal interactions

---

# Version 1.0.0:

Working proof of concept with a lot of limitations
