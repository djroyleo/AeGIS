//! The Contents panel (left): the layer stack of the open project.

use aegis_project::Project;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, project: &mut Project) {
    egui::Panel::left("left_panel")
        .default_size(500.0)
        .size_range(egui::Rangef::new(120.0, 600.0))
        .show(ui, |ui| {
            ui.label("Contents");
            for layer in &mut project.layers {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut layer.visible, "");
                    let name_response =
                        ui.add(egui::Label::new(&layer.name).sense(egui::Sense::click()));
                    name_response.context_menu(|ui| {
                        if ui.button("Open attribute table").clicked() {
                            ui.close();
                        }
                        if ui.button("Symbology").clicked() {
                            ui.close();
                        }
                        if ui.button("Properties").clicked() {
                            ui.close();
                        }
                    });
                });
            }
        });
}
