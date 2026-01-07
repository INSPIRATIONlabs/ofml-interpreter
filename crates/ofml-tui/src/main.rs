//! OFML TUI - Terminal User Interface for OFML Product Configuration
//!
//! A terminal-based product configurator for browsing and configuring
//! OFML (Office Furniture Modeling Language) product data.

use std::io;
use std::path::Path;

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::manufacturers;

mod runner;

/// OFML TUI - Terminal Product Configurator
#[derive(Parser)]
#[command(name = "ofml-tui")]
#[command(author, version, about = "Terminal UI for OFML product configuration")]
struct Cli {
    /// Path to OFML data directory
    data_path: String,

    /// Price date (YYYY-MM-DD)
    #[arg(short = 'd', long)]
    price_date: Option<String>,

    /// Increase output verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Setup tracing
    let filter = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)))
        .init();

    // Run the TUI
    run_tui(&cli.data_path, cli.price_date.as_deref())?;

    Ok(())
}

fn run_tui(data_path: &str, price_date_str: Option<&str>) -> Result<(), String> {
    let path = Path::new(data_path);
    if !path.exists() {
        return Err(format!("Data path not found: {}", data_path));
    }

    // Parse price date
    let price_date = match price_date_str {
        Some(s) => chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| format!("Invalid date format: {}. Use YYYY-MM-DD", s))?,
        None => chrono::Local::now().date_naive(),
    };

    // Setup terminal
    enable_raw_mode().map_err(|e| e.to_string())?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).map_err(|e| e.to_string())?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| e.to_string())?;

    // Create app and load manufacturers
    let mut app = ofml_tui::App::new(data_path.to_string());
    app.price_date = price_date;

    // Create configuration engine
    let mut engine = ConfigurationEngine::new(path);

    // Initialize manufacturer names from Manufacturers.ebase
    manufacturers::init_from_data_path(path);

    // Load installed manufacturers from SQLite (fast)
    let installed = manufacturers::get_installed_manufacturers(path);
    for mfr in installed {
        app.manufacturers.push(ofml_lib::oap::Manufacturer {
            id: mfr.id,
            name: mfr.name,
            path: mfr.path,
        });
    }

    // Main loop
    let result = runner::run_event_loop(&mut terminal, &mut app, &mut engine);

    // Restore terminal
    disable_raw_mode().map_err(|e| e.to_string())?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(|e| e.to_string())?;
    terminal.show_cursor().map_err(|e| e.to_string())?;

    result
}
