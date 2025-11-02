use sysinfo::System;
use std::fs;

pub struct CpuInfo {
    pub usage: f32,
    pub core_count: usize,
    pub core_usages: Vec<f32>,
    pub temperature: Option<f32>,
}

pub fn get_cpu_info(system: &System) -> CpuInfo {
    let usage = system.global_cpu_info().cpu_usage();
    let core_count = system.cpus().len();
    let core_usages: Vec<f32> = system
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect();
    
    let temperature = read_cpu_temperature();
    
    CpuInfo {
        usage,
        core_count,
        core_usages,
        temperature,
    }
}

fn read_cpu_temperature() -> Option<f32> {
    // 라즈베리파이 온도 읽기
    if let Ok(temp_str) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(temp) = temp_str.trim().parse::<f32>() {
            return Some(temp / 1000.0); // millidegrees to degrees
        }
    }
    None
}
