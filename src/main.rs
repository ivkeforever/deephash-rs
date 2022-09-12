#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use file_hasher_rs::HashApp;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        "Rust File Hasher (v1.0.1)",
        options,
        Box::new(|_cc| Box::new(HashApp::default())),
    );
}
