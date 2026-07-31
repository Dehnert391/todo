//se chrono::{NaiveDate, Utc};
use eframe::egui::{self};
use jiff::civil::Date;
use jiff::Zoned;

use crate::gui::font_config::FontConfig;
#[derive(Debug, Default)]
pub struct NewItem {
    name:String,
    priority_selected:String,
    deadline:jiff::civil::Date,
    created:bool,
    description:String,
    visible:bool,
}


impl NewItem {

    pub fn new(visible: &mut bool) -> Self {
        Self{
            name:String::new(),
            priority_selected:"Low".to_string(),
            deadline:Date::new(
                        Zoned::now().year(),
                        Zoned::now().month(),
                        Zoned::now().day(),
                        ).unwrap(),
            created:false,
            description:"".to_string(),
            visible:*visible,

        }
    }


    pub fn name(&self) -> String{
        self.name.clone()
    }

    pub fn deadline(&self) -> jiff::civil::Date {
        self.deadline
    }

    pub fn priority(&self) -> String {
        self.priority_selected.clone()
    }

    pub fn created(&self) -> bool {
        self.created
    }
    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let font = FontConfig::default();
        let ctx= ui.ctx();
        egui::Window::new("Add new Todo.")
            .collapsible(false)
            .movable(true)
            .show(ctx, |ui| {
                    ui.set_min_width(310.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui|{
                            ui.label(egui::RichText::new("Text:").size(font.font_size()).strong().text_style(egui::TextStyle::Monospace));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.name)
                            )
                        });
                        ui.horizontal(|ui|{
                            ui.push_id(self.priority_selected.clone(), |ui| {
                            ui.label(egui::RichText::new("Priority").size(font.font_size()).strong().text_style(egui::TextStyle::Monospace));
                            egui::ComboBox::from_label("")
                            .selected_text(format!("{}", self.priority_selected))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.priority_selected, "High".to_owned(), "High");
                                ui.selectable_value(&mut self.priority_selected, "Middle".to_owned(), "Middle");
                                ui.selectable_value(&mut self.priority_selected, "Low".to_owned(), "Low");

                            });
                            });
                        });

                        /* 
                        ui.horizontal(|ui|{
                            let mut radio_state = false;
                            ui.label(egui::RichText::new("Deadline").size(font.font_size()).strong().text_style(egui::TextStyle::Monospace));
                            ui.radio_value(&mut radio_state, _, "YES");
                            ui.add(egui_extras::DatePickerButton::new(&mut self.deadline));
                        
                        });*/
                        

                        ui.vertical(|ui| {

                        ui.label(egui::RichText::new("Description").size(font.font_size()).strong().text_style(egui::TextStyle::Monospace));
                        ui.horizontal(|ui|{
                            ui.add(
                                egui::TextEdit::multiline(&mut self.description)
                                                .desired_width(ui.available_width())
                                                .desired_rows(5)
                                )
                            });
                        });

                        ui.horizontal(|ui|{
        
                        if ui.button("Create").clicked() {
                            if self.name().len() == 0 {
                                println!("Es wurde kein Text eingegeben.");
                            }else {
                                //self.priority_selected = selected;
                                self.created = true;
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.visible = !self.visible;
                        }
                    });
                });
            });
    }
}