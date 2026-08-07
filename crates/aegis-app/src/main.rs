//! `AeGIS` — the desktop application binary.
//!
//! This is the **only** crate in the workspace that may depend on
//! `eframe`/`egui`. Everything else (data models, project, IO,
//! geoprocessing, render) is pure data + logic, so the GUI toolkit stays
//! swappable and features never force GUI rewrites.

mod app;
mod dev;
mod map_view;
mod panels;
mod render_egui;

use app::AegisApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "AeGIS",
        options,
        Box::new(|_cc| Ok(Box::new(AegisApp::default()))),
    )
}
