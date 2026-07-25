use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_ui_native("AeGIS", options, |ui, _frame| {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::WHITE))
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    if ui.button("EXIT").clicked() {
                        std::process::exit(0);
                    }
                });
            });
    })
}
