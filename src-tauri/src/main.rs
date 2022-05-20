#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

#[derive(Serialize)]
struct ProcessInfo {
    pid: String,
    name: String,
    cpu_usage: f32,
}

#[tauri::command]
fn get_process_list() -> Vec<ProcessInfo> {
    let mut sys = System::new();
    sys.refresh_processes();

    let mut data: Vec<ProcessInfo> = Vec::new();

    for (pid, process) in sys.processes() {
        data.push(ProcessInfo {
            pid: pid.to_string(),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
        });
    }

    data
}

#[tauri::command]
fn kill_process(_window: tauri::Window, pid: u32) {
    let sys = System::new();
    sys.process(Pid::from_u32(pid)).unwrap().kill();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_process_list, kill_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
