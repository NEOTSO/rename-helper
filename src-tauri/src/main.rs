use std::fs::{self, File};
use std::path::Path;
use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use tauri::{Manager, Window};

// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[derive(Clone, serde::Serialize)]
struct Payload<'a> {
    files: Vec<&'a str>,
}

#[derive(Clone, serde::Serialize)]
struct Message<'a> {
    message: &'a str,
}

#[tauri::command]
fn rename(files: Vec<&str>, separator: &str, window: Window) {
    for file in files {
        let path = Path::new(file);
        let md = path.metadata().unwrap();
        if md.is_dir() {
            let _result = rename_folder(path, separator);
        } else {
            let _result = rename_files(vec![&path], separator);
        }
    }
    let _result = window.emit(
        "done",
        Message {
            message: "重命名完成！",
        },
    );
}

fn rename_folder(folder: &Path, separator: &str) -> std::io::Result<()> {
    // fn rename_folder(folder: PathBuf, separator: &str) {
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        println!("{:?}", dir.path());
        println!("{:?}", dir.metadata());
        let md = dir.metadata()?;
        // println!("{:?}", md);
        println!("{:?}", md.is_dir());
        if md.is_dir() {
            rename_folder(dir.path().as_path(), separator)?;
        } else {
            rename_files(vec![dir.path().as_path()], separator)?;
        }
    }
    println!("####");
    println!("{:?}", folder);
    println!("{:?}", folder.parent().unwrap());
    let mut ancestors = folder.ancestors();
    ancestors.next();
    let parent_folder = ancestors.next().unwrap();
    let file_name = folder.file_name().unwrap();
    let chars: Vec<String> = String::from(file_name.to_str().unwrap())
        .chars()
        .map(|x| x.to_string())
        .collect();
    let new_file_name = chars.join(separator);
    let new_path = parent_folder.join(new_file_name);
    println!("重命名文件夹:");
    println!("{:?}", folder);
    println!("{:?}", new_path);
    fs::rename(folder, new_path)?;
    Ok(())
}

fn rename_files(files: Vec<&Path>, separator: &str) -> std::io::Result<()> {
    for path in files {
        // let path = Path::new(file);
        let parent_folder = path.parent().unwrap();
        let file_name = path.file_stem().unwrap();
        let file_ext = path.extension().unwrap();
        let chars: Vec<String> = String::from(file_name.to_str().unwrap())
            .chars()
            .map(|x| x.to_string())
            .collect();
        let new_file_name = chars.join(separator) + "." + file_ext.to_str().unwrap();
        let new_path = parent_folder.join(new_file_name);
        fs::rename(path, new_path)?;
    }
    Ok(())
}

#[tauri::command]
fn restore(files: Vec<&str>, separator: &str, window: Window) {
    for file in files {
        let path = Path::new(file);
        let md = path.metadata().unwrap();
        if md.is_dir() {
            let _result = restore_folder(path, separator);
        } else {
            let _result = restore_files(vec![path], separator);
        }
    }
    let _result = window.emit(
        "done",
        Message {
            message: "还原完成！",
        },
    );
}

fn restore_folder(folder: &Path, separator: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        let md = dir.metadata()?;
        if md.is_dir() {
            restore_folder(dir.path().as_path(), separator)?;
        } else {
            restore_files(vec![dir.path().as_path()], separator)?;
        }
    }
    let parent_folder = folder.parent().unwrap();
    let file_name = folder.file_name().unwrap();
    let chars: Vec<String> = String::from(file_name.to_str().unwrap())
        .chars()
        .map(|x| x.to_string())
        .filter(|x| x != separator)
        .collect();
    let new_file_name = chars.join("");
    let new_path = parent_folder.join(new_file_name);
    fs::rename(folder, new_path)?;
    Ok(())
}

fn restore_files(files: Vec<&Path>, separator: &str) -> std::io::Result<()> {
    for path in files {
        let parent_folder = path.parent().unwrap();
        let file_name = path.file_stem().unwrap();
        let file_ext = path.extension().unwrap();
        let chars: Vec<String> = String::from(file_name.to_str().unwrap())
            .chars()
            .map(|x| x.to_string())
            .filter(|x| x != separator)
            .collect();
        let new_file_name = chars.join("") + "." + file_ext.to_str().unwrap();
        let new_path = parent_folder.join(new_file_name);
        fs::rename(path, new_path)?;
    }
    Ok(())
}

// #[tauri::command]
// fn send_message(window: Window, message: &str) {
//     window.emit("done", Message { message }).unwrap();
// }

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
        .invoke_handler(tauri::generate_handler![rename, restore,])
        .run(tauri::generate_context!())
        .expect("failed to run app")
}
