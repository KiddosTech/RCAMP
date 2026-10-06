use std::io::stdout;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, widgets::{Block, Borders, Paragraph}, Terminal};

#[derive(Parser)]
#[command(name = "rcamp", version, about = "RCAMP/CLI — local-first hardware control")]
struct Cli { #[command(subcommand)] command: Option<Command> }

#[derive(Subcommand)]
enum Command { Devices { #[command(subcommand)] command: Option<DevicesCommand> }, Version, Doctor }

#[derive(Subcommand)]
enum DevicesCommand { List, Discover, Info { device: String } }

fn main() -> Result<()> {
    match Cli::parse().command {
        None => tui(),
        Some(Command::Version) => { println!("RCAMP/CLI {}", env!("CARGO_PKG_VERSION")); Ok(()) }
        Some(Command::Doctor) => { println!("RCAMP/CLI doctor\nCore: available\nProfiles: local-first"); Ok(()) }
        Some(Command::Devices { command: None | Some(DevicesCommand::List) }) => { println!("No devices registered. Add a device profile to get started."); Ok(()) }
        Some(Command::Devices { command: Some(DevicesCommand::Discover) }) => { println!("Discovery adapters are not configured yet."); Ok(()) }
        Some(Command::Devices { command: Some(DevicesCommand::Info { device }) }) => { println!("No profile found for {device}."); Ok(()) }
    }
}

fn tui() -> Result<()> {
    enable_raw_mode()?;
    let mut output = stdout(); execute!(output, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(output); let mut terminal = Terminal::new(backend)?;
    loop {
        terminal.draw(|frame| frame.render_widget(Paragraph::new("No devices registered\n\nAdd a profile, then use discovery or connect.\n\n[q] Quit   [r] Refresh   [l] Logs")
            .block(Block::default().title(" RCAMP/CLI  •  Local-first device control ").borders(Borders::ALL)), frame.area()))?;
        if let Event::Key(key) = event::read()? { if key.code == KeyCode::Char('q') { break; } }
    }
    disable_raw_mode()?; execute!(terminal.backend_mut(), LeaveAlternateScreen)?; terminal.show_cursor()?;
    Ok(())
}
