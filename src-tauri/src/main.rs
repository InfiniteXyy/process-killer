#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::str::FromStr;

use serde::Serialize;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[derive(Serialize)]
struct ProcessInfo {
    pid: String,
    name: String,
    cpu_usage: f32,
    parent_pid: Option<String>,
    exe: String,
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
            parent_pid: process.parent().map(|pid| pid.to_string()),
            exe: process.exe().display().to_string(),
        });
    }
    data
}

#[tauri::command]
fn kill_process(_window: tauri::Window, pid: String) -> bool {
    let pid = Pid::from_str(pid.as_str()).unwrap();
    let mut sys = System::new();
    sys.refresh_process(pid);
    if let Some(process) = sys.process(pid) {
        let result = process.kill();
        return result;
    }
    false
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_process_list, kill_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
