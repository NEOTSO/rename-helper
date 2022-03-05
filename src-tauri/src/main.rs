use std::fs::{self, File};
use std::path::Path;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[tauri::command]
fn rename(files: Vec<&str>, separator: &str) {
    println!("rename{:?}", files);
    for file in files {
        let path = Path::new(file);
        let md = path.metadata().unwrap();
        if md.is_dir() {
            rename_folder(file, separator);
        } else {
            rename_files(vec![file], separator);
        }
    }
}

#[tauri::command]
fn rename_folder(folder: &str, separator: &str) {
    let paths = fs::read_dir(folder).unwrap();
    for path in paths {
        let md = path.as_ref().unwrap().metadata().unwrap();
        if md.is_dir() {
            rename_folder(path.as_ref().unwrap().path().to_str().unwrap(), separator);
        } else {
            rename_files(
                vec![path.as_ref().unwrap().path().to_str().unwrap()],
                separator,
            );
        }
    }
    let mut ancestors = Path::new(folder).ancestors();
    ancestors.next();
    let parent_folder = ancestors.next().unwrap();
    let file_name = Path::new(folder)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let chars: Vec<String> = file_name.chars().map(|x| x.to_string()).collect();
    let new_file_name = chars.join(separator);
    let new_path = parent_folder.join(new_file_name);
    let result = fs::rename(folder, new_path);
    let result = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}

#[tauri::command]
fn rename_files(files: Vec<&str>, separator: &str) {
    println!("{}", separator);
    println!("{:?}", files);
    for file in files {
        let path = Path::new(file);
        let parent_folder = path.parent().unwrap();
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let file_ext = path.extension().unwrap().to_str().unwrap();
        let chars: Vec<String> = String::from(file_name)
            .chars()
            .map(|x| x.to_string())
            .collect();
        let new_file_name = chars.join(separator) + "." + file_ext;
        let new_path = parent_folder.join(new_file_name);
        let result = fs::rename(file, new_path);
        let result = match result {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file: {:?}", error)
            }
        };
        // println!("{}", item);
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload<'a> {
    files: Vec<&'a str>,
}

fn main() {
    let quit = CustomMenuItem::new("open-folder".to_string(), "打开文件夹");
    let close = CustomMenuItem::new("open-files".to_string(), "打开文件");
    let submenu = Submenu::new("文件", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new().add_submenu(submenu);
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open-folder" => FileDialogBuilder::new().pick_folder(move |folder_path| {
                println!("open folder: {:?}", folder_path);
                let folder = folder_path.unwrap().to_str().unwrap().to_string();
                let _result = event.window().emit(
                    "files-selected",
                    Payload {
                        files: vec![&folder],
                    },
                );
            }),
            "open-files" => {
                // event.window().close().unwrap();
                FileDialogBuilder::new().pick_files(|file_paths| {
                    println!("open files: {:?}", file_paths);
                })
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            rename,
            rename_folder,
            rename_files
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app")
}
