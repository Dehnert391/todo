use crate::core::file_handler::FileHandler;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive( Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    width:f32,
    height:f32,
}

impl WindowConfig {
    pub fn new() -> Self {
        let cfg = FileHandler::<WindowConfig>::build("./config_files/window_config.json");
        match cfg.read() {
            Ok(value) => <WindowConfig as Clone>::clone(&value[0]),
            _ => {
                let res = vec![WindowConfig {width:400.0, height:600.0}];
                cfg.write(&res).expect("Schreiben der WindowConfig-Datei nicht möglich.");
                <WindowConfig as Clone>::clone(&res[0])
            },
        }
    }
}

//Getter and Setter
impl WindowConfig {

    pub fn size(&self) -> egui::Vec2 {
        egui::Vec2 { x:self.width, y: self.height }
    }

    pub fn set_size(&self, size:(f32, f32)) {
        let cfg = FileHandler::<WindowConfig>::build("./config_files/window_config.json");
        let res = vec![WindowConfig {width:size.0, height:size.1}];
        cfg.write(&res).expect("Update der WindowConfig-Datei nicht möglich.");
    }
}

