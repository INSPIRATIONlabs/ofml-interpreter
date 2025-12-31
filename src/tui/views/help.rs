//! Help screen view

#[cfg(feature = "tui")]
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[cfg(feature = "tui")]
use crate::tui::app::App;

/// Render the help screen
#[cfg(feature = "tui")]
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
            Span::styled("    Enter     ", Style::default().fg(Color::Cyan)),
            Span::raw("Auswählen / Öffnen"),
        ]),
        Line::from(vec![
            Span::styled("    Esc       ", Style::default().fg(Color::Cyan)),
            Span::raw("Zurück / Schließen"),
        ]),
        Line::from(vec![
            Span::styled("    Tab       ", Style::default().fg(Color::Cyan)),
            Span::raw("Nächste Eigenschaft"),
        ]),
        Line::from(vec![
            Span::styled("    Shift+Tab ", Style::default().fg(Color::Cyan)),
            Span::raw("Vorherige Eigenschaft"),
        ]),
        Line::from(""),
        Line::from("  Aktionen:"),
        Line::from(vec![
            Span::styled("    /         ", Style::default().fg(Color::Cyan)),
            Span::raw("Suche starten"),
        ]),
        Line::from(vec![
            Span::styled("    d         ", Style::default().fg(Color::Cyan)),
            Span::raw("Preisdatum setzen"),
        ]),
        Line::from(vec![
            Span::styled("    e         ", Style::default().fg(Color::Cyan)),
            Span::raw("Konfiguration exportieren"),
        ]),
        Line::from(vec![
            Span::styled("    ?         ", Style::default().fg(Color::Cyan)),
            Span::raw("Diese Hilfe anzeigen"),
        ]),
        Line::from(vec![
            Span::styled("    q         ", Style::default().fg(Color::Cyan)),
            Span::raw("Programm beenden"),
        ]),
        Line::from(""),
        Line::from("  Eigenschaftseingabe:"),
        Line::from(vec![
            Span::styled("    0-9       ", Style::default().fg(Color::Cyan)),
            Span::raw("Numerischen Wert eingeben"),
        ]),
        Line::from(vec![
            Span::styled("    ←/→       ", Style::default().fg(Color::Cyan)),
            Span::raw("Auswahl ändern (bei Auswahllisten)"),
        ]),
        Line::from(vec![
            Span::styled("    Space     ", Style::default().fg(Color::Cyan)),
            Span::raw("Boolean umschalten"),
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
