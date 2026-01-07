//! Help screen view


use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};


use crate::app::App;

/// Render the help screen

pub fn render(frame: &mut Frame, _app: &App, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  OAP Konfigurator - Tastaturkürzel",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from("  Navigation:"),
        Line::from(vec![
            Span::styled("    ↑/↓       ", Style::default().fg(Color::Cyan)),
            Span::raw("In Liste navigieren"),
        ]),
        Line::from(vec![
            Span::styled("    PgUp/PgDn ", Style::default().fg(Color::Cyan)),
            Span::raw("10 Einträge vor/zurück"),
        ]),
        Line::from(vec![
            Span::styled("    Home/g    ", Style::default().fg(Color::Cyan)),
            Span::raw("Zum Anfang der Liste"),
        ]),
        Line::from(vec![
            Span::styled("    End/G     ", Style::default().fg(Color::Cyan)),
            Span::raw("Zum Ende der Liste"),
        ]),
        Line::from(vec![
            Span::styled("    Enter     ", Style::default().fg(Color::Cyan)),
            Span::raw("Auswählen / Öffnen"),
        ]),
        Line::from(vec![
            Span::styled("    Esc       ", Style::default().fg(Color::Cyan)),
            Span::raw("Zurück / Schließen"),
        ]),
        Line::from(vec![
            Span::styled("    h         ", Style::default().fg(Color::Cyan)),
            Span::raw("Zur Herstellerliste (Home)"),
        ]),
        Line::from(""),
        Line::from("  Aktionen:"),
        Line::from(vec![
            Span::styled("    /         ", Style::default().fg(Color::Cyan)),
            Span::raw("Suche starten"),
        ]),
        Line::from(vec![
            Span::styled("    t         ", Style::default().fg(Color::Cyan)),
            Span::raw("Tabellen-Browser öffnen"),
        ]),
        Line::from(vec![
            Span::styled("    e         ", Style::default().fg(Color::Cyan)),
            Span::raw("Konfiguration exportieren (JSON)"),
        ]),
        Line::from(vec![
            Span::styled("    Ctrl+G    ", Style::default().fg(Color::Cyan)),
            Span::raw("Geometrie exportieren (GLB)"),
        ]),
        Line::from(vec![
            Span::styled("    Ctrl+S    ", Style::default().fg(Color::Cyan)),
            Span::raw("Konfiguration speichern"),
        ]),
        Line::from(vec![
            Span::styled("    Ctrl+O    ", Style::default().fg(Color::Cyan)),
            Span::raw("Konfiguration laden"),
        ]),
        Line::from(vec![
            Span::styled("    f         ", Style::default().fg(Color::Cyan)),
            Span::raw("Favorit markieren/entfernen"),
        ]),
        Line::from(vec![
            Span::styled("    ?         ", Style::default().fg(Color::Cyan)),
            Span::raw("Diese Hilfe anzeigen"),
        ]),
        Line::from(vec![
            Span::styled("    T         ", Style::default().fg(Color::Cyan)),
            Span::raw("Theme umschalten (Hell/Dunkel)"),
        ]),
        Line::from(vec![
            Span::styled("    F12       ", Style::default().fg(Color::Cyan)),
            Span::raw("Debug-Konsole ein/aus"),
        ]),
        Line::from(vec![
            Span::styled("    q         ", Style::default().fg(Color::Cyan)),
            Span::raw("Programm beenden"),
        ]),
        Line::from(""),
        Line::from("  Eigenschaftseingabe:"),
        Line::from(vec![
            Span::styled("    ←/→       ", Style::default().fg(Color::Cyan)),
            Span::raw("Auswahl ändern (bei Auswahllisten)"),
        ]),
        Line::from(vec![
            Span::styled("    Space     ", Style::default().fg(Color::Cyan)),
            Span::raw("Boolean umschalten"),
        ]),
        Line::from(vec![
            Span::styled("    Ctrl+Z/u  ", Style::default().fg(Color::Cyan)),
            Span::raw("Letzte Änderung rückgängig"),
        ]),
        Line::from(vec![
            Span::styled("    Ctrl+Y    ", Style::default().fg(Color::Cyan)),
            Span::raw("Änderung wiederherstellen"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Drücke Esc um zur Anwendung zurückzukehren",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let help =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title(" Hilfe "));

    frame.render_widget(help, area);
}
