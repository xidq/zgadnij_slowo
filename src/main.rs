#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release! YAY! NO MORE CMD!!!!!!!!!!!

use eframe::NativeOptions;

mod ui;

fn main() {
    let native_options = NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((350.,600.)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        crate::ui::ui_defaults::ZgadnijSlowo::name(),
        native_options.clone(),
        Box::new(|_| Ok(Box::<crate::ui::ui_defaults::ZgadnijSlowo>::default()))
    ).unwrap()
}
