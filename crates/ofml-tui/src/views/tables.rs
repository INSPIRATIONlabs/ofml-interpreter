//! Table browser view for inspecting custom manufacturer tables
//!
//! This view allows browsing all tables in pdata.ebase files,
//! including standard OCD tables and custom manufacturer tables.


use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Frame,
};


use crate::app::{App, TableInfo};

use crate::ui::render_search_bar;

/// Standard OCD table names (for highlighting custom tables)
#[allow(dead_code)]
const _STANDARD_TABLES: &[&str] = &[
    "ocd_article",
    "ocd_articletext",
    "ocd_artshorttext",
    "ocd_artlongtext",
    "ocd_price",
    "ocd_pricetext",
    "ocd_property",
    "ocd_propertyclass",
    "ocd_propertyvalue",
    "ocd_propertyvaluetext",
    "ocd_propvaluetext",
    "ocd_variantcondition",
    "ocd_relation",
    "ocd_relationobj",
    "ocd_propertygroup",
    "ocd_article2propgroup",
    "ocd_composite",
    "ocd_billofitems",
    "propvalue2varcond",
];

/// Check if a table name is a standard OCD table
#[allow(dead_code)]
fn _is_standard_table(name: &str) -> bool {
    _STANDARD_TABLES
        .iter()
        .any(|&s| s.eq_ignore_ascii_case(name))
}

/// Render the table browser view (list of tables)

pub fn render_tables(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = if app.search_active {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search bar
                Constraint::Min(0),    // List
            ])
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

    // Count standard vs custom tables
    let custom_count = app.tables.iter().filter(|t| !t.is_standard).count();
    let standard_count = app.tables.len() - custom_count;

    let title = format!(
        " Tabellen ({} Standard, {} Custom) - 't' zum Öffnen ",
        standard_count, custom_count
    );

    let items: Vec<ListItem> = if app.filtered_indices.is_empty() {
        app.tables.iter().map(render_table_item).collect()
    } else {
        app.filtered_indices
            .iter()
            .filter_map(|&i| app.tables.get(i))
            .map(render_table_item)
            .collect()
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, list_area, &mut app.table_list_state.clone());
}

/// Render a single table item in the list

fn render_table_item(table: &TableInfo) -> ListItem<'static> {
    let (icon, style) = if table.is_standard {
        ("[S]", Style::default().fg(Color::DarkGray))
    } else {
        (
            "[C]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    };

    let cols_preview = if table.columns.len() > 3 {
        format!(
            "{}, {} +{}",
            table.columns.first().map(|s| s.as_str()).unwrap_or(""),
            table.columns.get(1).map(|s| s.as_str()).unwrap_or(""),
            table.columns.len() - 2
        )
    } else {
        table.columns.join(", ")
    };

    let line = Line::from(vec![
        Span::styled(icon, style),
        Span::raw(" "),
        Span::styled(
            table.name.clone(),
            if table.is_standard {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::BOLD)
            },
        ),
        Span::styled(
            format!(" ({} rows)", table.row_count),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            format!(" [{}]", cols_preview),
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    ListItem::new(line)
}

/// Render the table view (contents of selected table)

pub fn render_table_view(frame: &mut Frame, app: &App, area: Rect) {
    let table_info = match &app.selected_table {
        Some(t) => t,
        None => {
            let msg = Paragraph::new("Keine Tabelle ausgewählt")
                .block(Block::default().borders(Borders::ALL).title(" Tabelle "));
            frame.render_widget(msg, area);
            return;
        }
    };

    // Layout: header info + table contents
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header info
            Constraint::Min(0),    // Table contents
        ])
        .split(area);

    // Header info
    let header_text = vec![
        Line::from(vec![
            Span::styled("Tabelle: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                &table_info.name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(
                    " ({} Zeilen, {} Spalten)",
                    table_info.row_count,
                    table_info.columns.len()
                ),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("Spalten: ", Style::default().fg(Color::Cyan)),
            Span::raw(table_info.columns.join(", ")),
        ]),
        Line::from(vec![Span::styled(
            "←→: Scroll | ↑↓: Zeile | Esc: Zurück",
            Style::default().fg(Color::DarkGray),
        )]),
    ];
    let header =
        Paragraph::new(header_text).block(Block::default().borders(Borders::ALL).title(" Info "));
    frame.render_widget(header, chunks[0]);

    // Table contents
    if app.table_rows.is_empty() {
        let msg = Paragraph::new("Keine Daten in dieser Tabelle")
            .block(Block::default().borders(Borders::ALL).title(" Daten "));
        frame.render_widget(msg, chunks[1]);
        return;
    }

    // Calculate column widths based on content
    let col_count = table_info.columns.len();
    let visible_cols = col_count.saturating_sub(app.table_scroll_x);

    if visible_cols == 0 {
        let msg = Paragraph::new("Scroll zurück mit ←")
            .block(Block::default().borders(Borders::ALL).title(" Daten "));
        frame.render_widget(msg, chunks[1]);
        return;
    }

    // Build header row
    let header_cells: Vec<Cell> = table_info
        .columns
        .iter()
        .skip(app.table_scroll_x)
        .take(10) // Show max 10 columns
        .map(|c| {
            Cell::from(c.clone()).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        })
        .collect();

    let header_row = Row::new(header_cells).height(1);

    // Build data rows
    let rows: Vec<Row> = app
        .table_rows
        .iter()
        .map(|row| {
            let cells: Vec<Cell> = row
                .values
                .iter()
                .skip(app.table_scroll_x)
                .take(10)
                .map(|v| {
                    let display = if v.len() > 30 {
                        format!("{}...", &v[..27])
                    } else {
                        v.clone()
                    };
                    Cell::from(display)
                })
                .collect();
            Row::new(cells)
        })
        .collect();

    // Calculate widths for visible columns
    let widths: Vec<Constraint> = (0..visible_cols.min(10))
        .map(|_| Constraint::Min(15))
        .collect();

    let scroll_info = if app.table_scroll_x > 0 {
        format!(" (Spalte {}+) ", app.table_scroll_x + 1)
    } else {
        String::new()
    };

    let table = Table::new(rows, widths)
        .header(header_row)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Daten{} ", scroll_info)),
        )
        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    // Note: Table widget doesn't support stateful rendering, so we use plain render
    // The row selection is handled by scrolling
    frame.render_widget(table, chunks[1]);
}
