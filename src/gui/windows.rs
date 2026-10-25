use crate::{
    TodoList,
    gui::{
        font_config::FontConfig, gui_add_todo_item::NewItem, gui_todo_item::GuiTodoItem,
        window_config::WindowConfig,
    },
};
use eframe::egui::{
    self, Color32, LayerId, Popup, ScrollArea, Vec2, include_image,
    scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
};

#[derive(Default)]
pub struct MainWindow {
    todolist: Vec<TodoList>,
    check_item: Vec<GuiTodoItem>,
    add_item: bool,
    new_item: NewItem,
    temp_len: usize,
    windows_position: (f32, f32),
    windows_size: (f32, f32),
}

impl MainWindow {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        let mut s = Self::default();
        s.init();
        s
    }

    fn init(&mut self) {
        self.add_item = false;
        self.new_item = NewItem::new(&mut self.add_item);
        self.todolist = TodoList::new("./config_files/todolist.json");
        self.check_item = Vec::new();
        for item in self.todolist[0].list() {
            if !item.1.delete() {
                self.check_item.push(GuiTodoItem::new(item.0, &item.1));
            }
        }
        self.temp_len = self.todolist[0].list().len();
    }

    fn comit_item(&mut self, gui: &GuiTodoItem) {
        self.todolist[0].update_item(gui.id(), |item| {
            item.set_expand(gui.expand());
            item.set_state(gui.done().clone());
            item.set_description(gui.get_description());
            item.set_priority_state(&gui.priority());
            item.set_delete_flag(gui.delete());
            // item.set_deadline(gui.deadline());
        });
        //self.init();
    }

    fn render_todo_items(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .auto_shrink(true)
            .scroll_bar_visibility(VisibleWhenNeeded)
            .show(ui, |ui| {
                for todo in &mut self.check_item {
                    if !todo.delete() {
                        ui.push_id(todo.id(), |ui| todo.show(ui));
                        ui.separator();
                    }
                }
            });
    }

    fn synchronise_items(&mut self) {
        for id in self.check_item.clone() {
            let _ = &self.comit_item(&id);
        }

        self.todolist[0].order_by_state();
    }
}

impl eframe::App for MainWindow {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let font = FontConfig::default();

        self.windows_size = (ui.response().rect.width(), ui.response().rect.height());
        //let img = egui::include_image!("23044843_6721707.svg");
        //ui.add(egui::Image::new(img.clone()));
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("My Tiny To-Do List")
                        .size(font.heading_size())
                        .strong(),
                );

                /*  let add_button = ui.add(egui::Button::new(egui::RichText::new("+")
                .size(font.button_font_size())
                .strong())
                .min_size(Vec2{x:25.0, y:25.0}));*/
                let add_img = egui::include_image!("../../config_files/icons/add_task.svg");
                let add_button = ui.add(
                    egui::Button::new(egui::Image::new(add_img))
                        .fill(egui::Color32::from_rgb(56, 58, 59)),
                );

                if add_button.hovered() {
                    let id = ui.make_persistent_id("add_button");
                    Popup::new(
                        id,
                        ui.ctx().clone(),
                        &add_button,
                        LayerId::new(egui::Order::Tooltip, id),
                    )
                    .show(|ui| {
                        ui.label("Create new Task.");
                    });
                }

                if add_button.clicked() {
                    self.add_item = !self.add_item;
                    self.new_item = NewItem::new(&mut self.add_item);
                }

                if self.add_item {
                    self.new_item.show(ui);
                    if self.new_item.created() {
                        let (id, item) = self.todolist[0].add(
                            &self.new_item.name(),
                            None,
                            self.new_item.priority(),
                            self.new_item.description(),
                        );
                        self.check_item.push(GuiTodoItem::new(id, &item));
                        self.todolist[0].order_by_prio();
                        self.synchronise_items();
                        self.todolist[0].update_list();
                        self.init();

                        //self.new_item = NewItem::default();
                        self.add_item = false;
                    }
                    self.add_item = self.new_item.visible();
                }

                let sort_img = egui::include_image!("../../config_files/icons/sort_button.svg");

                let sort_button = ui.add(
                    egui::Button::new(egui::Image::new(sort_img))
                        .fill(egui::Color32::from_rgb(56, 58, 59)),
                );

                if sort_button.hovered() {
                    let id = ui.make_persistent_id("sort_button");
                    Popup::new(
                        id,
                        ui.ctx().clone(),
                        &sort_button,
                        LayerId::new(egui::Order::Tooltip, id),
                    )
                    .show(|ui| {
                        ui.label("Order Tasks by Prio.");
                    });
                }

                if sort_button.clicked() {
                    self.todolist[0].order_by_prio();
                    self.synchronise_items();
                    self.todolist[0].update_list();
                    self.init();
                }
            });

            ui.separator();

            self.render_todo_items(ui);

            if self.todolist[0].list().len() != self.temp_len {
                self.synchronise_items();
                self.init();
            }
        });
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        _ = (ctx, frame);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self) {
        self.synchronise_items();
        WindowConfig::new().set_size(self.windows_size);
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
