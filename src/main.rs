mod graphics;

use std::fs;
use eframe::egui::Context;
use eframe::Frame;
use eframe::egui;
use eframe::egui::debug_text::print;

struct Flags {
    a: bool, // Hidden files

}

struct MyEntries {
    item: fs::DirEntry,
    name: String,
}

fn read_dir_sorted(path: &String) -> Vec<MyEntries> {
    let dir = match fs::read_dir(path) {
        Ok(ent) => ent,
        Err(err) => {
            eprintln!("{err}");
            return Vec::new();
        }
    };
    let direntries: Vec<fs::DirEntry> = dir.filter_map(|entry| entry.ok()).collect();

    let mut entries: Vec<MyEntries> = Vec::new();

    for dirent in direntries {
        entries.push(MyEntries {
            name: dirent.file_name().to_string_lossy().to_string().clone(),
            item: dirent
        })
    }

    entries.sort_by(|a, b| a.item.path().to_str().unwrap().to_lowercase().cmp(&b.item.path().to_str().unwrap().to_lowercase()));

    entries
}

fn fmt_display_entries(entries: &Vec<MyEntries>, flags: &Flags) -> String {
    let mut str_all_entries: Vec<String> = Vec::new();


    for entry in entries {
        let entry_ostr = entry.item.file_name();


        // a flag
        let entry_bytes = entry_ostr.as_encoded_bytes();
        let point = std::ffi::OsString::from(".");
        let point = point.as_encoded_bytes();

        if (entry_bytes[0] == point[0]) && (!flags.a) {
            continue;
        }

        // type
        let filetype;
        match entry.item.file_type() {
            Ok(ft) => {
                if ft.is_dir() {
                    filetype = "D"
                } else {
                    filetype = "F"
                }
            },
            Err(_) => filetype = "U"
        };

        // final
        let entry = entry_ostr.display();
        str_all_entries.push(std::format!("  {filetype}  {entry}"));

    }

    str_all_entries.join("\n")
}

struct App {
    default_path: String,
    current_path: String,
    flags: Flags,
    entries: Vec<MyEntries>,
    name: String,
    age: u32,
}

impl Default for App {
    fn default() -> Self {
        let default_path: String  = match std::env::home_dir() {
            Some(p) => p.display().to_string() + "/",
            None => String::from("./"),
        };
        let current_path: String = String::from(&default_path);

        let flags = Flags {
            a: true
        };
        let entries: Vec<MyEntries> = read_dir_sorted(&current_path);
        App {
            default_path,
            current_path,
            flags,
            entries,

            name: "alice".to_string(),
            age: 0,
        }
    }
}

impl App {
    pub fn change_dir(&mut self, new_path: String) {

        self.current_path = new_path;
        self.entries = read_dir_sorted(&self.current_path);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RCPFE");
/*            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Age"));
            if ui.button("Click me!").clicked() {
                println!("Hello, {}! You are {} years old.", self.name, self.age);
            }*/

            for ent in &self.entries {
                if ui.button(&ent.name).clicked() {
                    // self.change_dir(ent.item.path())
                    println!("Change dir to {}", ent.item.file_name().to_string_lossy())
                }
            }


        });
    }

}


pub fn main() -> Result<(), eframe::Error>{
    eframe::run_native(
        "rcpfe",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::default()))),
    )
}
