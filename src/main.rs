#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod gui;

use core::todolist::{TodoList};
use gui::windows::MainWindow;
use eframe::{egui, NativeOptions};

fn main() {

    // Hier baue ich was eigenes. 
    let mut native_options = NativeOptions::default();
    native_options.viewport = egui::ViewportBuilder::default().with_inner_size(egui::vec2(600.0, 400.0));


    let _ = eframe::run_native("", native_options, Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))));
}