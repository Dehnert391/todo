
use eframe::egui;

#[derive(Default)]
pub struct MainWindow {}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        Self::default()
    }
}


impl eframe::App for MainWindow {

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut  eframe::Frame) {

        egui::CentralPanel::default().show(ui, |ui| {

            ui.heading("Meine TODO-Liste.");

            ui.label("Einkaufen");
            ui.ctx();
            ui.toggle_value(&mut true, "frgtr")
        });
    }
}