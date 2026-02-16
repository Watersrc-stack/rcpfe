mod graphics;

use std::fs;
use std::path::{Path, PathBuf};
use crate::graphics::App;

pub struct Flags {
    pub a: bool, // Hidden files
}

pub struct MyEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

pub fn read_dir_sorted(path: impl AsRef<Path>, show_hidden: bool) -> Vec<MyEntry> {
    let dir = match fs::read_dir(path) {
        Ok(ent) => ent,
        Err(err) => {
            eprintln!("{err}");
            return Vec::new();
        }
    };

    let mut entries: Vec<MyEntry> = dir
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !show_hidden && name.starts_with('.') {
                return None;
            }
            let path = entry.path();
            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            Some(MyEntry { path, name, is_dir })
        })
        .collect();

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    entries
}


pub fn main() -> Result<(), eframe::Error>{
    eframe::run_native(
        "rcpfe",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::default()))),
    )
}
