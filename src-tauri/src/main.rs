use std::fs;

// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[tauri::command]
fn rename_files(files: Vec<String>, separator: String) {
    println!("{}", separator);
    println!("{:?}", files);
    for item in files {
        println!("{}", item);
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
        .invoke_handler(tauri::generate_handler![rename_files])
        .run(tauri::generate_context!())
        .expect("failed to run app")
}
