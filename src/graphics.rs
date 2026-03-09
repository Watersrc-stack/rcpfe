use std::fs;
use std::path::PathBuf;
use eframe::egui::{Context, ScrollArea};
use eframe::Frame;
use eframe::egui;
use crate::{read_dir_sorted, Flags, MyEntry};
use crate::element::{Element, ElementStyle};

pub enum AppStyle {
    Icons,
    List
}

pub struct App {
    pub current_path: PathBuf,
    pub path_edit_buffer: String,
    pub flags: Flags,
    pub entries: Vec<MyEntry>,
    pub style: AppStyle,
    pub file_content: String,
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
            style: AppStyle::Icons,
            file_content : String::new(),
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
    pub fn get_file_buffer(&mut self, file_path: PathBuf) {
        if file_path.is_file() {
            let file_buffer = fs::read_to_string(file_path);
            match file_buffer {
                Ok(n) =>  self.file_content = n,
                Err(n) => eprintln!("Error occured when reading a file : {n}"),
            }
        }

    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let folder_img_data = include_bytes!("../assets/folder.png");
        let file_img_data = include_bytes!("../assets/file.png");
        let arrow_data = include_bytes!("../assets/arrow.png");

        ctx.include_bytes("bytes://folder.png", folder_img_data);
        ctx.include_bytes("bytes://file.png", file_img_data);
        ctx.include_bytes("bytes://arrow.png", arrow_data);

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
                let parent_btn: egui::Button = egui::Button::image_and_text(
                    egui::Image::new("bytes://arrow.png").rotate(std::f32::consts::PI * 1.5, egui::Vec2::splat(0.5)),
                    ".."
                );

                if ui.add(parent_btn).clicked() {
                    self.change_dir(parent.to_path_buf());
                }
            }

            ui.separator();

            ScrollArea::vertical().id_salt("text_display").auto_shrink([true; 2]).max_height(150.0).show(ui, |ui| {
                ui.text_edit_multiline(&mut self.file_content);
            });
            ui.separator();

            let mut next_path = None;
            let mut file_path = None;

            let mut scroll: ScrollArea = ScrollArea::vertical();

            let _ = match self.style {
                AppStyle::Icons => {
                    scroll = scroll.auto_shrink([true, false])
                }

                AppStyle::List => {
                    scroll = scroll.auto_shrink([false; 2])
                }
            };
            ScrollArea::vertical().id_salt("file_list").auto_shrink([false; 2]).show(ui, |ui| {

            scroll.scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {

                    match self.style {
                        AppStyle::Icons => {
                            ui.horizontal_wrapped(|ui| {
                                for ent in &self.entries {
                                    let img = if ent.is_dir { egui::Image::new("bytes://folder.png")
                                    } else { egui::Image::new("bytes://file.png") };

                                    let element = Element::new(img, [64.0, 64.0], &ent.name, ElementStyle::Icons);

                                    if ui.add(element).clicked() {
                                        if ent.is_dir {
                                            next_path = Some(ent.path.clone());
                                        } else {
                                            file_path = Some(ent.path.clone());
                                        }
                                    }
                                }
                            });
                        }
                        AppStyle::List => {
                            for ent in &self.entries {
                                let img = if ent.is_dir { egui::Image::new("bytes://folder.png")
                                } else { egui::Image::new("bytes://file.png") };

                                let element = Element::new(img, [24.0, 24.0], &ent.name, ElementStyle::List);

                                if ui.add(element).clicked() {
                                    if ent.is_dir {
                                        next_path = Some(ent.path.clone());
                                    } else {
                                        file_path = Some(ent.path.clone());
                                    }
                                }
                            }
                        }
                    }

                });

            if let Some(path) = next_path {
                self.change_dir(path);
            }

            if let Some(file_buffer) = file_path {
                self.get_file_buffer(file_buffer);
            }
        });
    });
}

}
