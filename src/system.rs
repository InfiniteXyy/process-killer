use std::{collections::HashMap, path::PathBuf, sync::Arc};

use gpui::{Image, ImageFormat};
use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, get_sockets_info};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortColumn {
    Process,
    Ports,
    Cpu,
    Memory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn reversed(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

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
    if let Some(port) = query.strip_prefix(':').or_else(|| query.strip_prefix('：')) {
        return process
            .ports
            .iter()
            .any(|value| value.to_string().contains(port));
    }
    let query = query.to_lowercase();
    process.name.to_lowercase().contains(&query) || process.pid.to_string().contains(&query)
}

pub fn sort_processes(processes: &mut [ProcessInfo], column: SortColumn, direction: SortDirection) {
    use std::cmp::Ordering;

    processes.sort_by(|left, right| {
        let order = match column {
            SortColumn::Process => left.name.to_lowercase().cmp(&right.name.to_lowercase()),
            SortColumn::Ports => match (left.ports.first(), right.ports.first()) {
                (Some(left), Some(right)) => left.cmp(right),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            SortColumn::Cpu => left.cpu_usage.total_cmp(&right.cpu_usage),
            SortColumn::Memory => left.memory_bytes.cmp(&right.memory_bytes),
        };
        let order = if direction == SortDirection::Descending
            && !(column == SortColumn::Ports && (left.ports.is_empty() || right.ports.is_empty()))
        {
            order.reverse()
        } else {
            order
        };
        order
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
            .then_with(|| left.pid.cmp(&right.pid))
    });
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
        assert!(matches_filter(&process, "：173"));
        assert!(!matches_filter(&process, ":8080"));
        assert_eq!(format_memory(process.memory_bytes), "128.0 MB");
        assert_eq!(format_memory(3 * 1024 * 1024 * 1024), "3.0 GB");
    }

    #[test]
    fn sorts_columns_and_keeps_missing_ports_last() {
        let mut first = process();
        first.name = "Alpha".into();
        first.cpu_usage = 8.0;
        first.memory_bytes = 300;
        first.ports = vec![5173];

        let mut second = process();
        second.pid = 7;
        second.name = "Beta".into();
        second.cpu_usage = 2.0;
        second.memory_bytes = 900;
        second.ports.clear();

        let mut processes = vec![first, second];
        sort_processes(&mut processes, SortColumn::Cpu, SortDirection::Ascending);
        assert_eq!(processes[0].name, "Beta");

        sort_processes(
            &mut processes,
            SortColumn::Memory,
            SortDirection::Descending,
        );
        assert_eq!(processes[0].name, "Beta");

        sort_processes(&mut processes, SortColumn::Ports, SortDirection::Descending);
        assert_eq!(processes[0].name, "Alpha");
    }
}
