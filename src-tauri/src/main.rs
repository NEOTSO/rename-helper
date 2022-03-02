use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::fs::DirEntry;

// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[tauri::command]
fn rename_folder(folder: &str, separator: &str) {
    let paths = fs::read_dir(folder).unwrap();
    for path in paths {
        let md = path.as_ref().unwrap().metadata().unwrap();
        if md.is_dir() {
            rename_folder(path.as_ref().unwrap().path().to_str().unwrap(), separator);
        } else {
            rename_files(vec![path.as_ref().unwrap().path().to_str().unwrap()], separator);
        }
    }
    let mut ancestors = Path::new(folder).ancestors();
    ancestors.next();
    let parent_folder = ancestors.next().unwrap();
    let file_name = Path::new(folder).file_name().unwrap().to_str().unwrap().to_string();
    let chars: Vec<String> = file_name.chars().map(|x| x.to_string()).collect();
    let new_file_name = chars.join(separator);
    let new_path = parent_folder.join(new_file_name);
    let result = fs::rename(folder, new_path);
    let result = match result {
        Ok(file) => {
            file
        },
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}

#[tauri::command]
fn rename_files(files: Vec<&str>, separator: &str) {
    println!("{}", separator);
    println!("{:?}", files);
    for file in files {
        let path = Path::new(file);
        let parent_folder = path.parent().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let chars: Vec<String> = String::from(file_name).chars().map(|x| x.to_string()).collect();
        let new_file_name = chars.join(separator);
        let new_path = parent_folder.join(new_file_name);
        let result = fs::rename(file, new_path);
        let result = match result {
            Ok(file) => {
                file
            },
            Err(error) => {
                panic!("Problem opening the file: {:?}", error)
            },
        };
        // println!("{}", item);
    }
}

// use tauri::api::dialog::FileDialogBuilder;
// tauri::Builder::default()
//   .build(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
//   .expect("failed to build tauri app")
//   .run(|_app, _event| {
//     FileDialogBuilder::new().pick_file(|file_path| {
//       // do something with the optional file path here
//       // the file path is `None` if the user closed the dialog
//     })
//   })

// fn main() {
//   tauri::Builder::default()
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");
// }

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        // .invoke_handler(tauri::generate_handler![my_custom_command])
        .invoke_handler(tauri::generate_handler![rename_folder, rename_files])
        .run(tauri::generate_context!())
        .expect("failed to run app")
}
