#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::BreakTimeApp;
use eframe::{NativeOptions, Theme};
use egui::{ViewportBuilder, WindowLevel};

#[allow(dead_code)]
const PROGRAM_NAME: &str = env!("CARGO_CRATE_NAME");
#[allow(dead_code)]
const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

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
            decorations: Some(false),
            active: Some(false),
            visible: Some(true),
            fullsize_content_view: None,
            title_shown: Some(false),
            titlebar_buttons_shown: Some(false),
            titlebar_shown: Some(false),
            close_button: Some(false),
            minimize_button: Some(false),
            maximize_button: Some(false),
            window_level: Option::from(WindowLevel::AlwaysOnTop),
            mouse_passthrough: Some(true),
            ..Default::default()
        },
        persist_window: false,
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
}
