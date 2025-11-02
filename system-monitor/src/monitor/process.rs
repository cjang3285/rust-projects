use sysinfo::System;

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

pub fn get_top_processes(system: &System, limit: usize) -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = system
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
        })
        .collect();
    
    // CPU 사용률 기준 정렬
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    processes.truncate(limit);
    
    processes
}
