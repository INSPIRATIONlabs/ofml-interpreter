//! Properties configuration view

#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[cfg(feature = "tui")]
use crate::oap::format_german_price_with_currency;
#[cfg(feature = "tui")]
use crate::property::{PropertyState, PropertyType};
#[cfg(feature = "tui")]
use crate::tui::app::App;

/// Render the properties configuration view
#[cfg(feature = "tui")]
pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // Split into left (properties) and right (summary/price)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    render_property_list(frame, app, chunks[0]);
    render_summary(frame, app, chunks[1]);
}

/// Render the property list
#[cfg(feature = "tui")]
fn render_property_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = if let Some(ref config) = app.configuration {
        let mut props: Vec<_> = config.properties.definitions.iter().collect();
        props.sort_by_key(|(_, def)| def.sort_order);

        props
            .iter()
            .enumerate()
            .map(|(i, (name, def))| {
                let value = config
                    .properties
                    .values
                    .get(*name)
                    .map(|v| format!("{:?}", v))
                    .unwrap_or_else(|| "-".to_string());

                let type_info = match &def.prop_type {
                    PropertyType::Int { min, max } => {
                        let min_str = min.map(|v| v.to_string()).unwrap_or_default();
                        let max_str = max.map(|v| v.to_string()).unwrap_or_default();
                        format!("[{}-{}]", min_str, max_str)
                    }
                    PropertyType::Float { min, max } => {
                        let min_str = min.map(|v| format!("{:.1}", v)).unwrap_or_default();
                        let max_str = max.map(|v| format!("{:.1}", v)).unwrap_or_default();
                        format!("[{}-{}]", min_str, max_str)
                    }
                    PropertyType::Choice { options } => {
                        format!("[{}]", options.join(","))
                    }
                    PropertyType::Bool => "[ja/nein]".to_string(),
                    PropertyType::String => "[Text]".to_string(),
                };

                let state_style = match def.state {
                    PropertyState::Enabled => Style::default(),
                    PropertyState::Hidden => Style::default().fg(Color::DarkGray),
                    PropertyState::ReadOnly => Style::default().fg(Color::Yellow),
                };

                let is_focused = i == app.focused_property;
                let prefix = if is_focused { "▶ " } else { "  " };

                let line = Line::from(vec![
                    Span::raw(prefix),
                    Span::styled(
                        format!("{:<16}", def.label),
                        if is_focused {
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            state_style
                        },
                    ),
                    Span::raw(" = "),
                    Span::styled(format!("{:<16}", value), Style::default().fg(Color::Cyan)),
                    Span::styled(type_info, Style::default().fg(Color::DarkGray)),
                ]);

                if is_focused {
                    ListItem::new(line).style(Style::default().bg(Color::DarkGray))
                } else {
                    ListItem::new(line)
                }
            })
            .collect()
    } else {
        vec![ListItem::new("Keine Eigenschaften verfügbar")]
    };

    let article_name = app
        .selected_article
        .as_ref()
        .map(|a| a.id.as_str())
        .unwrap_or("?");

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} - Eigenschaften ", article_name)),
    );

    frame.render_widget(list, area);
}

/// Render the summary panel with variant code and price
#[cfg(feature = "tui")]
fn render_summary(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Variant code
            Constraint::Min(0),    // Price info
        ])
        .split(area);

    // Variant code
    let variant_code = app
        .configuration
        .as_ref()
        .map(|c| c.variant_code.as_str())
        .unwrap_or("-");

    let variant_block = Paragraph::new(vec![
        Line::from(Span::styled(
            "Variantencode:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(variant_code, Style::default().fg(Color::Cyan))),
    ])
    .block(Block::default().borders(Borders::ALL).title(" Code "));

    frame.render_widget(variant_block, chunks[0]);

    // Price info
    let price_lines =
        if let Some(ref price) = app.configuration.as_ref().and_then(|c| c.price.as_ref()) {
            let mut lines = vec![Line::from(vec![
                Span::styled(
                    "Grundpreis:  ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format_german_price_with_currency(price.base_price, &price.currency),
                    Style::default().fg(Color::White),
                ),
            ])];

            for surcharge in &price.surcharges {
                let amount_str = if surcharge.is_percentage {
                    format!("{}%", surcharge.amount)
                } else {
                    format_german_price_with_currency(surcharge.amount, &price.currency)
                };
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  + {}:  ", surcharge.name),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(amount_str, Style::default().fg(Color::Yellow)),
                ]));
            }

            lines.push(Line::from(vec![
                Span::styled(
                    "Gesamtpreis: ",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Green),
                ),
                Span::styled(
                    format_german_price_with_currency(price.total_price, &price.currency),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Preisdatum:  ", Style::default().fg(Color::DarkGray)),
                Span::raw(price.price_date.format("%d.%m.%Y").to_string()),
            ]));

            lines
        } else {
            vec![Line::from(Span::styled(
                "Preis nicht verfügbar",
                Style::default().fg(Color::Red),
            ))]
        };

    let price_block =
        Paragraph::new(price_lines).block(Block::default().borders(Borders::ALL).title(" Preis "));

    frame.render_widget(price_block, chunks[1]);
}
