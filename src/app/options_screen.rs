use crate::app::BreakTimeApp;
use eframe::Frame;
use egui::Ui;

pub fn show_options_menu(
    ui: &mut Ui,
    _ctx: &egui::Context,
    break_time_app: &mut BreakTimeApp,
    _frame: &mut Frame,
) {
    ui.heading("Options menu");

    ui.horizontal(|ui| {
        ui.label("Break length time: ");
        ui.add(egui::widgets::Slider::new(
            &mut break_time_app.break_length_time_minutes,
            0.1..=60f32,
        ))
        .on_hover_text(
            "Length of time in minutes that a break will be. Recommended length is 5 minutes.",
        );
    });
    ui.horizontal(|ui| {
        ui.label("Break gap time: ");
        ui.add(egui::widgets::Slider::new(&mut break_time_app.break_gap_time_minutes,0.1..=60f32*3.0))
        .on_hover_text("Length of time in minutes that a working period will be, this is the time between breaks. Recommended to be 55 minutes.");
    });

    ui.horizontal(|ui| {
        ui.checkbox(
            &mut break_time_app.enable_notification,
            "Enable notification on state change: ",
        );
    });

    ui.horizontal(|ui| {
        ui.checkbox(
            &mut break_time_app.display_break_timer,
            "Enable time remaining display on break: ",
        );
    });

    if ui.button("Close options menu").clicked() {
        break_time_app.showing_options_menu = false;
    }
}
