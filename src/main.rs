mod test_layers;
use test_layers::sample_layers;

mod data_types;
use data_types::{Geometry, Layer};

use eframe::egui;

struct MapCamera {
    /// The world-coordinate point currently at the center of the viewport.
    center: egui::Pos2,
    /// Zoom level: how many screen pixels one world unit occupies.
    pixels_per_unit: f32,
}

impl Default for MapCamera {
    fn default() -> Self {
        Self {
            center: egui::pos2(0.0, 0.0),
            pixels_per_unit: 4.0,
        }
    }
}

impl MapCamera {
    const MIN_ZOOM: f32 = 0.01;
    const MAX_ZOOM: f32 = 1000.0;

    fn world_to_screen(&self, world: egui::Pos2, viewport: egui::Rect) -> egui::Pos2 {
        let c = viewport.center();
        egui::pos2(
            c.x + (world.x - self.center.x) * self.pixels_per_unit,
            c.y - (world.y - self.center.y) * self.pixels_per_unit,
        )
    }

    fn screen_to_world(&self, screen: egui::Pos2, viewport: egui::Rect) -> egui::Pos2 {
        let c = viewport.center();
        egui::pos2(
            self.center.x + (screen.x - c.x) / self.pixels_per_unit,
            self.center.y - (screen.y - c.y) / self.pixels_per_unit,
        )
    }

    fn visible_world_rect(&self, viewport: egui::Rect) -> egui::Rect {
        egui::Rect::from_two_pos(
            self.screen_to_world(viewport.left_top(), viewport),
            self.screen_to_world(viewport.right_bottom(), viewport),
        )
    }
}

// ============================================================================
// Step 1: The application struct
// ============================================================================

struct AegisApp {
    camera: MapCamera,
    /// Draw order = list order: index 0 is drawn first (bottom of the stack).
    layers: Vec<Layer>,
}

impl Default for AegisApp {
    fn default() -> Self {
        Self {
            camera: MapCamera::default(),
            layers: sample_layers(),
        }
    }
}



// ============================================================================
// Steps 3 + 5: The map canvas — input handling and rendering
// ============================================================================

impl AegisApp {
    fn show_map(&mut self, ui: &mut egui::Ui) {
        // Claim the whole remaining area as our canvas. We get back a
        // Response (input events on that region) and a Painter (clipped
        // drawing surface for that region).
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        let viewport = response.rect;

        // ---- Panning ----------------------------------------------------
        // Convert the drag delta from screen space to world space. Dragging
        // right moves the camera *left* (the world slides under the cursor),
        // and Y flips once again.
        if response.dragged() {
            let d = response.drag_delta();
            self.camera.center.x -= d.x / self.camera.pixels_per_unit;
            self.camera.center.y += d.y / self.camera.pixels_per_unit;
        }

        // ---- Zooming (anchored on the cursor) ---------------------------
        if let Some(cursor) = response.hover_pos() {
            // zoom_delta() covers pinch gestures and Ctrl+scroll; we also
            // fold the plain scroll wheel in, since that's the convention
            // in every GIS / web map. (When Ctrl is held, egui reports the
            // scroll as zoom_delta and zeroes smooth_scroll_delta, so the
            // two never double-count.)
            let (pinch, scroll_y) =
                ui.input(|i| (i.zoom_delta(), i.smooth_scroll_delta.y));
            let zoom = pinch * (scroll_y * 0.005).exp();

            if zoom != 1.0 {
                // The trick for cursor-anchored zoom: note which world point
                // is under the cursor, rescale, then shift the camera so that
                // same world point is under the cursor again.
                let before = self.camera.screen_to_world(cursor, viewport);
                self.camera.pixels_per_unit = (self.camera.pixels_per_unit * zoom)
                    .clamp(MapCamera::MIN_ZOOM, MapCamera::MAX_ZOOM);
                let after = self.camera.screen_to_world(cursor, viewport);
                self.camera.center += before - after;
            }
        }

        // ---- Rendering --------------------------------------------------
        const POINT_RADIUS: f32 = 5.0; // in *pixels*: markers keep their size at any zoom

        // Everything outside this world-space rect is invisible and gets
        // skipped. Expanded slightly so pixel-sized point markers straddling
        // the edge don't pop in and out.
        let cull_rect = self
            .camera
            .visible_world_rect(viewport)
            .expand(POINT_RADIUS / self.camera.pixels_per_unit);

        for layer in &self.layers {
            if !layer.visible {
                continue;
            }
            for feature in &layer.features {
                if !cull_rect.intersects(feature.bounds()) {
                    continue; // culled: not on screen
                }

                match &feature.geometry {
                    Geometry::Point(p) => {
                        let screen = self.camera.world_to_screen(*p, viewport);
                        painter.circle_filled(screen, POINT_RADIUS, layer.fill);
                        painter.circle_stroke(screen, POINT_RADIUS, layer.stroke);
                    }
                    Geometry::Polyline(pts) => {
                        let screen: Vec<egui::Pos2> = pts
                            .iter()
                            .map(|p| self.camera.world_to_screen(*p, viewport))
                            .collect();
                        painter.add(egui::Shape::line(screen, layer.stroke));
                    }
                    Geometry::Polygon(pts) => {
                        let screen: Vec<egui::Pos2> = pts
                            .iter()
                            .map(|p| self.camera.world_to_screen(*p, viewport))
                            .collect();
                        // egui can only fill *convex* polygons natively, and
                        // real GIS polygons rarely are. For now: closed
                        // outline. Phase 2: triangulate (e.g. `earcutr`) and
                        // emit a filled Mesh using `layer.fill`.
                        painter.add(egui::Shape::closed_line(screen, layer.stroke));
                    }
                }
            }
        }
    }
}

// ============================================================================
// eframe plumbing
// ============================================================================

impl eframe::App for AegisApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menu_bar")
            .exact_size(24.0)
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    let _ = ui.button("File");
                    let _ = ui.button("Edit");
                    let _ = ui.button("View");
                });
            });

        egui::Panel::left("left_panel")
            .default_size(500.0)
            .size_range(egui::Rangef::new(120.0, 600.0))
            .show(ui, |ui| {
                ui.label("Left Panel");
            });

        egui::Panel::right("right_panel")
            .default_size(500.0)
            .size_range(egui::Rangef::new(120.0, 600.0))
            .show(ui, |ui| {
                ui.label("Right Panel");
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::WHITE))
            .show(ui, |ui| {
                self.show_map(ui);
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "AeGIS",
        options,
        Box::new(|_cc| Ok(Box::new(AegisApp::default()))),
    )
}