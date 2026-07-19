
use std::collections::HashMap;
use crate::core::{file_handler::FileHandler, todoitem::TodoItem};
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TodoList {
    id:u32,
    list:HashMap<u32, TodoItem>,
}


impl TodoList {

    /// Konstructor
    /// Hier prüfe ich ob schon eine txt-Datei abliegt die ich laden kann, wenn nicht wird eine Leere Liste erstellt.
    /// eventuell implementiere ich das mann unterschliedliche Listen laden kann, ob parallel oder mergen bin weiß ich noch nicht. 05.07.2026
    pub fn new() -> Vec<Self> {
        let file_handler = FileHandler::<TodoList>::build(String::from("./config_files/todolist.json"));
        match file_handler.read() {
            Ok(value) => value,
            _ => {
                let res = vec![TodoList {id:0, list:HashMap::new()}];
                file_handler.write(&res).expect("Schreiben der TodoList-Datei nicht möglich.");
                res
            },
        }
    }

    /// Fügt ein Element der Liste bzw. der Map hinzu, die id wird dynamisch vergeben.
    /// Ich Organisier die Elemente in einer HashMap damit ich TodoItem besser zuordnen und Bearbeiten kann.
    pub fn add(&mut self, item_to_add:&str) -> &mut TodoList {

        let item = TodoItem::new(item_to_add.to_string());
        self.id += 1;
        self.list.entry(self.id).or_insert(item);
        self.update_file();
        self
    }
    
    pub fn list(&mut self) -> &mut HashMap<u32, TodoItem> {
        &mut self.list
    }

    fn update_file(&self) {

        let file_handler = FileHandler::<TodoList>::build(String::from("./config_files/todolist.json"));
        match file_handler.write(&vec![self.clone()]) {
            Ok(_) => println!("Die Datei wurde geschrienben."),
            Err(_) => println!("Die Datei konnte nicht geschrieben werden."),
        }
    } 

    pub fn update_item(&mut self, id:u32, mutator: impl FnOnce(&mut TodoItem)) {
        if let Some(item) = self.list.get_mut(&id) {
            mutator(item);
        }
        self.update_file();
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}