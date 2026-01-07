//! Articles list view


use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};


use crate::app::App;

use crate::ui::render_search_bar;

/// Render the articles list view

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

    // Group articles by series
    let items: Vec<ListItem> = if app.filtered_indices.is_empty() {
        app.articles
            .iter()
            .map(|a| {
                let config_indicator = if a.has_configuration {
                    Span::styled("●", Style::default().fg(Color::Green))
                } else {
                    Span::styled("○", Style::default().fg(Color::DarkGray))
                };

                let line = Line::from(vec![
                    config_indicator,
                    Span::raw(" "),
                    Span::styled(
                        format!("{:<24}", a.id),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" {}", a.short_description)),
                ]);
                ListItem::new(line)
            })
            .collect()
    } else {
        app.filtered_indices
            .iter()
            .filter_map(|&i| app.articles.get(i))
            .map(|a| {
                let config_indicator = if a.has_configuration {
                    Span::styled("●", Style::default().fg(Color::Green))
                } else {
                    Span::styled("○", Style::default().fg(Color::DarkGray))
                };

                let line = Line::from(vec![
                    config_indicator,
                    Span::raw(" "),
                    Span::styled(
                        format!("{:<24}", a.id),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" {}", a.short_description)),
                ]);
                ListItem::new(line)
            })
            .collect()
    };

    let manufacturer_name = app
        .selected_manufacturer
        .as_ref()
        .map(|m| m.name.as_str())
        .unwrap_or("?");

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " {} - Artikel ({}) ",
            manufacturer_name,
            if app.filtered_indices.is_empty() {
                app.articles.len()
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

    frame.render_stateful_widget(list, list_area, &mut app.article_list_state.clone());
}
