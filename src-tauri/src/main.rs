#![windows_subsystem = "windows"]
use std::fs;
use std::path::Path;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Menu, Submenu};
use tauri::{Window};

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
fn rename(files: Vec<&str>, keywords: &str, window: Window) {
    for file in files {
        let path = Path::new(file);
        let md = path.metadata().unwrap();
        if md.is_dir() {
            let _result = rename_folder(path, keywords);
        } else {
            let _result = rename_files(vec![&path], keywords);
        }
    }
    let _result = window.emit(
        "done",
        Message {
            message: "重命名完成！",
        },
    );
}

fn rename_folder(folder: &Path, keywords: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        let md = dir.metadata()?;
        if md.is_dir() {
            rename_folder(dir.path().as_path(), keywords)?;
        } else {
            rename_files(vec![dir.path().as_path()], keywords)?;
        }
    }
    let mut ancestors = folder.ancestors();
    ancestors.next();
    let parent_folder = ancestors.next().unwrap();
    let file_name = folder.file_name().unwrap();
    let chars: Vec<String> = String::from(file_name.to_str().unwrap())
        .chars()
        .map(|x| x.to_string())
        .collect();
    let new_file_name = chars.join(keywords);
    let new_path = parent_folder.join(new_file_name);
    fs::rename(folder, new_path)?;
    Ok(())
}

fn rename_files(files: Vec<&Path>, keywords: &str) -> std::io::Result<()> {
    for path in files {
        let parent_folder = path.parent().unwrap();
        let file_name = path.file_stem().unwrap();
        let file_ext = path.extension().unwrap();
        let chars: Vec<String> = String::from(file_name.to_str().unwrap())
            .chars()
            .map(|x| x.to_string())
            .collect();    
        // let new_file_name = chars.join(keywords) + "." + file_ext.to_str().unwrap();
        let new_file_name = chars.join(keywords) + "㊙" + file_ext.to_str().unwrap();
        let new_path = parent_folder.join(new_file_name);
        fs::rename(path, new_path)?;
    }
    Ok(())
}

#[tauri::command]
fn restore(files: Vec<&str>, keywords: &str, window: Window) {
    for file in files {
        let path = Path::new(file);
        let md = path.metadata().unwrap();
        if md.is_dir() {
            let _result = restore_folder(path, keywords);
        } else {
            let _result = restore_files(vec![path], keywords);
        }
    }
    let _result = window.emit(
        "done",
        Message {
            message: "还原完成！",
        },
    );
}

fn restore_folder(folder: &Path, keywords: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        let md = dir.metadata()?;
        if md.is_dir() {
            restore_folder(dir.path().as_path(), keywords)?;
        } else {
            restore_files(vec![dir.path().as_path()], keywords)?;
        }
    }
    let parent_folder = folder.parent().unwrap();
    let file_name = folder.file_name().unwrap();
    let chars: Vec<String> = String::from(file_name.to_str().unwrap())
        .chars()
        .map(|x| x.to_string())
        .filter(|x| x != keywords)
        .collect();
    let new_file_name = chars.join("");
    let new_path = parent_folder.join(new_file_name);
    fs::rename(folder, new_path)?;
    Ok(())
}

fn restore_files(files: Vec<&Path>, keywords: &str) -> std::io::Result<()> {
    for path in files {
        let parent_folder = path.parent().unwrap();
        let file_name = path.file_stem().unwrap();
        let file_ext = path.extension().unwrap();
        let chars: Vec<String> = String::from(file_name.to_str().unwrap())
            .chars()
            .map(|x| x.to_string())
            .filter(|x| x != keywords)
            .collect();
        let new_file_name = chars.join("") + "." + file_ext.to_str().unwrap();
        let new_path = parent_folder.join(new_file_name);
        fs::rename(path, new_path)?;
    }
    Ok(())
}

#[tauri::command]
fn delete_str(files: Vec<&str>, keywords: &str, window: Window) {
    for file in files {
        let path = Path::new(file);
        let md = path.metadata().unwrap();
        if md.is_dir() {
            let _result = delete_folder_str(path, keywords);
        } else {
            let _result = delete_files_str(vec![path], keywords);
        }
    }
    let _result = window.emit(
        "done",
        Message {
            message: "删除完成！",
        },
    );
}

fn delete_folder_str(folder: &Path, keywords: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        let md = dir.metadata()?;
        if md.is_dir() {
            delete_folder_str(dir.path().as_path(), keywords)?;
        } else {
            delete_files_str(vec![dir.path().as_path()], keywords)?;
        }
    }
    let parent_folder = folder.parent().unwrap();
    let file_name = folder.file_name().unwrap();
    let new_folder_name = str::replace(file_name.to_str().unwrap(), keywords, "");
    let new_path = parent_folder.join(new_folder_name);
    fs::rename(folder, new_path)?;
    Ok(())
}

fn delete_files_str(files: Vec<&Path>, keywords: &str) -> std::io::Result<()> {
    for path in files {
        let parent_folder = path.parent().unwrap();
        let file_name = path.file_name().unwrap();
        let new_file_name = str::replace(file_name.to_str().unwrap(), keywords, "");
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
                let _result = event.window().emit(
                    "files-selected",
                    Payload {
                        files: vec![folder_path.unwrap().to_str().unwrap()],
                    },
                );
            }),
            "open-files" => {
                FileDialogBuilder::new().pick_files(move |file_paths| {
                    let _result = event.window().emit(
                        "files-selected",
                        Payload {
                            files: file_paths.unwrap().iter().map(|x| x.to_str().unwrap()).collect(),
                        },
                    );
                })
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![rename, restore, delete_str])
        .run(tauri::generate_context!())
        .expect("failed to run app")
}
