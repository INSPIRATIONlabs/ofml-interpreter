//! Product Families list view
//!
//! Displays product families grouped by series, with configuration indicator.

#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

#[cfg(feature = "tui")]
use crate::oap::families::ProductFamily;
#[cfg(feature = "tui")]
use crate::tui::app::App;
#[cfg(feature = "tui")]
use crate::tui::ui::render_search_bar;

/// Render the product families list view
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

    let items: Vec<ListItem> = if app.filtered_indices.is_empty() {
        app.families
            .iter()
            .map(|f| render_family_item(f))
            .collect()
    } else {
        app.filtered_indices
            .iter()
            .filter_map(|&i| app.families.get(i))
            .map(|f| render_family_item(f))
            .collect()
    };

    let manufacturer_name = app
        .selected_manufacturer
        .as_ref()
        .map(|m| m.name.as_str())
        .unwrap_or("?");

    let configurable_count = app.families.iter().filter(|f| f.is_configurable).count();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " {} - Produktfamilien ({}, {} konfigurierbar) ",
            manufacturer_name,
            if app.filtered_indices.is_empty() {
                app.families.len()
            } else {
                app.filtered_indices.len()
            },
            configurable_count
        )))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, list_area, &mut app.family_list_state.clone());
}

/// Render a single product family item
#[cfg(feature = "tui")]
fn render_family_item(family: &ProductFamily) -> ListItem<'static> {
    // Indicator: ⚙ = has OCD properties (configurable options)
    //            ● = has OAM mapping (CLS geometry) but no OCD properties
    //            ○ = simple article (no configuration)
    let indicator = if !family.prop_classes.is_empty() {
        Span::styled("⚙", Style::default().fg(Color::Cyan))
    } else if family.is_configurable {
        Span::styled("●", Style::default().fg(Color::Yellow))
    } else {
        Span::styled("○", Style::default().fg(Color::DarkGray))
    };

    let variant_info = if family.variant_count > 1 {
        format!(" ({} Art.)", family.variant_count)
    } else {
        String::new()
    };

    let line = Line::from(vec![
        indicator,
        Span::raw(" "),
        Span::styled(
            format!("{:<12}", truncate_str(&family.id, 12)),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::raw(format!("{}{}", truncate_str(&family.name, 45), variant_info)),
    ]);

    ListItem::new(line)
}

/// Truncate a string to max length (UTF-8 safe)
fn truncate_str(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count > max_len {
        let truncated: String = s.chars().take(max_len - 1).collect();
        format!("{}…", truncated)
    } else {
        s.to_string()
    }
}
