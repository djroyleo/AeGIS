//! The Catalog panel (right): browsing folder connections for data.
//!
//! Currently a placeholder. It will be driven by `aegis-io`: folder
//! connections and catalog entries come from `aegis_io::catalog`, and
//! which files are openable is decided by the `aegis_io::DriverRegistry` —
//! never by this panel.

use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    egui::Panel::right("right_panel")
        .default_size(500.0)
        .size_range(egui::Rangef::new(120.0, 600.0))
        .show(ui, |ui| {
            ui.label("Catalog");
            if ui.button("Add folder connection").clicked() {
                ui.close();
            }
        });
}
