use std::path::PathBuf;
use eframe::egui::Context;
use eframe::Frame;
use eframe::egui;
use crate::{read_dir_sorted, Flags, MyEntry};

pub struct App {
    pub current_path: PathBuf,
    pub flags: Flags,
    pub entries: Vec<MyEntry>,
}

impl Default for App {
    fn default() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let flags = Flags { a: false };
        let entries = read_dir_sorted(&current_path, flags.a);

        App {
            current_path,
            flags,
            entries,
        }
    }
}

impl App {
    pub fn change_dir(&mut self, new_path: PathBuf) {
        if new_path.is_dir() {
            self.current_path = new_path;
            self.entries = read_dir_sorted(&self.current_path, self.flags.a);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RCPFE");
            ui.horizontal(|ui| {
                ui.label(format!("Current path: {}", self.current_path.display()));
                if ui.checkbox(&mut self.flags.a, "Show hidden").changed() {
                    self.entries = read_dir_sorted(&self.current_path, self.flags.a);
                }
            });

            if let Some(parent) = self.current_path.parent() {
                if ui.button(".. (Parent Directory)").clicked() {
                    self.change_dir(parent.to_path_buf());
                }
            }

            ui.separator();

            let mut next_path = None;

            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for ent in &self.entries {
                    let label = if ent.is_dir {
                        format!("D {}", ent.name)
                    } else {
                        format!("F {}", ent.name)
                    };

                    if ui.button(label).clicked() {
                        if ent.is_dir {
                            next_path = Some(ent.path.clone());
                        }
                    }
                }
            });

            if let Some(path) = next_path {
                self.change_dir(path);
            }
        });
    }
}
