use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use crate::color_settings::{get_overlay_color, get_overlay_text_color};
use crate::constants::OVERLAY_RADIUS;
use crate::state::ProgramState;
use chrono::{DateTime, Duration, Local};
use device_query::{DeviceQuery, DeviceState, MousePosition};
use eframe::{CreationContext, Frame};
use egui::epaint::Shadow;
use egui::{Align2, Color32, Context, FontFamily, FontId, ImageSource, Pos2, Stroke, Style, Ui, Visuals};
use std::thread::sleep;
use eframe::emath::Rect;
use egui::load::Bytes;

pub struct BreakTimeApp {
    showing_options_menu: bool,
    program_state: ProgramState,
    device_query: DeviceState,
    break_gap_time: Duration,
    break_length_time: Duration,
    redraw_rate: Duration, // unused as of now
    last_key_pressed_time: DateTime<Local>,
    last_mouse_moved_time: DateTime<Local>,
    last_redraw_time: DateTime<Local>,
    previous_mouse_pos: MousePosition,
}

/// The number which we divide the break length when checking for activity,
/// we consider activity to be occuring if its `last activity time` < `break_length_time` / `BREAK_ACTIVITY_DIVISOR`
const BREAK_ACTIVITY_DIVISOR: i32 = 4;

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

impl BreakTimeApp {
    pub fn new(cc: &CreationContext) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for BreakTimeApp {
    fn default() -> Self {
        Self {
            showing_options_menu: false,
            program_state: ProgramState::Working(Local::now()),
            device_query: DeviceState::new(),
            break_gap_time: Duration::seconds(5),
            break_length_time: Duration::seconds(2),
            redraw_rate: Duration::seconds(1),
            last_key_pressed_time: Local::now(),
            last_mouse_moved_time: Local::now(),
            last_redraw_time: Local::now(),
            previous_mouse_pos: MousePosition::default(),
        }
    }
}

impl eframe::App for BreakTimeApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        egui_extras::install_image_loaders(ctx);

        let clear_visuals = Visuals {
            dark_mode: false,
            widgets: Default::default(),
            selection: Default::default(),
            hyperlink_color: Default::default(),
            faint_bg_color: Default::default(),
            extreme_bg_color: Default::default(),
            code_bg_color: Default::default(),
            warn_fg_color: Default::default(),
            error_fg_color: Default::default(),
            window_rounding: Default::default(),
            window_shadow: Shadow {
                extrusion: 0.0,
                color: Color32::TRANSPARENT,
            },
            window_fill: Color32::TRANSPARENT,
            window_stroke: Stroke {
                width: 0.0,
                color: Color32::TRANSPARENT,
            },
            menu_rounding: Default::default(),
            panel_fill: Color32::TRANSPARENT,
            popup_shadow: Shadow {
                extrusion: 0.0,
                color: Color32::TRANSPARENT,
            },
            collapsing_header_frame: false,
            ..Default::default()
        };

        let clear_style = Style {
            spacing: Default::default(),
            interaction: Default::default(),
            visuals: clear_visuals.clone(),
            animation_time: 0.0,
            debug: Default::default(),
            explanation_tooltips: false,
            ..Default::default()
        };

        let opaque_visuals = Visuals {
            dark_mode: false,
            widgets: Default::default(),
            selection: Default::default(),
            hyperlink_color: Default::default(),
            faint_bg_color: Default::default(),
            extreme_bg_color: Default::default(),
            code_bg_color: Default::default(),
            warn_fg_color: Default::default(),
            error_fg_color: Default::default(),
            window_rounding: Default::default(),
            window_shadow: Shadow {
                extrusion: 0.0,
                color: Color32::from_rgba_premultiplied(30, 30, 30, 200),
            },
            window_fill: Color32::from_rgba_premultiplied(30, 30, 30, 100),
            window_stroke: Stroke {
                width: 0.0,
                color: Color32::from_rgba_premultiplied(80, 80, 80, 150),
            },
            menu_rounding: Default::default(),
            panel_fill: Color32::from_rgba_premultiplied(60, 60, 60, 100),
            popup_shadow: Shadow {
                extrusion: 0.0,
                color: Color32::from_rgba_premultiplied(10, 10, 10, 100),
            },
            collapsing_header_frame: false,
            ..Default::default()
        };

