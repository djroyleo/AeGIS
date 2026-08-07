//! The top menu bar.

use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    egui::Panel::top("menu_bar")
        .exact_size(24.0)
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                let _ = ui.button("File");
                let _ = ui.button("Edit");
                let _ = ui.button("View");
            });
        });
}
