#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod gui;

use core::todolist::{TodoList};
use gui::windows::MainWindow;
use eframe::{NativeOptions, egui::{self}};

use crate::gui::window_config::WindowConfig;

fn main() {
    let cfg = WindowConfig::new();
    let mut native_options = NativeOptions::default();
    native_options.viewport = egui::ViewportBuilder::default().with_inner_size(cfg.size());

    let _ = eframe::run_native("", native_options, Box::new(|cc| {
                                                                                                                egui_extras::install_image_loaders(&cc.egui_ctx); 
                                                                                                                Ok(Box::new(MainWindow::new(cc)))}));
} 