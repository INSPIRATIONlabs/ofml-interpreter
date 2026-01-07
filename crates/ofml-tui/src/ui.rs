//! TUI rendering functions
//!
//! This module contains the main render loop and layout logic for the TUI.


use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};


use super::app::{App, Screen};

use super::views::{
    articles, catalog, families, family_config, help, manufacturers, properties, tables,
};

/// Main render function that dispatches to the appropriate view

pub fn render(frame: &mut Frame, app: &App) {
    let theme = app.theme();

    // Set background for light theme
    if app.theme_variant == super::theme::ThemeVariant::Light {
        let background = Block::default().style(Style::default().bg(theme.background));
        frame.render_widget(background, frame.area());
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer/help
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);

    // Split content area for debug panel if enabled
    let content_area = if app.debug_mode {
        let debug_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(65), // Main content
                Constraint::Percentage(35), // Debug panel
            ])
            .split(chunks[1]);

        render_debug_panel(frame, app, debug_chunks[1]);
        debug_chunks[0]
    } else {
        chunks[1]
    };

    match app.screen {
        Screen::Manufacturers => manufacturers::render(frame, app, content_area),
        Screen::Catalog => catalog::render(frame, app, content_area),
        Screen::Families => families::render(frame, app, content_area),
        Screen::FamilyConfig => family_config::render(frame, app, content_area),
        Screen::Articles => articles::render(frame, app, content_area),
        Screen::Properties => properties::render(frame, app, content_area),
        Screen::Tables => tables::render_tables(frame, app, content_area),
        Screen::TableView => tables::render_table_view(frame, app, content_area),
        Screen::SavedConfigs => render_saved_configs(frame, app, content_area),
        Screen::Help => help::render(frame, app, content_area),
    }

    render_footer(frame, app, chunks[2]);
}

/// Render the debug panel

fn render_debug_panel(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = app
        .debug_log
        .iter()
        .take(area.height.saturating_sub(2) as usize)
        .map(|msg| {
            // Color code different types of messages
            let style = if msg.contains("Property changed") {
                Style::default().fg(Color::Cyan)
            } else if msg.contains("Price:") {
                Style::default().fg(Color::Green)
            } else if msg.contains("+ ") {
                Style::default().fg(Color::Yellow)
            } else if msg.contains("Error") || msg.contains("None") {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(Span::styled(msg.as_str(), style))
        })
        .collect();

    let debug_panel = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Debug Console (F12) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(debug_panel, area);
}

/// Render the saved configurations list

fn render_saved_configs(frame: &mut Frame, app: &App, area: Rect) {
    if app.saved_configs.is_empty() {
        let message = Paragraph::new("Keine gespeicherten Konfigurationen gefunden.\n\nDrücken Sie Ctrl+S im Konfigurationsbildschirm, um die aktuelle Konfiguration zu speichern.")
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Gespeicherte Konfigurationen "),
            );
        frame.render_widget(message, area);
        return;
    }

    let items: Vec<ListItem> = app
        .saved_configs
        .iter()
        .enumerate()
        .map(|(i, (path, config))| {
            let is_selected = app.saved_configs_list_state.selected() == Some(i);
            let style = if is_selected {
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            // Format the config info
            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            let line1 = format!(
                "{} - {} ({})",
                config.manufacturer.to_uppercase(),
                config.article_nr,
                config.variant_code
            );
            let line2 = format!("  {} | {}", config.saved_at, filename);

            ListItem::new(vec![
                Line::from(Span::styled(line1, style)),
                Line::from(Span::styled(
                    line2,
                    if is_selected {
                        style
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                )),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Gespeicherte Konfigurationen "),
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_stateful_widget(list, area, &mut app.saved_configs_list_state.clone());
}

/// Render the header bar

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
        Screen::SavedConfigs => {
            format!(
                "OAP Konfigurator - Gespeicherte Konfigurationen ({} vorhanden)",
                app.saved_configs.len()
            )
        }
        Screen::Help => "OAP Konfigurator - Hilfe".to_string(),
    };

    let theme = app.theme();
    let header = Paragraph::new(title)
        .style(theme.title())
        .block(Block::default().borders(Borders::BOTTOM).border_style(theme.border_style()));

    frame.render_widget(header, area);
}

/// Render the footer with keybindings

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
            Span::raw("f: "),
            Span::styled("★Favorit", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Ctrl+S: "),
            Span::styled("Speichern", Style::default().fg(Color::Yellow)),
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
        Screen::SavedConfigs => vec![
            Span::raw("↑↓: "),
            Span::styled("Navigation", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Enter: "),
            Span::styled("Laden", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("Del: "),
            Span::styled("Löschen", Style::default().fg(Color::Red)),
            Span::raw(" | "),
            Span::raw("Esc: "),
            Span::styled("Zurück", Style::default().fg(Color::Yellow)),
        ],
        Screen::Help => vec![
            Span::raw("Esc: "),
            Span::styled("Zurück", Style::default().fg(Color::Yellow)),
        ],
    };

    let theme = app.theme();
    let help_text = Paragraph::new(Line::from(keybindings))
        .style(theme.text())
        .block(Block::default().borders(Borders::TOP).border_style(theme.border_style()));

    frame.render_widget(help_text, chunks[0]);

    // Status message
    let status = if let Some(ref msg) = app.status_message {
        Paragraph::new(msg.as_str())
            .style(theme.text_success())
            .block(Block::default().borders(Borders::TOP).border_style(theme.border_style()))
    } else {
        Paragraph::new("").block(Block::default().borders(Borders::TOP).border_style(theme.border_style()))
    };

    frame.render_widget(status, chunks[1]);
}

/// Render a search bar

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
