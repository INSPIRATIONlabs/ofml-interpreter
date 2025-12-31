//! Manufacturers list view

#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

#[cfg(feature = "tui")]
use crate::tui::app::App;
#[cfg(feature = "tui")]
use crate::tui::ui::render_search_bar;

/// Render the manufacturers list view
#[cfg(feature = "tui")]
pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = if app.search_active {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0)])
            .split(area)
    };

    let list_area = if app.search_active {
        render_search_bar(frame, app, chunks[0]);
        chunks[1]
    } else {
        chunks[0]
    };

    // Build list items - show ID and display name
    let items: Vec<ListItem> = if app.filtered_indices.is_empty() {
        app.manufacturers
            .iter()
            .map(|m| {
                let line = Line::from(vec![
                    Span::styled(
                        format!("{:<12}", m.id),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(&m.name, Style::default().fg(Color::Cyan)),
                ]);
                ListItem::new(line)
            })
            .collect()
    } else {
        app.filtered_indices
            .iter()
            .filter_map(|&i| app.manufacturers.get(i))
            .map(|m| {
                let line = Line::from(vec![
                    Span::styled(
                        format!("{:<12}", m.id),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(&m.name, Style::default().fg(Color::Cyan)),
                ]);
                ListItem::new(line)
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Hersteller ({}) ",
            if app.filtered_indices.is_empty() {
                app.manufacturers.len()
            } else {
                app.filtered_indices.len()
            }
        )))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, list_area, &mut app.manufacturer_list_state.clone());
}
