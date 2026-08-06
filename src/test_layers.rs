use crate::data_types::{Feature, Geometry, Layer};
use eframe::egui;

/// one layer per geometry type, so there is something to look at.
pub fn sample_layers() -> Vec<Layer> {

    let lake = Layer {
        name: "Lake".to_owned(),
        visible: true,
        features: vec![Feature {
            // Deliberately non-convex — which is why we outline it for now
            // instead of using egui's convex-only polygon fill (see Step 5).
            geometry: Geometry::Polygon(vec![
                egui::pos2(-60.0, -20.0),
                egui::pos2(-20.0, -35.0),
                egui::pos2(15.0, -25.0),
                egui::pos2(30.0, -50.0),
                egui::pos2(55.0, -40.0),
                egui::pos2(45.0, -5.0),
                egui::pos2(10.0, 5.0),
                egui::pos2(-15.0, -10.0),
                egui::pos2(-45.0, 5.0),
            ]),
        }],
        stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(30, 100, 200)),
        fill: egui::Color32::from_rgb(180, 210, 240), // unused until we triangulate
    };

    let river = Layer {
        name: "River".to_owned(),
        visible: true,
        features: vec![Feature {
            geometry: Geometry::Polyline(vec![
                egui::pos2(-90.0, 80.0),
                egui::pos2(-70.0, 55.0),
                egui::pos2(-55.0, 60.0),
                egui::pos2(-40.0, 35.0),
                egui::pos2(-30.0, 10.0),
                egui::pos2(-45.0, 5.0), // flows into the lake
            ]),
        }],
        stroke: egui::Stroke::new(2.5, egui::Color32::from_rgb(60, 130, 220)),
        fill: egui::Color32::TRANSPARENT,
    };

    let stations = Layer {
        name: "Stations".to_owned(),
        visible: true,
        features: vec![
            Feature { geometry: Geometry::Point(egui::pos2(-70.0, 55.0)) },
            Feature { geometry: Geometry::Point(egui::pos2(0.0, 40.0)) },
            Feature { geometry: Geometry::Point(egui::pos2(60.0, -60.0)) },
            Feature { geometry: Geometry::Point(egui::pos2(75.0, 30.0)) },
        ],
        stroke: egui::Stroke::new(1.5, egui::Color32::from_rgb(120, 20, 20)),
        fill: egui::Color32::from_rgb(200, 40, 40),
    };

    // Bottom-to-top: polygon under line under points.
    vec![lake, river, stations]
}