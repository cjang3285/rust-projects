use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};
use crate::monitor::{
    cpu::CpuInfo, 
    memory::MemoryInfo, 
    disk::DiskInfo, 
    network::NetworkInfo,
    process::ProcessInfo,
};

pub fn render(
    f: &mut Frame, 
    cpu: &CpuInfo, 
    memory: &MemoryInfo,
    disks: &[DiskInfo],
    network: &[NetworkInfo],
    processes: &[ProcessInfo],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // CPU 전체
            Constraint::Length(3),  // 온도
            Constraint::Length(cpu.core_count as u16 + 2),  // 코어별
            Constraint::Length(3),  // 메모리
            Constraint::Length((disks.len() * 3) as u16 + 2),  // 디스크
            Constraint::Length((network.len() * 2) as u16 + 2), // 네트워크
            Constraint::Min(0),     // 프로세스
        ])
        .split(f.size());

    let mut idx = 0;
    render_cpu_total(f, chunks[idx], cpu);
    idx += 1;
    
    render_temperature(f, chunks[idx], cpu);
    idx += 1;
    
    render_cpu_cores(f, chunks[idx], cpu);
    idx += 1;
    
    render_memory(f, chunks[idx], memory);
    idx += 1;
    
    render_disks(f, chunks[idx], disks);
    idx += 1;
    
    render_network(f, chunks[idx], network);
    idx += 1;
    
    render_processes(f, chunks[idx], processes);
}

fn render_cpu_total(f: &mut Frame, area: Rect, cpu: &CpuInfo) {
    let gauge = Gauge::default()
        .block(Block::default().title("CPU Usage (Total)").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(cpu.usage as u16)
        .label(format!("{:.1}% ({} cores)", cpu.usage, cpu.core_count));
    
    f.render_widget(gauge, area);
}

fn render_temperature(f: &mut Frame, area: Rect, cpu: &CpuInfo) {
    let temp_text = if let Some(temp) = cpu.temperature {
        let color = if temp > 70.0 {
            Color::Red
        } else if temp > 60.0 {
            Color::Yellow
        } else {
            Color::Green
        };
        
        Paragraph::new(format!("🌡️  {:.1}°C", temp))
            .style(Style::default().fg(color))
            .block(Block::default().title("Temperature").borders(Borders::ALL))
    } else {
        Paragraph::new("N/A")
            .block(Block::default().title("Temperature").borders(Borders::ALL))
    };
    
    f.render_widget(temp_text, area);
}

fn render_cpu_cores(f: &mut Frame, area: Rect, cpu: &CpuInfo) {
    let items: Vec<ListItem> = cpu
        .core_usages
        .iter()
        .enumerate()
        .map(|(i, usage)| {
            let bar = create_bar(*usage as u16, 20);
            ListItem::new(format!("Core {}: {:>5.1}% {}", i, usage, bar))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().title("CPU Cores").borders(Borders::ALL));
    
    f.render_widget(list, area);
}

fn render_memory(f: &mut Frame, area: Rect, memory: &MemoryInfo) {
    let usage_percent = (memory.used as f64 / memory.total as f64 * 100.0) as u16;
    let gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(usage_percent)
        .label(format!("{:.2} GB / {:.2} GB ({:.1}%)", 
            memory.used as f64 / 1024.0 / 1024.0 / 1024.0,
            memory.total as f64 / 1024.0 / 1024.0 / 1024.0,
            usage_percent));
    
    f.render_widget(gauge, area);
}

fn render_disks(f: &mut Frame, area: Rect, disks: &[DiskInfo]) {
    let items: Vec<ListItem> = disks
        .iter()
        .flat_map(|disk| {
            let usage_percent = if disk.total > 0 {
                (disk.used as f64 / disk.total as f64 * 100.0) as u16
            } else {
                0
            };
            
            vec![
                ListItem::new(format!("📁 {} ({})", disk.name, disk.mount_point)),
                ListItem::new(format!("   {:.2} GB / {:.2} GB ({:.1}%)",
                    disk.used as f64 / 1024.0 / 1024.0 / 1024.0,
                    disk.total as f64 / 1024.0 / 1024.0 / 1024.0,
                    usage_percent)),
                ListItem::new(""),
            ]
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().title("Disks").borders(Borders::ALL));
    
    f.render_widget(list, area);
}

fn render_network(f: &mut Frame, area: Rect, network: &[NetworkInfo]) {
    let items: Vec<ListItem> = network
        .iter()
        .flat_map(|net| {
            vec![
                ListItem::new(format!("🌐 {}", net.interface)),
                ListItem::new(format!("   ↓ {:.2} MB  ↑ {:.2} MB",
                    net.received as f64 / 1024.0 / 1024.0,
                    net.transmitted as f64 / 1024.0 / 1024.0)),
            ]
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().title("Network").borders(Borders::ALL));
    
    f.render_widget(list, area);
}

fn render_processes(f: &mut Frame, area: Rect, processes: &[ProcessInfo]) {
    let items: Vec<ListItem> = processes
        .iter()
        .map(|proc| {
            ListItem::new(format!(
                "{:<8} {:<20} CPU: {:>5.1}%  MEM: {:>8} KB",
                proc.pid,
                truncate_string(&proc.name, 20),
                proc.cpu_usage,
                proc.memory / 1024
            ))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().title("Top Processes (by CPU)").borders(Borders::ALL));
    
    f.render_widget(list, area);
}

fn create_bar(percent: u16, width: usize) -> String {
    let filled = (percent as usize * width / 100).min(width);
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len-3])
    } else {
        s.to_string()
    }
}
