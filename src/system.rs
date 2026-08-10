use std::{collections::HashMap, path::PathBuf, sync::Arc};

use gpui::{Image, ImageFormat};
use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, get_sockets_info};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub exe: PathBuf,
    pub ports: Vec<u16>,
}

pub struct ProcessSource {
    system: System,
}

impl ProcessSource {
    pub fn new() -> Self {
        let refresh = RefreshKind::new().with_processes(ProcessRefreshKind::everything());
        let mut system = System::new_with_specifics(refresh);
        system.refresh_all();
        Self { system }
    }

    pub fn collect(&mut self) -> Vec<ProcessInfo> {
        self.system.refresh_all();
        let mut ports_by_pid: HashMap<u32, Vec<u16>> = HashMap::new();
        let af = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let protocols = ProtocolFlags::TCP | ProtocolFlags::UDP;
        if let Ok(sockets) = get_sockets_info(af, protocols) {
            for socket in sockets {
                let port = match socket.protocol_socket_info {
                    ProtocolSocketInfo::Tcp(info) => info.local_port,
                    ProtocolSocketInfo::Udp(info) => info.local_port,
                };
                for pid in socket.associated_pids {
                    ports_by_pid.entry(pid).or_default().push(port);
                }
            }
        }

        let mut processes: Vec<_> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let mut ports = ports_by_pid.remove(&pid.as_u32()).unwrap_or_default();
                ports.sort_unstable();
                ports.dedup();
                ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string_lossy().into_owned(),
                    cpu_usage: process.cpu_usage(),
                    memory_bytes: process.memory(),
                    exe: process
                        .exe()
                        .map_or_else(PathBuf::new, std::path::Path::to_path_buf),
                    ports,
                }
            })
            .collect();
        processes.sort_by_cached_key(|process| (process.name.to_lowercase(), process.pid));
        processes
    }
}

pub fn kill_process(pid: u32) -> bool {
    let system = System::new_all();
    system
        .process(Pid::from_u32(pid))
        .is_some_and(|process| process.kill())
}

pub fn matches_filter(process: &ProcessInfo, query: &str) -> bool {
    if let Some(port) = query.strip_prefix(':') {
        return process
            .ports
            .iter()
            .any(|value| value.to_string().contains(port));
    }
    let query = query.to_lowercase();
    process.name.to_lowercase().contains(&query) || process.pid.to_string().contains(&query)
}

pub fn format_memory(bytes: u64) -> String {
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * MIB;
    if bytes as f64 >= GIB {
        format!("{:.1} GB", bytes as f64 / GIB)
    } else {
        format!("{:.1} MB", bytes as f64 / MIB)
    }
}

#[cfg(target_os = "windows")]
pub fn extract_icon(path: &std::path::Path) -> Option<Arc<Image>> {
    use std::{os::windows::process::CommandExt, process::Command};

    use base64::{Engine as _, engine::general_purpose::STANDARD};

    if path.as_os_str().is_empty() {
        return None;
    }
    let helper = std::env::temp_dir().join("process-killer-file-icon.exe");
    if !helper.exists() {
        std::fs::write(
            &helper,
            include_bytes!("../vendor/file-icon-x86_64-pc-windows-msvc.exe"),
        )
        .ok()?;
    }
    let output = Command::new(helper)
        .arg(path)
        .creation_flags(0x0800_0000)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let encoded = std::str::from_utf8(&output.stdout).ok()?.trim();
    let bytes = STANDARD.decode(encoded).ok()?;
    Some(Arc::new(Image::from_bytes(ImageFormat::Png, bytes)))
}

#[cfg(not(target_os = "windows"))]
pub fn extract_icon(_: &std::path::Path) -> Option<Arc<Image>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process() -> ProcessInfo {
        ProcessInfo {
            pid: 4242,
            name: "DevServer.EXE".into(),
            cpu_usage: 1.0,
            memory_bytes: 128 * 1024 * 1024,
            exe: PathBuf::new(),
            ports: vec![3000, 5173],
        }
    }

    #[test]
    fn filters_by_name_pid_and_port() {
        let process = process();
        assert!(matches_filter(&process, "devserver"));
        assert!(matches_filter(&process, "424"));
        assert!(matches_filter(&process, ":173"));
        assert!(!matches_filter(&process, ":8080"));
        assert_eq!(format_memory(process.memory_bytes), "128.0 MB");
        assert_eq!(format_memory(3 * 1024 * 1024 * 1024), "3.0 GB");
    }
}
