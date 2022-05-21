#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
use commands::{close_splashscreen, get_port_list, get_process_list, kill_process};
use tauri::{Menu, MenuItem, Submenu};

fn main() {
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    let menu = Menu::new().add_submenu(edit_menu);

    let mut builder = tauri::Builder::default();
    if cfg!(target_os = "macos") {
        builder = builder.menu(menu)
    }
    builder
        .invoke_handler(tauri::generate_handler![
            get_process_list,
            kill_process,
            get_port_list,
            close_splashscreen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
