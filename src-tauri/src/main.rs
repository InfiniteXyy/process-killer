#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod structs;

use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use structs::{PortInfo, ProcessInfo};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

#[tauri::command]
fn get_process_list() -> Vec<ProcessInfo> {
    let mut sys = System::new();
    sys.refresh_processes();
    let mut data: Vec<ProcessInfo> = Vec::new();
    for (pid, process) in sys.processes() {
        data.push(ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            parent_pid: process.parent().map(|pid| pid.as_u32()),
            exe: process.exe().display().to_string(),
        });
    }
    data
}

#[tauri::command]
fn get_port_list() -> Vec<PortInfo> {
    let mut data: Vec<PortInfo> = Vec::new();
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).expect("error");
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => data.push(PortInfo {
                local_addr: tcp_si.local_addr.to_string(),
                local_port: tcp_si.local_port.to_string(),
                pids: si.associated_pids,
                tcp_state: Some(tcp_si.state.to_string()),
            }),
            ProtocolSocketInfo::Udp(udp_si) => data.push(PortInfo {
                local_addr: udp_si.local_addr.to_string(),
                local_port: udp_si.local_port.to_string(),
                pids: si.associated_pids,
                tcp_state: None,
            }),
        }
    }
    data
}

#[tauri::command]
fn kill_process(_window: tauri::Window, pid: u32) -> bool {
    let pid = Pid::from_u32(pid);
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
        .invoke_handler(tauri::generate_handler![
            get_process_list,
            kill_process,
            get_port_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
