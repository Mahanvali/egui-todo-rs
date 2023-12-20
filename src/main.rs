use egui::CentralPanel;
use std::collections::BTreeMap;

// Create TodoApp
struct TodoApp {
    tasks: BTreeMap<u64, String>,
    next_id: u64,
    new_task: String,
}

// Implement default values
impl Default for TodoApp {
    fn default() -> Self {
        Self {
            // assign defaults
            tasks: BTreeMap::new(),
            next_id: 0,
            new_task: String::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            //Heading of the app
            ui.heading("Todo List");

            // Remove function
            let mut to_remove = Vec::new();
            for (&id, task) in &self.tasks {
                ui.horizontal(|ui| {
                    ui.label(task);
                    if ui.button("Remove").clicked() {
                        to_remove.push(id);
                    }
                });
            }
            for id in to_remove {
                self.tasks.remove(&id);
            }
            // Inserting todo
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Add").clicked() {
                    self.tasks.insert(self.next_id, self.new_task.clone());
                    self.next_id += 1;
                    self.new_task.clear();
                }
            });
        });
    }
}

// Running the app
fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let _ = eframe::run_native("Todo app", options,  Box::new(|_cc| Box::<TodoApp>::default()));
}