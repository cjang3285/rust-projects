use sysinfo::{System, Cpu};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Gauge},
    layout::{Layout, Constraint, Direction},
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tokio::time;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;               // 1️⃣ 터미널을 Raw 모드로 전환
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; // 2️⃣ 별도 화면 버퍼로 전환
    // ... 여기에 UI 코드 올 예정 ...
    disable_raw_mode()?;              // 3️⃣ 프로그램 종료 시 Raw 모드 해제
    execute!(stdout, LeaveAlternateScreen)?; // 4️⃣ 원래 화면으로 복귀
    Ok(())
}
