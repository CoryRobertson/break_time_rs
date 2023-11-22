use crate::app::options_screen::show_options_menu;
use crate::state::ProgramState;
use chrono::{DateTime, Duration, Local};
use device_query::{DeviceQuery, DeviceState, Keycode, MousePosition};
use eframe::{CreationContext, Frame, App};
use egui::epaint::Shadow;
use egui::load::Bytes;
use egui::Pos2;
use egui::{Color32, Context, Id, ImageSource, Stroke, Style, ViewportId, Visuals};
use egui::ViewportBuilder;
use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct BreakTimeApp {
    #[serde(skip)]
    showing_options_menu: bool,
    #[serde(skip)]
    program_state: ProgramState,
    #[serde(skip)]
    device_query: DeviceState,
    
    break_gap_time_minutes: f32,
    break_length_time_minutes: f32,

    #[serde(skip)]
    break_gap_time: Duration,
    #[serde(skip)]
    break_length_time: Duration,

    #[serde(skip)]
    redraw_rate: Duration, // unused as of now

    #[serde(skip)]
    last_key_pressed_time: DateTime<Local>,
    #[serde(skip)]
    last_mouse_moved_time: DateTime<Local>,
    #[serde(skip)]
    last_redraw_time: DateTime<Local>,
    #[serde(skip)]
    previous_mouse_pos: MousePosition,
}

mod options_screen;

/// The number which we divide the break length when checking for activity,
/// we consider activity to be occurring if its `last activity time` < `break_length_time` / `BREAK_ACTIVITY_DIVISOR`
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
// let mut previous_mouse_pos = dq.get_mouse().cords;
// let mut last_mouse_moved_time = Local::now();
// let mut last_key_pressed_time = Local::now();
// let mut last_redraw_time = Local::now();

impl BreakTimeApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {

        if let Some(storage) = cc.storage {
            println!("aaaa {}", eframe::get_value::<Self>(storage, eframe::APP_KEY).unwrap().break_gap_time_minutes);
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }


        Self::default()
    }
}

impl Default for BreakTimeApp {
    fn default() -> Self {
        Self {

            showing_options_menu: false,

            program_state: ProgramState::Working(Local::now()),

            device_query: DeviceState::new(),
            break_gap_time_minutes: 55.0,
            break_length_time_minutes: 5.0,
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

        // TODO: get rid of this unwrap
        self.break_gap_time = Duration::from_std(std::time::Duration::from_secs_f32(self.break_gap_time_minutes * 60.0)).unwrap();
        self.break_length_time = Duration::from_std(std::time::Duration::from_secs_f32(self.break_length_time_minutes * 60.0)).unwrap();

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
        let (keys_pressed, _keys) = {
            let keys = self.device_query.get_keys();
            if !keys.is_empty() {
                self.last_key_pressed_time = Local::now();
                if keys.contains(&Keycode::Slash) && keys.contains(&Keycode::BackSlash) {
                    self.showing_options_menu = true;
                }
            }
            (
                Local::now().signed_duration_since(&self.last_key_pressed_time)
                    < self.break_length_time / BREAK_ACTIVITY_DIVISOR,
                keys,
            )
        };

        // boolean representing if there was any detected activity from the user
        let user_activity_happened = keys_pressed || mouse_moved;

        // ctx.request_repaint_after(std::time::Duration::from_millis(self.redraw_rate.num_milliseconds() as u64 * 100));

        // sleep(core::time::Duration::from_millis(100));

        if self.showing_options_menu {
            ctx.set_visuals(Visuals::default());

            if let Some(storage) = frame.storage_mut() {
                self.save(storage);
            }

            ctx.show_viewport_immediate(
                ViewportId(Id::from(1234.to_string())),
                ViewportBuilder::default()
                    .with_always_on_top()
                    .with_active(true)
                    .with_decorations(false)
                    .with_visible(true)
                    .with_position(Pos2::new(0.0, 0.0))
                    .with_title("Options")
                    .with_transparent(false),
                |a, b| {
                    egui::CentralPanel::default().show(a, |ui| {
                        show_options_menu(ui, ctx, self,frame);
                    });
                },
            );
        }

        ctx.request_repaint();

        ctx.set_style(clear_style.clone());

        self.clear_color(&clear_visuals);

        // state switching logic
        match &self.program_state {
            ProgramState::TakingBreak(break_start_time) => {
                let diff = Local::now().signed_duration_since(break_start_time);

                if diff >= self.break_length_time {
                    self.program_state = ProgramState::Working(Local::now());
                }

                if user_activity_happened {
                    self.program_state = ProgramState::TakingBreak(Local::now());
                }

                ctx.set_style(opaque_style.clone());

                egui::CentralPanel::default().show(ctx, |ui| {
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

                    // TODO: overlay some text displaying how long till the break is over ??

                    if let Ok(mut file) = File::open("./overlay.png") {

                        let img = ImageSource::Bytes {
                            uri: Cow::from("bytes://overlay.png"),
                            bytes: {
                                let mut v = vec![];
                                file.read_to_end(&mut v).expect("TODO: panic message");
                                Bytes::from(v)
                            },
                        };
                        ui.image(img);
                    } else {
                        ui.image(egui::include_image!("../assets/overlay.png"));
                    }
                });
            }
            ProgramState::Working(working_start_time) => {
                let diff = Local::now().signed_duration_since(working_start_time);

                if diff >= self.break_gap_time {
                    self.program_state = ProgramState::TakingBreak(Local::now());
                }
            }
            ProgramState::Paused => {}
        }

        // egui::TopBottomPanel::top("top panel!").show(&ctx, |ui| {
        //     ui.heading("this is text!!");
        // });

        self.previous_mouse_pos = self.device_query.get_mouse().coords;
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
