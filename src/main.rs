mod core;
mod gui;

use core::todolist::{TodoList};
use gui::windows::MainWindow;


fn main() {

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Todo Liste", native_options,Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))));

} 