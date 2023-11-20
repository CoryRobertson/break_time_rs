use std::num::NonZeroU32;
use std::rc::Rc;
use chrono::{Local, Timelike};
use winit::event::{Event, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Fullscreen, WindowBuilder};
use winit::window::CursorIcon;
use winit::window::WindowLevel;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::window::Window;


fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(init_window(&event_loop));
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let open_time = Local::now();
    let mut last_redraw_time = Local::now();

    const FULL_ALPHA: u32 = 0x00000000;

    event_loop
        .run(move |event, elwt| {

            let time_now = Local::now();



            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    if let (Some(width), Some(height)) = {
                        let size = window.inner_size();
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    } {
                        surface.resize(width, height).unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();

                        buffer.fill(FULL_ALPHA);

                        if time_now.minute() > open_time.minute() {
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
                    if Local::now().signed_duration_since(last_redraw_time).num_minutes() >= 1 {
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
            .with_skip_taskbar(false)
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
