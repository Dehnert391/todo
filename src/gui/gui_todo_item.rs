use chrono::{DateTime, NaiveDateTime, Utc};
use eframe::{egui::{self, Color32}};

use crate::core::todoitem::TodoItem;
#[derive(Clone)]
pub struct GuiTodoItem {
    text:String,
    expand:bool,
    done:bool,
    id:u32,
    descrption:String,
    deadline:String,
    priority:String,
}


impl GuiTodoItem {  

    pub fn new(id:u32, todo_item: &TodoItem) -> Self {

        GuiTodoItem { text: todo_item.name().clone(), 
                        expand: todo_item.expand(), 
                        done: todo_item.state(), 
                        id,
                        descrption:todo_item.description(),
                        deadline:todo_item.deadline_as_string(),
                        priority:todo_item.priority(),
                    }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn get_description(&self) -> String {
        self.descrption.clone()
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn expand(&self) -> bool {
        self.expand
    }

    pub fn priority(&self) -> String {
        self.priority.to_string()
    }

    pub fn deadline(&self) -> String{
        self.deadline.clone()
    }
     
    pub fn show(&mut self, ui: &mut egui::Ui) { 
    
        let id = ui.make_persistent_id(&self.id);
        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        self.expand,
        );

        let mut selected = self.priority.clone();

        let header_res = ui.horizontal(|ui| {
            ui.checkbox(&mut self.done, "");
            ui.label(egui::RichText::new(&self.text).size(14.0).strong().text_style(egui::TextStyle::Monospace));

            ui.push_id(self.id, |ui| {
                ui.label(egui::RichText::new(format!("Prio")).size(14.0).strong().text_style(egui::TextStyle::Monospace));   
                egui::ComboBox::from_label(format!(""))
                            .selected_text(format!("{}", selected.clone()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut selected, "Hoch".to_owned(), "Hoch");
                                ui.selectable_value(&mut selected, "Mittel".to_owned(), "Mittel");
                                ui.selectable_value(&mut selected, "Gering".to_owned(), "Gering");

                            });
            });

            self.priority = selected;

            ui.label(egui::RichText::new(format!("Deadline: {}", &self.deadline)).size(14.0).strong().text_style(egui::TextStyle::Monospace));             

            state.show_toggle_button(ui, Self::toggle_icon_circle);
        });


        state.show_body_indented(&header_res.response, ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Beschreibung:");
                });
                  
                ui.add(
                    egui::TextEdit::multiline(&mut self.descrption)
                    .desired_rows(2),
                    );

                });
            });
        self.expand = state.is_open();
    }


    fn toggle_icon_circle(ui: &mut egui::Ui, openness: f32, response: &egui::Response) {
        let stroke = ui.style().interact(&response).fg_stroke;
        let radius = egui::lerp(3.0..=5.0, openness);
        let color = Color32::from_rgba_unmultiplied(37, 244, 127, 255);
        if openness > 0.5 {
            ui.painter()
                .circle_filled(response.rect.center(), radius, color);
        }else {
            ui.painter()
                .circle_filled(response.rect.center(), radius, stroke.color);
        }
    }
}
