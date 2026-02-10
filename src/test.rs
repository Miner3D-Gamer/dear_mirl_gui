use mirl::prelude::*;

use crate::prelude::*;

#[allow(clippy::unwrap_used)]
#[allow(dead_code)]
fn actual_main() {
    let mut buffer = mirl::prelude::Buffer::new_empty((800, 600));
    let mut window = mirl::platform::minifb::Framework::new(
        "Rust Window",
        mirl::platform::WindowSettings::default(&buffer).center_window(),
    )
    .unwrap();
    let file_system = mirl::platform::file_system::FileSystem::new().unwrap();
    main_loop(&mut window, &file_system, &mut buffer);
}

#[cfg(feature = "experimental")]
pub use struct_editor_test::*;

#[allow(clippy::unwrap_used, clippy::float_cmp, clippy::too_many_lines)]
fn main_loop<
    F: mirl::platform::framework_traits::ExtendedWindowingFramework,
    D: mirl::platform::file_system::file_system_traits::FileSystemTrait,
>(
    window: &mut F,
    file_system: &D,
    buffer: &mut Buffer,
) {
    use mirl::prelude::*;

    mirl::enable_traceback();
    let font = mirl::platform::file_system::get_default_font(file_system)
        .unwrap()
        .to_font()
        .unwrap();

    set_formatting(Formatting::default(&font, 20));

    let mut ticker = mirl::misc::Ticker::new(30.0).unwrap();

    #[cfg(feature = "experimental")]
    let mut editable_struct = create_test_struct();

    #[cfg(feature = "experimental")]
    let struct_editor = register_module(
        // "struct_editor",
        modules::StructEditor::new(&editable_struct).unwrap(),
    )
    .with_name("Struct Editor".to_string());

    let m1 = register_module(
        //"module1",
        modules::TextDisplay::new("Hello World"),
    )
    .with_name("Text");
    let m2 = register_module(
        //"module2",
        modules::TextDisplay::new("there is text now ig"),
    )
    .with_name("Info");

    let slider = register_module(
        //"slider",
        modules::Slider::<f64, f64>::new(None, true, None).unwrap(),
    )
    .with_name("Slider");
    let progress_bar_up = register_module(
        // "progress_bar_up",
        modules::ProgressBar::new(None, false),
    )
    .with_name("Progress bar");
    let progress_bar_down = register_module(
        //"progress_bar_down",
        modules::ProgressBar::new(None, true),
    )
    .with_name("Vertical Progress bar");

    let button = register_module(
        // "button",
        modules::Button::new("Clickn't Me!".into()).with_interaction_function(
            |state| {
                if state.clicked {
                    println!("Oh no, I've been pressed!");
                }
            },
        ),
    )
    .with_name("Button");
    let button2 = register_module(
        //"button2",
        modules::Button::new(
            //"A Button".into(),
            "A Button with really really long text".into(),
        )
        .with_width(100),
    )
    .with_name("Button with long text");
    let checkbox1 = register_module(
        // "checkbox1",
        modules::CheckBox::new_3_state(20, "sample text".to_string()),
    )
    .with_name("3 state check box");
    let checkbox2 = register_module(
        // "checkbox2",
        modules::CheckBox::new_2_state(20, "bottom text".to_string()),
    )
    .with_name("2 state check box");
    // #[cfg(feature = "BAD_APPLE")]
    // add_module(
    //     "bad_apple".into(),
    //     CheckBox::new_3_state(100, "Evil Apple".to_string()),
    // );
    let divider = register_module(
        // "divider",
        modules::Separator::new(20, 300, false, None),
    )
    .with_name("Divider line");

    let selection = register_module(
        // "selection",
        modules::Selection::new(
            &[
                "Option 1a".into(),
                "Option 2a".into(),
                "Option 3a".into(),
                "Option 4a".into(),
            ],
            true,
            None,
        ),
    )
    .with_name("Selection");
    let same_line = register_module(
        //"anti_new_line",
        modules::SameLine::new(crate::DearMirlGuiCoordinateType::CONST_10),
    )
    .with_name("Anti new line module");
    let selection2 = register_module(
        // "selection2",
        modules::Selection::new(
            &[
                "Option 1b".into(),
                "Option 2b".into(),
                "Option 3b".into(),
                "Option 4b".into(),
            ],
            false,
            None,
        ),
    )
    .with_name("Other selection");
    let resetter = register_module(
        //"formatting",
        modules::ResetOffset::new(),
    );
    let text_input = register_module(
        //  "input",
        modules::TextInput::new(
            400,
            4,
            Some(Vec::from([
                "text.chars().take(i + 1).collect::<String>(),".into(),
                "Another creative line for thinky thinky".into(),
                "   Indent testing :/".into(),
            ])),
        )
        .with_placeholder_text("Click me to start writing"),
    );
    let crank = register_module(
        //"crank",
        modules::Crank::new(60, 0, 0.0),
    );
    let lever = register_module(
        //"lever",
        modules::Lever::new(40, 80),
    );
    let lever2 = register_module(
        //"lever2",
        modules::Lever::new(40, 80),
    );
    let lever3 = register_module(
        //"lever3",
        modules::Lever::new(40, 80),
    );
    let number_input = register_module(
        //"lever3",
        modules::NumberInput::new(10).with_width(100),
    );

    // let display =
    //     register_module("display", modules::NumberDisplay::new(0, 3, 20.0));

    let crank_info = register_module(
        //"crank_info",
        modules::TextDisplay::new("0"),
    );
    // let mut sub_window = DearMirlGui::<false, true>::new_simple(
    //     "Minesweeper",
    //     0,
    //     0,
    //     0,
    //     0,
    //     &font,
    // );
    // let rows = 5;
    // let columns = 2;

    // for c in 0..columns {
    //     for r in 0..rows {
    //         let idx = r + rows * c;
    //         println!("Hi {}", idx);
    //         let module = CheckBox::new_2_state(5, "".into());
    //         println!("Don't");
    //         sub_window.add_module(format!("column_{}", idx), module);
    //         println!("All good");
    //         sub_window
    //             .add_module(format!("formatting_{}", idx), SameLine::new(0));
    //         println!("Added formatter");
    //     }
    //     sub_window.add_module(format!("resetter_{}", c), ResetFormatting::new())
    // }

    // sub_window.set_size_to_see_all_modules();

    // add_module("minesweeper".into(), WindowEmulator::new(sub_window));

    // gui.set_size_to_see_all_modules();
    // gui2.set_size_to_see_all_modules();
    let mut window_manager: DearMirlGuiManager<false, true> =
        DearMirlGuiManager::new(Vec::from([
            DearMirlGui::new_simple(
                "Example Window",
                (20.into_value(), 10.into_value()),
                &[
                    m1.id(),
                    m2.id(),
                    button.id(),
                    button2.id(),
                    checkbox2.id(),
                    register_module(
                        //"sub_gui",
                        DearMirlGui::<false, true>::new_simple(
                            "Container",
                            (0.into_value(), 0.into_value()),
                            &Vec::from([text_input.id(), m1.id()]),
                        )
                        .collapsed(),
                    )
                    .id(),
                    checkbox1.id(),
                    //display.id(),
                    crank_info.id(),
                    crank.id(),
                    number_input.id(),
                    //text_input.id(),
                    #[cfg(feature = "experimental")]
                    struct_editor.id(),
                ],
            ),
            DearMirlGui::new_simple(
                "Another Window",
                (550.into_value(), 280.into_value()),
                &[
                    m1.id(),
                    checkbox1.id(),
                    selection.id(),
                    same_line.id(),
                    selection2.id(),
                    resetter.id(),
                    divider.id(),
                    button.id(),
                    button2.id(),
                    checkbox2.id(),
                    lever.id(),
                    same_line.id(),
                    lever2.id(),
                    same_line.id(),
                    lever3.id(),
                ],
            ),
        ]));

    let cursor_style_manager = window
        .load_custom_cursors(
            mirl::platform::mouse::CursorResolution::X32,
            mirl::graphics::rgb_to_u32(0, 255, 200),
            mirl::graphics::rgb_to_u32(0, 100, 100),
        )
        .unwrap();

    let mut fps_list = Vec::new();

    let mut slider_animation: f64 = 0.0;
    let mut request_clipboard_data = false;

    while window.is_open() {
        ticker.tick();
        let (delta_time, fps) = ticker.get_delta_time_and_fps_f64();
        buffer
            .clear_buffer_with_color(mirl::graphics::rgb_to_u32(110, 150, 140));

        // Set this to true if you wanna see how the gui handles casting modules to an incorrect type
        if false {
            get_module_as::<_, ()>(&slider, |_| {}).unwrap();
        }

        get_module_as_mut::<_, ()>(&progress_bar_up, |slider| {
            slider.progress += delta_time as f32 / 10.0;
            slider.progress = slider.progress.clamp(0.0, 1.0);
            if slider.progress == 1.0 {
                slider.progress = 0.0;
            }
        })
        .unwrap();
        get_module_as_mut::<_, ()>(&crank_info, |crank_info| {
            let value =
                get_module_as_mut::<_, isize>(&crank, |crank| crank.rotations)
                    .unwrap();
            crank_info.set_text(format!("Rotations: {value}"));
        })
        .unwrap();

        get_module_as_mut::<modules::Selection, ()>(&selection, |buttons| {
            if buttons.currently_selected[3] {
                buttons.currently_selected[3] = false;
                buttons.radio_buttons = !buttons.radio_buttons;
                buttons.needs_redraw.set(true);
            }
        })
        .unwrap();

        get_module_as_mut::<modules::ProgressBar, ()>(
            &progress_bar_down,
            |slider| {
                let exact = slider_animation.sin().midpoint(1.0);

                slider.progress = exact as f32;
                slider.progress = slider.progress.clamp(0.0, 1.0);
                if slider.progress == 0.0 {
                    slider.progress = 1.0;
                }
            },
        )
        .unwrap();
        #[cfg(feature = "experimental")]
        get_module_as_mut::<_, ()>(&struct_editor, |editor| {
            editor.sync(&mut editable_struct).unwrap();
            //println!("Hi");
        })
        .unwrap();

        let mouse_pos = window.get_mouse_position();

        slider_animation += delta_time * 2.0;

        let clipboard_data: Option<mirl::platform::file_system::FileData> = {
            if request_clipboard_data {
                println!("A module requested the current clipboard value");
            }
            None
        };
        let pressed_keys = window.get_all_keys_down();

        let gui_output = window_manager.update(
            &window.get_mouse_snapshot(),
            &pressed_keys,
            delta_time,
            &clipboard_data,
        );

        // let gui_output = gui.update(
        //     mouse_pos,
        //     mouse_scroll,
        //     window.is_mouse_down(mirl::platform::MouseButton::Left),
        //     window.is_mouse_down(mirl::platform::MouseButton::Middle),
        //     window.is_mouse_down(mirl::platform::MouseButton::Right),
        //     &pressed_keys,
        //     delta_time,
        //     &clipboard_data,
        // );

        // let gui_output = gui_output
        //     | gui2.update(
        //         mouse_pos,
        //         mouse_scroll,
        //         window.is_mouse_down(mirl::platform::MouseButton::Left),
        //         window.is_mouse_down(mirl::platform::MouseButton::Middle),
        //         window.is_mouse_down(mirl::platform::MouseButton::Right),
        //         &pressed_keys,
        //         delta_time,
        //         &clipboard_data,
        //     );
        // gui.draw_on_buffer(buffer);
        // gui2.draw_on_buffer(buffer);
        window_manager.draw_on_buffer(buffer);

        request_clipboard_data = gui_output.request_clipboard_data;

        if !gui_output.focus_taken {
            // Do stuff that uses mouse/keyboard
        }
        if let Some(data) = &gui_output.new_clipboard_data {
            println!(
                "New clipboard data has been provided: {}",
                data.to_printable()
            );
        }

        if let Some(_pos) = gui_output.new_cursor_position {
            // Not natively possible in mirl -> On the to do list
        }
        if let Some(_data) = gui_output.new_clipboard_data {
            // Not natively possible in mirl -> On the to do list
        }
        if gui_output.hide_cursor {
            // Not natively possible in mirl -> On the to do list
        }

        if let Some(mouse_pos) = mouse_pos
            && buffer
                .create_collision::<false, f32>((0.0, 0.0))
                .unwrap_or_default()
                .does_area_contain_point(mouse_pos)
        {
            window
                .set_cursor_style(
                    cursor_style_manager.from_cursor_style(
                        gui_output
                            .new_cursor_style
                            .unwrap_or(mirl::platform::CursorStyle::Default),
                    ),
                )
                .unwrap();
        }

        let average_fps: u64 = fps_list.average().unwrap_or_default();

        fps_list.push_or_replace_on_max_size(
            average_fps.max(1) as usize,
            fps as u64,
        );

        window.update(buffer).unwrap();
        window.set_title(&format!("Rust Window {average_fps} <- {fps}"));
    }
}
