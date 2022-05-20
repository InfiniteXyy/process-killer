use serde::Serialize;

#[derive(Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub parent_pid: Option<u32>,
    pub exe: String,
}

#[derive(Serialize)]
pub struct PortInfo {
    pub local_addr: String,
    pub local_port: String,
    pub pids: Vec<u32>,
    pub tcp_state: Option<String>,
}
