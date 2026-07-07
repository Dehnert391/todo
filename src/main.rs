mod core;
mod gui;

use core::todolist::{TodoList};

use chrono::naive;
use eframe::egui;
use gui::windows::MainWindow;




fn main() {
    // Stufe 1
    let todo:Vec<&str> = vec!["Einkaufen", "wäsche waschen", "schulaufgaben machen"];

    for (index, item) in todo.into_iter().enumerate() {
        println!("{}. {}", index+1, item);
    }


    let mut todellu = TodoList::new();

    todellu.add("Einkaufen");
    todellu.add("Wäsche waschen");
    todellu.add("Gartenarbeit");

    todellu.get_todo_element_by_id(2)
            .expect("Fehler")
            .set_state("closed")
            .set_priority_state("middle");


    todellu.add("Mittagessen Kochen")
            .set_priority_state("height");


    todellu.get_todo_element_by_id(0).expect("Fehler").set_state("closed");


    for entry in todellu.list() {
        println!("Eintrag: {:?}", entry);
    }
     
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("my First Gui", native_options,Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))));


    
}
