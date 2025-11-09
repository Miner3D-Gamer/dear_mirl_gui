use mirl::extensions::*;
use reflectable_derive::Inspectable;

use crate::modules::support::struct_editing::{Inspectable, InspectableType};

#[derive(Debug, Clone, Inspectable, Default, PartialEq)]
pub struct TestConfig {
    pub name: String,
    pub enabled: bool,
    pub count: i32,
    pub ratio: f64,
    pub nested: NestedSettings,
}

#[derive(Debug, Clone, Inspectable, Default, PartialEq)]
pub struct NestedSettings {
    //pub timeout_ms: u64,
    pub retry_count: u8,
    pub priority: f32,
}

// impl crate::DearMirlGuiModule for NestedSettings {
//     fn draw(
//         &mut self,
//         formatting: &crate::Formatting,
//         _info: &crate::ModuleDrawInfo,
//     ) -> (mirl::Buffer, crate::InsertionMode) {
//         (
//             mirl::Buffer::generate_fallback(
//                 self.get_size(formatting).tuple_into(),
//                 2,
//             ),
//             crate::InsertionMode::ReplaceAll,
//         )
//     }
//     fn get_height(&mut self, _formatting: &crate::Formatting) -> i32 {
//         50
//     }
//     fn get_width(&mut self, _formatting: &crate::Formatting) -> i32 {
//         50
//     }
//     fn update(
//         &mut self,
//         _inputs: &crate::ModuleUpdateInfo,
//     ) -> crate::GuiOutput {
//         crate::GuiOutput::empty()
//     }
//     fn need_redraw(&mut self) -> bool {
//         false
//     }
// }

pub fn create_test_struct() -> TestConfig {
    TestConfig {
        name: "MyConfig".to_string(),
        enabled: true,
        count: 42,
        ratio: std::f64::consts::PI,
        nested: NestedSettings {
            //timeout_ms: 5000,
            retry_count: 3,
            priority: 0.75,
        },
    }
}
