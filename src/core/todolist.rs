use crate::core::{file_handler::FileHandler, todoitem::TodoItem};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TodoList {
    id: u32,
    list: Vec<(u32, TodoItem)>,
    path: String,
}

impl TodoList {
    /// Konstructor
    /// Hier prüfe ich ob schon eine Json-Datei abliegt die ich laden kann, wenn nicht wird eine Leere Liste erstellt.
    /// eventuell implementiere ich das mann unterschliedliche Listen laden kann, ob parallel oder mergen bin weiß ich noch nicht.
    pub fn new(path: &str) -> Vec<Self> {
        let file_handler = FileHandler::<TodoList>::build(&path);
        match file_handler.read() {
            Ok(value) => value,
            _ => {
                let res = vec![TodoList {
                    id: 0,
                    list: Vec::new(),
                    path: path.to_string(),
                }];
                file_handler
                    .write(&res)
                    .expect("Schreiben der TodoList-Datei nicht möglich.");
                res
            }
        }
    }

    /// Ein Element wird der Liste hinzugefügt und eine Id für den Eintag wird vergeben.
    /// Ich hatte die Elemente in der ersten Version in einer HashMap organisiert, bin dann aber auf ein Vec umgestiegen,
    /// weil die Position der Elemente beibehalten werden und ich so nicht nach jeder akualisierung erneut sortieren muss.
    pub fn add(
        &mut self,
        item_to_add: &str,
        deadline: Option<chrono::DateTime<Utc>>,
        priority: String,
        description: String,
    ) -> (u32, TodoItem) {
        let item = TodoItem::new(item_to_add.to_string(), deadline, &priority, &description);
        self.id += 1;
        self.list.push((self.id, item.clone()));
        self.update_file();
        (self.id, item)
    }

    pub fn list(&mut self) -> &mut Vec<(u32, TodoItem)> {
        &mut self.list
    }

    /// Der aktuelle Inhalt der Todoliste wird in das .json-File geschrieben hier wird der Inhalt ersetzt um unnütze Daten zu vermeiden.
    pub fn update_file(&self) {
        let file_handler = FileHandler::<TodoList>::build(&self.path);
        match file_handler.write(&vec![self.clone()]) {
            Ok(_) => println!("Die Datei wurde geschrienben."),
            Err(_) => println!("Die Datei konnte nicht geschrieben werden."),
        }
    }

    ///Synchronisiert den Inhalt der toditems mit dem Inhalt aus den gui_todo_items.
    pub fn update_item(&mut self, id: u32, mutator: impl FnOnce(&mut TodoItem)) {
        for (list_id, item) in &mut self.list {
            if *list_id == id {
                mutator(item);
                break;
            }
        }
        self.update_file();
    }

    /// Aktuallisiert die Todoliste wenn elemente gelöscht werden.
    pub fn update_list(&mut self) {
        let temp: Vec<(u32, TodoItem)> = self.list.clone();
        self.list.clear();
        for (key, value) in temp {
            if !value.delete() {
                let _ = &mut self.list.push((key, value.clone()));
            }
        }
        self.update_file();
    }

    pub fn order_by_prio(&mut self) {
        self.list
            .sort_by(|item1, item2| item1.1.priority_state().cmp(&item2.1.priority_state()));
    }

    pub fn order_by_state(&mut self) {
        self.list
            .sort_by(|item1, item2| item1.1.state().cmp(&item2.1.state()));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_order_by_prio() {
        let mut todolist = TodoList::new("./tests/test_order_by_prio.json");
        let result = vec!["High", "High", "Middle", "Middle", "Low", "Low"];

        todolist[0].order_by_prio();

        assert!(todolist[0].list().len() == result.len());

        for index in 0..todolist[0].list().len() {
            assert_eq!(todolist[0].list()[index].1.priority(), result[index]);
        }
    }
}
