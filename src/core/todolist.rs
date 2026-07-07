
use std::collections::HashMap;
use crate::core::todoitem::TodoItem;

use std::result::Result;

#[derive(Debug)]
pub struct TodoList {
    id:u32,
    list:HashMap<u32, TodoItem>,
}


impl TodoList {

    /// Konstructor
    /// Hier prüfe ich ob schon eine txt-Datei abliegt die ich laden kann, wenn nicht wird eine Leere Liste erstellt.
    /// eventuell implementiere ich das mann unterschliedliche Listen laden kann, ob parallel oder mergen bin weiß ich noch nicht. 05.07.2026
    pub fn new() -> Self {
        TodoList {id:0, list:HashMap::new()}
    }

    /// Fügt ein Element der Liste bzw. der Map hinzu, die id wird dynamisch vergeben.
    /// Ich Organisier die Elemente in einer HashMap damit ich TodoItem besser zuordnen und Bearbeiten kann.
    pub fn add(&mut self, item_to_add:&str) -> &mut TodoItem{

        let item = TodoItem::new(item_to_add.to_string());

        let res = self.list.entry(self.id).or_insert(item);
        self.id += 1;

        res //gibt das TodoItem zurück damit ich es weiter bearbeiten kann ich strebe hier ein Builderpattern an.
    }
    
    pub fn list(&mut self) -> &mut HashMap<u32, TodoItem> {
        &mut self.list
    }

    pub fn get_todo_element_by_id(&mut self, id:u32) -> Result<&mut TodoItem, String> {

        match self.list.get_mut(&id) {
            Some(value) => Ok(value),
            None =>  Err("Fehler beim Laden das TodoItems".to_string()),
        }
    }

}