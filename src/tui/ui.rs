//! TUI rendering functions
//!
//! This module contains the main render loop and layout logic for the TUI.

#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[cfg(feature = "tui")]
use super::app::{App, Screen};
#[cfg(feature = "tui")]
use super::views::{articles, catalog, families, family_config, help, manufacturers, properties, tables};

/// Main render function that dispatches to the appropriate view
#[cfg(feature = "tui")]
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer/help
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);

    match app.screen {
        Screen::Manufacturers => manufacturers::render(frame, app, chunks[1]),
        Screen::Catalog => catalog::render(frame, app, chunks[1]),
        Screen::Families => families::render(frame, app, chunks[1]),
        Screen::FamilyConfig => family_config::render(frame, app, chunks[1]),
        Screen::Articles => articles::render(frame, app, chunks[1]),
        Screen::Properties => properties::render(frame, app, chunks[1]),
        Screen::Tables => tables::render_tables(frame, app, chunks[1]),
        Screen::TableView => tables::render_table_view(frame, app, chunks[1]),
        Screen::Help => help::render(frame, app, chunks[1]),
    }

    render_footer(frame, app, chunks[2]);
}

/// Render the header bar
#[cfg(feature = "tui")]
fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let title = match app.screen {
        Screen::Manufacturers => "OAP Konfigurator - Hersteller".to_string(),
        Screen::Catalog => {
            if let Some(ref m) = app.selected_manufacturer {
                format!("OAP Konfigurator - {} - Katalog", m.name)
            } else {
                "OAP Konfigurator - Katalog".to_string()
            }
        }
        Screen::Families => {
            if let Some(ref m) = app.selected_manufacturer {
                format!("OAP Konfigurator - {} - Produktfamilien", m.name)
            } else {
                "OAP Konfigurator - Produktfamilien".to_string()
            }
        }
        Screen::FamilyConfig => {
            if let Some(ref f) = app.selected_family {
                format!("OAP Konfigurator - {} - Konfiguration", f.name)
            } else {
                "OAP Konfigurator - Konfiguration".to_string()
            }
        }
        Screen::Articles => {
            if let Some(ref m) = app.selected_manufacturer {
                format!("OAP Konfigurator - {} - Artikel", m.name)
            } else {
                "OAP Konfigurator - Artikel".to_string()
            }
        }
        Screen::Properties => {
            if let Some(ref a) = app.selected_article {
                format!("OAP Konfigurator - {} - Konfiguration", a.id)
            } else {
                "OAP Konfigurator - Konfiguration".to_string()
            }
        }
        Screen::Tables => {
            if let Some(ref m) = app.selected_manufacturer {
                format!("OAP Konfigurator - {} - Tabellen", m.name)
            } else {
                "OAP Konfigurator - Tabellen".to_string()
            }
        }
        Screen::TableView => {
            if let Some(ref t) = app.selected_table {
                format!("OAP Konfigurator - Tabelle: {}", t.name)
            } else {
                "OAP Konfigurator - Tabelle".to_string()
            }
        }
        Screen::Help => "OAP Konfigurator - Hilfe".to_string(),
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::White).bg(Color::Blue))
        .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(header, area);
}

/// Render the footer with keybindings
#[cfg(feature = "tui")]
fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    // Keybindings
    let keybindings = match app.screen {
        Screen::Manufacturers | Screen::Catalog | Screen::Families | Screen::Articles => {
            if app.search_active {
                vec![
                    Span::raw("Esc: "),
                    Span::styled("Suche beenden", Style::default().fg(Color::Yellow)),
                    Span::raw(" | "),
                    Span::raw("Enter: "),
                    Span::styled("Auswählen", Style::default().fg(Color::Yellow)),
                ]
            } else {
                vec![
                    Span::raw("↑↓: "),
                    Span::styled("Navigation", Style::default().fg(Color::Yellow)),
                    Span::raw(" | "),
                    Span::raw("Enter: "),
                    Span::styled("Auswählen", Style::default().fg(Color::Yellow)),
                    Span::raw(" | "),
                    Span::raw("/: "),
                    Span::styled("Suchen", Style::default().fg(Color::Yellow)),
                    Span::raw(" | "),
                    Span::raw("q: "),
                    Span::styled("Beenden", Style::default().fg(Color::Yellow)),
                ]
            }
        }
        Screen::FamilyConfig => vec![
            Span::raw("↑↓: "),
            Span::styled("Eigenschaft", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("←→: "),
            Span::styled("Wert ändern", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("e: "),
            Span::styled("Export", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Esc: "),
            Span::styled("Zurück", Style::default().fg(Color::Yellow)),
        ],
        Screen::Properties => vec![
            Span::raw("↑↓: "),
            Span::styled("Navigation", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Tab: "),
            Span::styled("Nächste", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("e: "),
            Span::styled("Export", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Esc: "),
            Span::styled("Zurück", Style::default().fg(Color::Yellow)),
        ],
        Screen::Tables => vec![
            Span::raw("↑↓: "),
            Span::styled("Navigation", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Enter: "),
            Span::styled("Öffnen", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("/: "),
            Span::styled("Suchen", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Esc: "),
            Span::styled("Zurück", Style::default().fg(Color::Yellow)),
        ],
        Screen::TableView => vec![
            Span::raw("↑↓: "),
            Span::styled("Zeile", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("←→: "),
            Span::styled("Spalten scrollen", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Esc: "),
            Span::styled("Zurück", Style::default().fg(Color::Yellow)),
        ],
        Screen::Help => vec![
            Span::raw("Esc: "),
            Span::styled("Zurück", Style::default().fg(Color::Yellow)),
        ],
    };

    let help_text = Paragraph::new(Line::from(keybindings))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(help_text, chunks[0]);

    // Status message
    let status = if let Some(ref msg) = app.status_message {
        Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::TOP))
    } else {
        Paragraph::new("").block(Block::default().borders(Borders::TOP))
    };

    frame.render_widget(status, chunks[1]);
}

/// Render a search bar
#[cfg(feature = "tui")]
pub fn render_search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let search_text = format!("Suche: {}", app.search_query);
    let search = Paragraph::new(search_text)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL).title("Suche"));

    frame.render_widget(search, area);
}
