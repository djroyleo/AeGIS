use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let mut counter: i32 = 0;

    eframe::run_ui_native("AeGIS", options, move |ui, _frame| {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::WHITE))
            .show(ui, |ui| {
                if ui.button("EXIT").clicked() {
                    std::process::exit(0);
                }

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
