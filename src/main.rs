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