        let opaque_style = Style {
            spacing: Default::default(),
            interaction: Default::default(),
            visuals: opaque_visuals.clone(),
            animation_time: 0.0,
            debug: Default::default(),
            explanation_tooltips: false,
            ..Default::default()
        };

        let mouse_moved = {
            let dist = {
                let p1 = self.device_query.get_mouse().coords;
                let p2 = self.previous_mouse_pos.clone();

                ((p2.1 - p1.1).pow(2) as f32 + (p2.0 - p1.0).pow(2) as f32).sqrt()
            };

            let significant_dist = dist > 1.0;

            if significant_dist {
                self.last_mouse_moved_time = Local::now();
            }

            Local::now().signed_duration_since(&self.last_mouse_moved_time)
                < self.break_length_time / BREAK_ACTIVITY_DIVISOR
        };

        // boolean for if there are any pressed keys detected
        let keys_pressed = {
            if !self.device_query.get_keys().is_empty() {
                self.last_key_pressed_time = Local::now();
            }

            Local::now().signed_duration_since(&self.last_key_pressed_time)
                < self.break_length_time / BREAK_ACTIVITY_DIVISOR
        };

        // boolean representing if there was any detected activity from the user
        let user_activity_happened = keys_pressed || mouse_moved;

        // ctx.request_repaint_after(std::time::Duration::from_millis(self.redraw_rate.num_milliseconds() as u64 * 100));

        // sleep(core::time::Duration::from_millis(100));

        ctx.request_repaint();

        ctx.set_style(clear_style.clone());

        self.clear_color(&clear_visuals);

        // state switching logic
        match &self.program_state {
            ProgramState::TakingBreak(break_start_time) => {
                let diff = Local::now().signed_duration_since(break_start_time);
                #[cfg(debug_assertions)]
                println!("taking break {}", diff);
                if diff >= self.break_length_time {
                    self.program_state = ProgramState::Working(Local::now());
                }

                if user_activity_happened {
                    #[cfg(debug_assertions)]
                    println!("user activity happened, resetting break time...");
                    self.program_state = ProgramState::TakingBreak(Local::now());
                }

                ctx.set_style(opaque_style.clone());

                egui::CentralPanel::default().show(&ctx, |ui| {
                    // overlay for when a break is occurring
                    // let (width, height) = (
                    //     ui.ctx().screen_rect().width(),
                    //     ui.ctx().screen_rect().height(),
                    // );
                    // ui.painter().text(
                    //     Pos2::new(width / 2.0, height / 4.0),
                    //     Align2::CENTER_CENTER,
                    //     "TAKE A BREAK",
                    //     FontId::new(60.0, FontFamily::Proportional),
                    //     get_overlay_text_color(),
                    // );
                    // ui.painter().circle_filled(
                    //     Pos2::new(width / 2.0, height / 2.0),
                    //     OVERLAY_RADIUS,
                    //     get_overlay_color(),
                    // );

                    // let the user specify their own overlay if they want to
                    if let Ok(mut file) = File::open("./overlay.png") {
                        let img = ImageSource::Bytes {
                            uri: Cow::from("bytes://overlay.png"),
                            bytes: {
                                let mut v = vec![];
                                file.read_to_end(&mut v).expect("TODO: panic message");
                                Bytes::from(v)
                            }
                        };
                        ui.image(img);
                    } else {
                        ui.image(egui::include_image!("../assets/overlay.png"));
                    }
                });
            }
            ProgramState::Working(working_start_time) => {
                let diff = Local::now().signed_duration_since(working_start_time);
                #[cfg(debug_assertions)]
                println!("taking working {}", diff);
                if diff >= self.break_gap_time {
                    self.program_state = ProgramState::TakingBreak(Local::now());
                }
            }
            ProgramState::Paused => {
                #[cfg(debug_assertions)]
                println!("taking paused");
            }
        }

        // egui::TopBottomPanel::top("top pannel!").show(&ctx, |ui| {
        //     ui.heading("this is text!!");
        // });

        self.previous_mouse_pos = self.device_query.get_mouse().coords;
    }
}
