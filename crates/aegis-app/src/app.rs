//! The application: top-level state and the frame layout.

use aegis_project::Project;
use aegis_render::MapCamera;
use eframe::egui;

use crate::{dev, map_view, panels};

pub struct AegisApp {
    project: Project,
    camera: MapCamera,
}

impl Default for AegisApp {
    fn default() -> Self {
        let project = if cfg!(debug_assertions) {
            Project::new(dev::sample_layers())
        } else {
            Project::default()
        };
        Self {
            project,
            camera: MapCamera::default(),
        }
    }
}

impl eframe::App for AegisApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        panels::menu_bar::show(ui);
        panels::contents::show(ui, &mut self.project);
        panels::catalog::show(ui);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::WHITE))
            .show(ui, |ui| {
                map_view::show(ui, &self.project, &mut self.camera);
            });
    }
}
