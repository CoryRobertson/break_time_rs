#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::num::NonZeroU32;
use std::path::PathBuf;
use std::rc::Rc;
use chrono::{DateTime, Duration, Local};
use native_dialog::MessageType;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Fullscreen, WindowBuilder};
use winit::window::CursorIcon;
use winit::window::WindowLevel;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::raw_window_handle::HasRawWindowHandle;
use winit::window::Window;


/// ProgramState represents what the program is doing related to rendering
/// TakingBreak(time) is for when the program should be rendering the break overlay
/// Working(time) is for when the program should not be overlaying anything
///
/// Date time inside represents the time that this break started
enum ProgramState {
    TakingBreak(DateTime<Local>),
    Working(DateTime<Local>),
    Paused,
}

const FULL_ALPHA: u32 = 0x00000000;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(init_window(&event_loop));
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let tray_menu = Menu::new();


    let about_menu_item = MenuItem::new("About", true, None);
    let quit_menu_item = MenuItem::new("Quit", true, None);


    tray_menu.append_items(&[&about_menu_item, &quit_menu_item]).unwrap();

    let about_menu_text = String::from(format!("{} v{}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION")));

    let mut tray_icon = Some(TrayIconBuilder::new().with_menu(Box::new(tray_menu))
        .with_tooltip("test tray menu tooltip")
        .with_icon(
            Icon::from_path(
                PathBuf::from(
                    "./assets/tray_ico.ico"),
                Some((128,128)))
                .unwrap()
        )
        .build()
        .unwrap());

    let mut last_redraw_time = Local::now();
    let mut state = ProgramState::Working(Local::now());

    let break_gap_time = Duration::seconds(5);
    let break_length_time = Duration::seconds(2);
    let redraw_rate = Duration::seconds(1);

    event_loop
        .run(move |event, elwt| {

            elwt.set_control_flow(ControlFlow::Poll);
            {
                let reciever = TrayIconEvent::receiver().try_recv();
                if let Ok(tray_event) = reciever {
                    println!("{:?}", tray_event);
                }
            }
            {
                let reciever = MenuEvent::receiver().try_recv();
                if let Ok(menu_event) = reciever {
                    if menu_event.id == about_menu_item.id() {
                        let about_dialogue = native_dialog::MessageDialog::new()
                            .set_title("About")
                            .set_text(&about_menu_text)
                            .set_type(MessageType::Info);
                        println!("about menu clicked");


                        about_dialogue.show_alert().unwrap();
                    }
                    if menu_event.id == quit_menu_item.id() {

                        window.set_visible(false);
                        // drop the tray icon so it goes away properly -> https://github.com/zkxs/simple-crosshair-overlay/blob/master/src/main.rs
                        tray_icon.take();
                        elwt.exit();

                        println!("quit menu clicked");

                    }

                    println!("{:?}", menu_event);
                }
            }

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    if let (Some(width), Some(height)) = {
                        let size = window.inner_size();
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    } {

                        // state switching logic
                        match &state {
                            ProgramState::TakingBreak(break_start_time) => {

                                let diff = Local::now().signed_duration_since(break_start_time);
                                #[cfg(debug_assertions)]
                                println!("taking break {}", diff);
                                if diff >= break_length_time {
                                    state = ProgramState::Working(Local::now());
                                }

                            }
                            ProgramState::Working(working_start_time) => {

                                let diff = Local::now().signed_duration_since(working_start_time);
                                #[cfg(debug_assertions)]
                                println!("taking working {}", diff);
                                if diff >= break_gap_time {
                                    state = ProgramState::TakingBreak(Local::now());
                                }
                            }
                            ProgramState::Paused => {
                                #[cfg(debug_assertions)]
                                println!("taking paused");
                            }
                        }

                        surface.resize(width, height).unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();

                        buffer.fill(FULL_ALPHA);

                        if let ProgramState::TakingBreak(_) = &state {
                            for y in 0..height.get() {
                                for x in 0..width.get() {
                                    if x > 1000 && x < 1200 {
                                        let red = x % 255;
                                        let green = y % 255;
                                        let blue = (x * y) % 255;
                                        let index = y as usize * width.get() as usize + x as usize;
                                        buffer[index] = blue | (green << 8) | (red << 16);
                                    }
                                }
                            }
                        }
                        buffer.present().unwrap();
                    }
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        },
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {
                    // only redraw every minute
                    if Local::now().signed_duration_since(last_redraw_time) >= redraw_rate {
                        window.request_redraw();
                        last_redraw_time = Local::now();
                        #[cfg(debug_assertions)]
                        println!("redrawn");
                    }
                }
            }
        })
        .unwrap();
}

// code stolen hopefully with permission, permission currently pending
fn init_window(event_loop: &EventLoop<()>) -> Window {
    let window_builder = WindowBuilder::new()
        .with_visible(false) // things get very buggy on Windows if you default the window to invisible...
        .with_transparent(true)
        .with_decorations(false)
        .with_resizable(false)
        .with_title("PROGRAM NAME")
        .with_position(PhysicalPosition::new(0, 0)) // can't determine monitor size until the window is created, so just use some dummy values
        .with_inner_size(PhysicalSize::new(1, 1)) // this might flicker so make it very tiny
        .with_active(false);

    #[cfg(target_os = "windows")] let window_builder = {
        use winit::platform::windows::WindowBuilderExtWindows;
        window_builder
            .with_drag_and_drop(false)
            .with_skip_taskbar(true)
    };

    let window = window_builder.build(event_loop)
        .unwrap();

    window.set_outer_position(PhysicalPosition::new(0,0));


    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    // window.request_inner_size(PhysicalSize::new(1920,1080));

    // once the window is ready, show it
    window.set_visible(true);

    // set these weirder settings AFTER the window is visible to avoid even more buggy Windows behavior
    // Windows particularly hates if you unset cursor_hittest while the window is hidden
    window.set_cursor_hittest(false).unwrap();
    window.set_window_level(WindowLevel::AlwaysOnTop);
    window.set_cursor_icon(CursorIcon::Crosshair); // Yo Dawg, I herd you like crosshairs so I put a crosshair in your crosshair so you can aim while you aim.

    window
}
