//! Family Configuration view
//!
//! Displays configurable properties for a product family with dropdown selection.

#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[cfg(feature = "tui")]
use crate::oap::families::{FamilyProperty, PropertyType};
#[cfg(feature = "tui")]
use crate::oap::format_german_price_with_currency;
#[cfg(feature = "tui")]
use crate::tui::app::App;

/// Render the family configuration view
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

/// Render the property list with dropdown-style selection
#[cfg(feature = "tui")]
fn render_property_list(frame: &mut Frame, app: &App, area: Rect) {
    let family_name = app
        .selected_family
        .as_ref()
        .map(|f| f.name.as_str())
        .unwrap_or("?");

    let items: Vec<ListItem> = if app.family_properties.is_empty() {
        // No OCD properties - show article variants instead
        let mut items = vec![
            ListItem::new(Line::from(Span::styled(
                "Keine konfigurierbaren Eigenschaften - Artikel wählen:",
                Style::default().fg(Color::Yellow),
            ))),
            ListItem::new(Line::from("")),
        ];

        // Show available article variants
        if let Some(ref family) = app.selected_family {
            if family.article_nrs.len() > 1 {
                items.push(ListItem::new(Line::from(Span::styled(
                    format!("Verfügbare Artikel ({}) - ↑↓ Enter:", family.article_nrs.len()),
                    Style::default().add_modifier(Modifier::BOLD),
                ))));
                items.push(ListItem::new(Line::from("")));

                for (i, art_nr) in family.article_nrs.iter().enumerate() {
                    let is_focused = i == app.focused_article_index;
                    let prefix = if is_focused { "▶ " } else { "  " };

                    // Get description for this article
                    let desc = family.article_descriptions.get(i)
                        .map(|d| truncate_str(d, 40))
                        .unwrap_or_default();

                    let style = if is_focused {
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    let desc_style = if is_focused {
                        Style::default()
                            .fg(Color::Yellow)
                            .bg(Color::DarkGray)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };

                    let line = Line::from(vec![
                        Span::styled(prefix.to_string(), style),
                        Span::styled(format!("{:<12}", art_nr), style),
                        Span::raw(" "),
                        Span::styled(desc, desc_style),
                    ]);

                    items.push(ListItem::new(line));
                }
            } else {
                // Single article
                let desc = family.article_descriptions.first()
                    .map(|d| d.as_str())
                    .unwrap_or("-");
                items.push(ListItem::new(Line::from(vec![
                    Span::styled("Artikel: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(family.base_article_nr.clone()),
                ])));
                items.push(ListItem::new(Line::from(Span::styled(
                    desc.to_string(),
                    Style::default().fg(Color::DarkGray),
                ))));
            }
        }

        items
    } else {
        app.family_properties
            .iter()
            .enumerate()
            .map(|(i, prop)| render_property_item(prop, app, i))
            .collect()
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} - Konfiguration ", family_name)),
    );

    frame.render_widget(list, area);
}

/// Render a single property item with current selection
#[cfg(feature = "tui")]
fn render_property_item(
    prop: &FamilyProperty,
    app: &App,
    index: usize,
) -> ListItem<'static> {
    let is_focused = index == app.focused_property;

    // Get current value from configuration
    let current_value = app
        .family_config
        .as_ref()
        .and_then(|c| c.selections.get(&prop.key))
        .cloned()
        .unwrap_or_default();

    // Find matching option label
    let value_label = prop
        .options
        .iter()
        .find(|o| o.value == current_value)
        .map(|o| o.label.clone())
        .unwrap_or_else(|| {
            if current_value.is_empty() {
                "-".to_string()
            } else {
                current_value.clone()
            }
        });

    let prefix = if is_focused { "▶ " } else { "  " };
    let required_mark = if prop.required { "*" } else { "" };

    // Type indicator
    let type_info = match &prop.prop_type {
        PropertyType::Choice => {
            let count = prop.options.len();
            format!("[{} Optionen]", count)
        }
        PropertyType::Range { min, max, step } => {
            format!("[{:.0}-{:.0}, ±{:.0}]", min, max, step)
        }
        PropertyType::Integer { min, max } => {
            format!("[{}-{}]", min, max)
        }
        PropertyType::Boolean => "[Ja/Nein]".to_string(),
        PropertyType::Text => "[Text]".to_string(),
    };

    let label_style = if is_focused {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let value_style = if is_focused {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let line = Line::from(vec![
        Span::raw(prefix.to_string()),
        Span::styled(format!("{:<20}{}", prop.label, required_mark), label_style),
        Span::raw(" = "),
        Span::styled(format!("{:<20}", value_label), value_style),
        Span::styled(type_info, Style::default().fg(Color::DarkGray)),
    ]);

    if is_focused {
        ListItem::new(line).style(Style::default().bg(Color::DarkGray))
    } else {
        ListItem::new(line)
    }
}

/// Render the summary panel with variant code, price, and options
#[cfg(feature = "tui")]
fn render_summary(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Variant code
            Constraint::Length(10), // Current property options
            Constraint::Min(0),     // Price info
        ])
        .split(area);

    // Variant code
    let variant_code = app
        .family_config
        .as_ref()
        .map(|c| c.variant_code.as_str())
        .unwrap_or("-");

    // Show focused article when navigating article list (no properties mode)
    let (article_nr, article_desc) = if app.family_properties.is_empty() {
        // Article selection mode - show focused article
        app.selected_family.as_ref().and_then(|f| {
            let idx = app.focused_article_index;
            f.article_nrs.get(idx).map(|nr| {
                let desc = f.article_descriptions.get(idx)
                    .map(|d| d.as_str())
                    .unwrap_or("");
                (nr.as_str(), desc)
            })
        }).unwrap_or(("-", ""))
    } else {
        // Property mode - show base article
        app.selected_family
            .as_ref()
            .map(|f| (f.base_article_nr.as_str(), ""))
            .unwrap_or(("-", ""))
    };

    let mut lines = vec![
        Line::from(vec![
            Span::styled("Artikel:      ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(article_nr.to_string()),
        ]),
    ];

    // Show article description if available
    if !article_desc.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(
                truncate_variant_code(article_desc, 35),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    lines.push(Line::from(vec![
        Span::styled("Variante:     ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(
            truncate_variant_code(variant_code, 30),
            Style::default().fg(Color::Cyan),
        ),
    ]));

    let variant_block = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Code "));

    frame.render_widget(variant_block, chunks[0]);

    // Current property options (for focused property)
    render_options_panel(frame, app, chunks[1]);

    // Price info
    render_price_panel(frame, app, chunks[2]);
}

/// Render the options panel for the currently focused property
#[cfg(feature = "tui")]
fn render_options_panel(frame: &mut Frame, app: &App, area: Rect) {
    let (options_lines, title) = if app.family_properties.is_empty() {
        // Article selection mode - show hint
        let lines = if let Some(ref family) = app.selected_family {
            vec![
                Line::from(Span::styled(
                    format!("{} Artikel verfügbar", family.article_nrs.len()),
                    Style::default().fg(Color::White),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "↑↓ Navigieren",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "Enter Auswählen",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    "Esc  Zurück",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        } else {
            vec![Line::from(Span::styled(
                "Keine Artikel",
                Style::default().fg(Color::DarkGray),
            ))]
        };
        (lines, " Artikel ".to_string())
    } else if let Some(prop) = app.family_properties.get(app.focused_property) {
        let current_value = app
            .family_config
            .as_ref()
            .and_then(|c| c.selections.get(&prop.key))
            .cloned()
            .unwrap_or_default();

        let lines: Vec<Line> = prop.options
            .iter()
            .take(6)
            .map(|opt| {
                let is_selected = opt.value == current_value;
                let prefix = if is_selected { "● " } else { "○ " };
                let style = if is_selected {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                Line::from(vec![
                    Span::styled(prefix.to_string(), style),
                    Span::styled(opt.label.clone(), style),
                ])
            })
            .collect();
        (lines, format!(" {} ", prop.label))
    } else {
        (vec![Line::from(Span::styled(
            "Keine Optionen",
            Style::default().fg(Color::DarkGray),
        ))], " Optionen ".to_string())
    };

    let options_block = Paragraph::new(options_lines)
        .block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(options_block, area);
}

/// Render the price panel
#[cfg(feature = "tui")]
fn render_price_panel(frame: &mut Frame, app: &App, area: Rect) {
    let price_lines = if let Some(ref price) = app.family_price {
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
                    format!("  + {}:  ", truncate_str(&surcharge.name, 15)),
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
            "Preis wird geladen...",
            Style::default().fg(Color::Yellow),
        ))]
    };

    let price_block =
        Paragraph::new(price_lines).block(Block::default().borders(Borders::ALL).title(" Preis "));

    frame.render_widget(price_block, area);
}

/// Truncate variant code for display (UTF-8 safe)
fn truncate_variant_code(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count > max_len {
        let truncated: String = s.chars().take(max_len - 1).collect();
        format!("{}…", truncated)
    } else {
        s.to_string()
    }
}

/// Truncate a string for display (UTF-8 safe)
fn truncate_str(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count > max_len {
        let truncated: String = s.chars().take(max_len - 1).collect();
        format!("{}…", truncated)
    } else {
        s.to_string()
    }
}
