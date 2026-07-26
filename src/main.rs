use eframe::egui;

struct AeGISApp {
    camera: MapCamera,
    layers: Vec<Layer>,
}

impl eframe::App for AeGISApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_map(ui);
        });
    }
}

struct MapCamera {
    center: egui::Pos2,
    pixels_per_unit: f32,
}

impl MapCamera {
    fn world_to_screen(&self, world: egui::Pos2, viewport: egui::Rect) -> egui::Pos2 {
        let c = viewport.center();
        egui::Pos2 {
            c.x + (world.x - self.center.x) * self.pixels_per_unit,
            c.y - (world.y - self.center.y) * self.pixels_per_unit,
        }
    }

    fn screen_to_world(&self, screen: egui::Pos2, viewport: egui::Rect) -> egui::Pos2 {
        let c = viewport.center();
        egui::Pos2 {
            self.center.x + (screen.x - c.x) / self.pixels_per_unit,
            self.center.y - (screen.y - c.y) / self.pixels_per_unit,
        }
    }
}

let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

let viewport = response.rect;

if response.dragged() {
    let d = response.drag_delta();
    self.camera.center.x -= d.x / self.camera.pixels_per_unit;
    self.camera.center.y += d.y / self.camera.pixels_per_unit;
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let mut counter: i32 = 0;

    eframe::run_ui_native("AeGIS", options, move |ui, _frame| {

        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() && counter > 0 {
                    counter -= 1;
                }
                if ui.button("Reset Counter").clicked() {
                    counter = 0;
                }
            });
        });

        egui::Panel::left("contents_panel")
            .exact_size(250.0)
            .show(ui, |ui| {
                ui.label("Contents pane");
        });

        egui::Panel::right("catalog_panel")
            .exact_size(250.0)
            .show(ui, |ui| {
                ui.label("Catalog pane");
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::WHITE))
            .show(ui, |ui| {
                let half_height = ui.available_height() / 2.0;
                ui.vertical_centered(|ui| {
                    ui.add_space(half_height - 30.0);
                    ui.heading(egui::RichText::new(counter.to_string()).color(egui::Color32::BLACK));
                    if ui.button("ADD ONE").clicked() {
                        counter += 1;
                    }
                });
            });
    })
}
