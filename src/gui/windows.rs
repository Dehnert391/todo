use eframe::egui::{self, Vec2};
use crate::{TodoList, gui::gui_todo_item::GuiTodoItem};
#[derive(Default)]
pub struct MainWindow {
    todellu:Vec<TodoList>,
    check_item:Vec<GuiTodoItem>,
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        let mut s = Self::default();
        s.init();
        s 
    }

    fn init(&mut self) {
        
        self.todellu = TodoList::new();
        //self.todellu[0].add("Todos hinzufügen implementieren.");
        self.check_item = Vec::new();
        for item in self.todellu[0].list() {
            self.check_item.push(GuiTodoItem::new(*item.0, item.1));
        }
    }
    
    fn comit_item(&mut self, gui:&GuiTodoItem) {
        self.todellu[0].update_item(gui.id(), |item| {
            item.set_expand(gui.expand());
            item.set_state(gui.done());
            item.set_description(gui.get_description());
            item.set_priority_state(&gui.priority());
           // item.set_deadline(gui.deadline());
        })

    }

    fn render_todo_items(&mut self, ui: &mut egui::Ui) {
        for todo in &mut self.check_item {   
            todo.show(ui)
        }
    }
}


impl eframe::App for MainWindow {

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut  eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("My Ugly TODO-List").size(24.0).strong());
                let add_button = ui.add(egui::Button::new(egui::RichText::new("+").size(20.0).strong()).min_size(Vec2{x:25.0, y:25.0}));
                if add_button.clicked() {

                    self.todellu[0].add("Neuer Eintrag.");

                    println!("{:?}", self.todellu);
                    
                    self.check_item = Vec::new();
                    for item in self.todellu[0].list() {
                        self.check_item.push(GuiTodoItem::new(*item.0, item.1));
                    }
                    self.render_todo_items(ui);
                }
            });

            self.render_todo_items(ui);
        });
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        _ = (ctx, frame);
    }
    
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        for id in self.check_item.clone() {
            let _ = &self.comit_item(&id);
        }
        println!("Save");
    }
    
    fn on_exit(&mut self) {
        for id in self.check_item.clone() {
            let _ = &self.comit_item(&id);
        }
    }
    
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 240).to_normalized_gamma_f32()
    
        // _visuals.window_fill() would also be a natural choice
    }
    
    fn persist_egui_memory(&self) -> bool {
        true
    }
    
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}