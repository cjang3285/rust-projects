mod monitor;
mod ui;
mod config;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use sysinfo::System;

use config::settings::Settings;
use monitor::{cpu, memory, disk, network, process};

fn main() -> Result<()> {
    let settings = Settings::new();
    let mut system = System::new_all();
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut system, settings);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    system: &mut System,
    settings: Settings,
) -> Result<()> {
    loop {
        system.refresh_all();
        
        let cpu_info = cpu::get_cpu_info(system);
        let memory_info = memory::get_memory_info(system);
        let disk_info = disk::get_disk_info();
        let network_info = network::get_network_info();
        let process_info = process::get_top_processes(system, 10);

        terminal.draw(|f| {
            ui::dashboard::render(
                f, 
                &cpu_info, 
                &memory_info,
                &disk_info,
                &network_info,
                &process_info,
            );
        })?;

        if event::poll(settings.update_interval)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
