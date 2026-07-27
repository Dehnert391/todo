use eframe::{egui::{self, Color32 }};
use crate::gui::font_config::{FontConfig};

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
    delete:bool,
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
                        delete:false,
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

    pub fn deadline(&self) -> String {
        self.deadline.clone()
    }

    pub fn delete(&self) -> bool {
        self.delete
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

                let screen_width = ui.available_width();   
                let label_width = screen_width-160.0;  
           

            let font = FontConfig::default();


            let text = if self.done() {
                                        egui::RichText::new(&self.text)
                                                            .text_style(egui::TextStyle::Monospace)
                                                            .small()
                                } else {
                                        egui::RichText::new(&self.text)
                                            .size(font.font_size())
                                            .strong()
                                            .text_style(egui::TextStyle::Monospace)
                                };

            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.add_sized([label_width, ui.available_height()],
                    egui::Label::new(text)
                                .wrap_mode(egui::TextWrapMode::Wrap)
                );
            });


            state.show_toggle_button(ui, Self::toggle_icon_circle);
        
            ui.horizontal(|ui| {
           
            ui.push_id(self.id, |ui| { 
                egui::ComboBox::from_label(format!("Prio"))
                            .selected_text(format!("{}", selected.clone()))
                            .width(66.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut selected, "High".to_owned(), "High");
                                ui.selectable_value(&mut selected, "Middle".to_owned(), "Middle");
                                ui.selectable_value(&mut selected, "Low".to_owned(), "Low");
                            });
            });

            self.priority = selected;
       
            //ui.label(egui::RichText::new(format!("{} Deadline", &self.deadline)).size(font.font_size()).text_style(egui::TextStyle::Monospace));  
            let delete_button = ui.add(egui::Button::new(egui::RichText::new("x").size(font.button_font_size()).strong()));  

            if delete_button.clicked() {
                self.delete = true;
            }
            });           
        });
        


        state.show_body_indented(&header_res.response, ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Description:");
                  
                ui.add(
                    egui::TextEdit::multiline(&mut self.descrption)
                    .desired_rows(2)
                    .desired_width(ui.available_width()-160.0)
                
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
