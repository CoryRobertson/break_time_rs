#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::BreakTimeApp;
use eframe::{NativeOptions, Theme};
use egui::{ViewportBuilder, WindowLevel};

const PROGRAM_NAME: &str = env!("CARGO_CRATE_NAME");
const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: switch to egui master branch cause it seems to work really good!

const FULL_ALPHA: u32 = 0x00000000;

mod app;
mod color_settings;
mod state;

mod constants;

fn main() {
    let native_options = NativeOptions {
        viewport: ViewportBuilder {
            title: Some(PROGRAM_NAME.to_string()),
            fullscreen: Some(true),
            maximized: Some(true),
            resizable: Some(false),
            transparent: Some(true),
            decorations: None,
            icon: None,
            active: Some(true),
            visible: Some(true),
            title_hidden: Some(true),
            titlebar_transparent: Some(true),
            drag_and_drop: None,
            close_button: None,
            minimize_button: None,
            window_level: WindowLevel::AlwaysOnTop,
            mouse_passthrough: Some(true),
            ..Default::default()
        },
        follow_system_theme: false,
        default_theme: Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        PROGRAM_NAME,
        native_options,
        Box::new(|cc| Box::new(BreakTimeApp::new(cc))),
    )
    .expect("Failed to run egui app");

    // let event_loop = EventLoop::new().unwrap();
    // let window = Rc::new(init_window(&event_loop));
    // let context = softbuffer::Context::new(window.clone()).unwrap();
    // let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    //
    // let tray_menu = Menu::new();
    //
    // let about_menu_item = MenuItem::new("About", true, None);
    // let quit_menu_item = MenuItem::new("Quit", true, None);
    //
    // tray_menu
    //     .append_items(&[&about_menu_item, &quit_menu_item])
    //     .unwrap();
    //
    // let about_menu_text = String::from(format!("{} v{}", PROGRAM_NAME, PROGRAM_VERSION));
    //
    // let mut tray_icon = Some(
    //     TrayIconBuilder::new()
    //         .with_menu(Box::new(tray_menu))
    //         .with_tooltip("test tray menu tooltip")
    //         .with_icon(
    //             Icon::from_path(PathBuf::from("./assets/tray_ico.ico"), Some((128, 128))).unwrap(),
    //         )
    //         .build()
    //         .unwrap(),
    // );
    //
    // let dq = device_query::device_state::DeviceState::new();
    //
    // // durations for program settings
    // let break_gap_time = Duration::seconds(5);
    // let break_length_time = Duration::seconds(2);
    // let redraw_rate = Duration::seconds(1);
    //
    // // shared state between loops for functionality
    // let mut state = ProgramState::Working(Local::now());
    // let mut previous_mouse_pos = dq.get_mouse().coords;
    // let mut last_mouse_moved_time = Local::now();
    // let mut last_key_pressed_time = Local::now();
    // let mut last_redraw_time = Local::now();
    //
    // event_loop
    //     .run(move |event, elwt| {
    //         // boolean for if the mouse was moved during the duration of the break time
    //         let mouse_moved = {
    //             let dist = {
    //                 let p1 = dq.get_mouse().coords;
    //                 let p2 = previous_mouse_pos.clone();
    //
    //                 ((p2.1 - p1.1).pow(2) as f32 + (p2.0 - p1.0).pow(2) as f32).sqrt()
    //             };
    //
    //             let significant_dist = dist > 1.0;
    //
    //             if significant_dist {
    //                 last_mouse_moved_time = Local::now();
    //             }
    //
    //             Local::now().signed_duration_since(&last_mouse_moved_time) < break_length_time
    //         };
    //
    //         // boolean for if there are any pressed keys detected
    //         let keys_pressed = {
    //             if !dq.get_keys().is_empty() {
    //                 last_key_pressed_time = Local::now();
    //             }
    //
    //             Local::now().signed_duration_since(&last_key_pressed_time) < break_length_time
    //         };
    //
    //         // boolean representing if there was any detected activity from the user
    //         let user_activity_happened = keys_pressed || mouse_moved;
    //
    //         elwt.set_control_flow(ControlFlow::Poll);
    //
    //         // tray event handler block
    //         {
    //             let receiver = TrayIconEvent::receiver().try_recv();
    //             if let Ok(tray_event) = receiver {
    //                 println!("{:?}", tray_event);
    //             }
    //         }
    //
    //         // tray menu event handler block
    //         {
    //             let receiver = MenuEvent::receiver().try_recv();
    //             if let Ok(menu_event) = receiver {
    //                 if menu_event.id == about_menu_item.id() {
    //                     let about_dialogue = native_dialog::MessageDialog::new()
    //                         .set_title("About")
    //                         .set_text(&about_menu_text)
    //                         .set_type(MessageType::Info);
    //                     #[cfg(debug_assertions)]
    //                     println!("about menu clicked");
    //
    //                     about_dialogue.show_alert().unwrap();
    //                 }
    //                 if menu_event.id == quit_menu_item.id() {
    //                     window.set_visible(false);
    //                     // drop the tray icon so it goes away properly -> https://github.com/zkxs/simple-crosshair-overlay/blob/master/src/main.rs
    //                     tray_icon.take();
    //                     elwt.exit();
    //                     #[cfg(debug_assertions)]
    //                     println!("quit menu clicked");
    //                 }
    //                 #[cfg(debug_assertions)]
    //                 println!("{:?}", menu_event);
    //             }
    //         }
    //
    //         match event {
    //             Event::WindowEvent {
    //                 window_id,
    //                 event: WindowEvent::RedrawRequested,
    //             } if window_id == window.id() => {
    //                 if let (Some(width), Some(height)) = {
    //                     let size = window.inner_size();
    //                     (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
    //                 } {
    //                     // state switching logic
    //                     match &state {
    //                         ProgramState::TakingBreak(break_start_time) => {
    //                             let diff = Local::now().signed_duration_since(break_start_time);
    //                             #[cfg(debug_assertions)]
    //                             println!("taking break {}", diff);
    //                             if diff >= break_length_time {
    //                                 state = ProgramState::Working(Local::now());
    //                             }
    //
    //                             if user_activity_happened {
    //                                 #[cfg(debug_assertions)]
    //                                 println!("user activity happened, resetting break time...");
    //                                 state = ProgramState::TakingBreak(Local::now());
    //                             }
    //                         }
    //                         ProgramState::Working(working_start_time) => {
    //                             let diff = Local::now().signed_duration_since(working_start_time);
    //                             #[cfg(debug_assertions)]
    //                             println!("taking working {}", diff);
    //                             if diff >= break_gap_time {
    //                                 state = ProgramState::TakingBreak(Local::now());
    //                             }
    //                         }
    //                         ProgramState::Paused => {
    //                             #[cfg(debug_assertions)]
    //                             println!("taking paused");
    //                         }
    //                     }
    //
    //                     surface.resize(width, height).unwrap();
    //
    //                     let mut buffer = surface.buffer_mut().unwrap();
    //
    //                     buffer.fill(FULL_ALPHA);
    //
    //                     if let ProgramState::TakingBreak(_) = &state {
    //                         for y in 0..height.get() {
    //                             for x in 0..width.get() {
    //                                 if x > 1000 && x < 1200 {
    //                                     let red = x % 255;
    //                                     let green = y % 255;
    //                                     let blue = (x * y) % 255;
    //                                     let index = y as usize * width.get() as usize + x as usize;
    //                                     buffer[index] = blue | (green << 8) | (red << 16);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     buffer.present().unwrap();
    //                 }
    //             }
    //             Event::WindowEvent {
    //                 event:
    //                     WindowEvent::CloseRequested
    //                     | WindowEvent::KeyboardInput {
    //                         event:
    //                             KeyEvent {
    //                                 logical_key: Key::Named(NamedKey::Escape),
    //                                 ..
    //                             },
    //                         ..
    //                     },
    //                 window_id,
    //             } if window_id == window.id() => {
    //                 elwt.exit();
    //             }
    //             _ => {
    //                 // only redraw every minute
    //                 if Local::now().signed_duration_since(last_redraw_time) >= redraw_rate {
    //                     window.request_redraw();
    //                     last_redraw_time = Local::now();
    //                     #[cfg(debug_assertions)]
    //                     println!("redrawn");
    //                 }
    //             }
    //         }
    //
    //         previous_mouse_pos = dq.get_mouse().coords;
    //     })
    //     .unwrap();
}
