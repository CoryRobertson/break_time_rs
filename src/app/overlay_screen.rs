use crate::app::BreakTimeApp;
use crate::color_settings::get_overlay_text_color;
use crate::state::ProgramState;
use chrono::Local;
use eframe::Frame;
use egui::load::Bytes;
use egui::{Align2, FontFamily, FontId, ImageSource, Pos2, Ui};
use std::borrow::Cow;

pub fn show_overlay_screen(
    ui: &mut Ui,
    _ctx: &egui::Context,
    break_time_app: &mut BreakTimeApp,
    _frame: &mut Frame,
) {
    if let Some(bytes) = &break_time_app.overridden_overlay_image {
        let img = ImageSource::Bytes {
            uri: Cow::from("bytes://overlay.png"),
            bytes: Bytes::from(bytes.clone()),
        };
        ui.image(img);
    } else {
        ui.image(egui::include_image!("../../assets/overlay.png"));
    }

    let (width, height) = (
        ui.ctx().screen_rect().width(),
        ui.ctx().screen_rect().height(),
    );

    if let ProgramState::TakingBreak(break_time_start) = &break_time_app.program_state {
        if break_time_app.display_break_timer {
            let diff = break_time_app.break_length_time
                - Local::now().signed_duration_since(break_time_start);
            let display_time = format!(
                "{}:{:02}",
                diff.num_minutes(),
                diff.num_seconds() - (diff.num_minutes() * 60)
            );

            ui.painter().text(
                Pos2::new(width / 2.0, height / 2.0),
                Align2::CENTER_CENTER,
                display_time,
                FontId::new(60.0, FontFamily::Proportional),
                get_overlay_text_color(),
            );
        }
    }
}
