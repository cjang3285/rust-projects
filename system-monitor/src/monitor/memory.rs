
use sysinfo::System;
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

pub fn get_memory_info(system: &System) -> MemoryInfo {
    let total = system.total_memory();
    let used = system.used_memory();
    let available = system.available_memory();
    
    MemoryInfo {
        total,
        used,
        available,
    }
}
