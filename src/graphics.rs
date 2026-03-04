use std::fmt::Debug;
use std::path::PathBuf;
use eframe::egui::{Context, ScrollArea};
use eframe::Frame;
use eframe::egui;
use crate::{read_dir_sorted, Flags, MyEntry};
use crate::element::{Element, ElementStyle};

#[derive(Debug)]
pub enum AppStyle {
    Icons,
    List
}

pub struct App {
    pub current_path: PathBuf,
    pub path_edit_buffer: String,
    pub flags: Flags,
    pub entries: Vec<MyEntry>,
    pub style: AppStyle
}


impl Default for App {
    fn default() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let path_edit_buffer = current_path.to_string_lossy().to_string();
        let flags = Flags { a: false };
        let entries = read_dir_sorted(&current_path, flags.a);

        App {
            current_path,
            path_edit_buffer,
            flags,
            entries,
            style: AppStyle::Icons
        }
    }
}

impl App {
    pub fn change_dir(&mut self, new_path: PathBuf) {
        if new_path.is_dir() {
            self.current_path = new_path;
            self.path_edit_buffer = self.current_path.to_string_lossy().to_string();
            self.entries = read_dir_sorted(&self.current_path, self.flags.a);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RCPFE");
            ui.horizontal(|ui| {

                ui.label("Current path: ");
                let response = ui.text_edit_singleline(&mut self.path_edit_buffer);

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.change_dir(PathBuf::from(&self.path_edit_buffer));
                }

                if ui.checkbox(&mut self.flags.a, "Show hidden").changed() {
                    self.entries = read_dir_sorted(&self.current_path, self.flags.a);
                }

                if ui.button("Toggle icons").clicked() {
                    match self.style {
                        AppStyle::Icons => self.style = AppStyle::List,
                        AppStyle::List => self.style = AppStyle::Icons,
                    }
                }

            });

            if let Some(parent) = self.current_path.parent() {
                if ui.button(".. (Parent Directory)").clicked() {
                    self.change_dir(parent.to_path_buf());
                }
            }

            ui.separator();

            let mut next_path = None;

            let mut scroll: ScrollArea = ScrollArea::vertical();

            let _ = match self.style {
                AppStyle::Icons => {
                    scroll = scroll.auto_shrink([true, false])
                }

                AppStyle::List => {
                    scroll = scroll.auto_shrink([false; 2])
                }
            };

            scroll.scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {

                    for ent in &self.entries {

                    let img = if ent.is_dir { egui::Image::new("file://assets/folder.png")
                    } else { egui::Image::new("file://assets/file.png") };

                    let element: Element;

                    match self.style {
                        AppStyle::Icons => {
                            element = Element::new(img, [24.0, 24.0], &ent.name, ElementStyle::Icons);
                        }
                        AppStyle::List => {
                            element = Element::new(img, [24.0, 24.0], &ent.name, ElementStyle::List);
                        }
                    }

                        if ui.add(element).clicked() {
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
