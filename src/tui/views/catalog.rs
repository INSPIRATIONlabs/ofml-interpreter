//! XCF Catalog browser view
//!
//! Displays the hierarchical catalog structure from XCF format,
//! allowing navigation through categories to find products.

#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[cfg(feature = "tui")]
use crate::oap::catalog::NodeType;
#[cfg(feature = "tui")]
use crate::tui::app::App;
#[cfg(feature = "tui")]
use crate::tui::ui::render_search_bar;

/// Render the catalog browser view
#[cfg(feature = "tui")]
pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // Layout: breadcrumb + list (+ search bar if active)
    let chunks = if app.search_active {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search bar
                Constraint::Length(2), // Breadcrumb
                Constraint::Min(0),    // List
            ])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Breadcrumb
                Constraint::Min(0),    // List
            ])
            .split(area)
    };

    let (breadcrumb_area, list_area) = if app.search_active {
        render_search_bar(frame, app, chunks[0]);
        (chunks[1], chunks[2])
    } else {
        (chunks[0], chunks[1])
    };

    // Render breadcrumb
    let breadcrumb = app.catalog_breadcrumb();
    let breadcrumb_widget = Paragraph::new(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(breadcrumb, Style::default().fg(Color::Cyan)),
    ]));
    frame.render_widget(breadcrumb_widget, breadcrumb_area);

    // Render catalog items
    let items: Vec<ListItem> = if app.filtered_indices.is_empty() {
        app.catalog_children
            .iter()
            .map(|node| render_catalog_item(node))
            .collect()
    } else {
        app.filtered_indices
            .iter()
            .filter_map(|&i| app.catalog_children.get(i))
            .map(|node| render_catalog_item(node))
            .collect()
    };

    let manufacturer_name = app
        .selected_manufacturer
        .as_ref()
        .map(|m| m.name.as_str())
        .unwrap_or("?");

    let folder_count = app.catalog_children.iter()
        .filter(|n| n.node_type == NodeType::Folder)
        .count();
    let article_count = app.catalog_children.iter()
        .filter(|n| n.node_type == NodeType::Article)
        .count();

    let title = format!(
        " {} - Katalog ({} Kategorien, {} Artikel) ",
        manufacturer_name,
        folder_count,
        article_count
    );

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, list_area, &mut app.catalog_list_state.clone());
}

/// Render a single catalog item
#[cfg(feature = "tui")]
fn render_catalog_item(node: &crate::oap::catalog::CatalogNode) -> ListItem<'static> {
    let (icon, icon_style) = match node.node_type {
        NodeType::Folder => get_folder_icon(&node.name),
        NodeType::Article => (" * ", Style::default().fg(Color::Green)),
        NodeType::Root => ("   ", Style::default()),
    };

    let child_info = if node.node_type == NodeType::Folder && !node.children.is_empty() {
        let sub_folders = node.children.iter().filter(|c| c.node_type == NodeType::Folder).count();
        let sub_articles = node.children.iter().filter(|c| c.node_type == NodeType::Article).count();
        if sub_folders > 0 && sub_articles > 0 {
            format!(" ({} Kat., {} Art.)", sub_folders, sub_articles)
        } else if sub_folders > 0 {
            format!(" ({} Kat.)", sub_folders)
        } else if sub_articles > 0 {
            format!(" ({} Art.)", sub_articles)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let line = Line::from(vec![
        Span::styled(icon, icon_style),
        Span::raw(" "),
        Span::styled(
            truncate_str(&node.name, 60),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(child_info, Style::default().fg(Color::DarkGray)),
    ]);

    ListItem::new(line)
}

/// Get folder icon based on folder name (language-agnostic pattern matching)
#[cfg(feature = "tui")]
fn get_folder_icon(name: &str) -> (&'static str, Style) {
    let name_lower = name.to_lowercase();

    // New/Latest products (German: Neuheiten, English: New, Spanish: Novedades, French: Nouveautés)
    if name_lower.contains("neu") || name_lower.contains("new") || name_lower.contains("novel") || name_lower.contains("nouveau") {
        return ("<3 ", Style::default().fg(Color::Magenta));
    }

    // Bestsellers/Popular (German: Bestseller, English: Best, Spanish: Popular)
    if name_lower.contains("best") || name_lower.contains("popular") || name_lower.contains("top") {
        return ("#1 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    }

    // Information/About (German: Information, English: Info/About)
    if name_lower.contains("info") || name_lower.contains("about") || name_lower.contains("über") {
        return ("(i)", Style::default().fg(Color::Cyan));
    }

    // Categories/All products (German: Kategorien/Alle, English: Categories/All)
    if name_lower.contains("kategor") || name_lower.contains("categor") || name_lower.contains("alle") || name_lower.contains("all ") {
        return ("[+]", Style::default().fg(Color::Blue));
    }

    // Rooms/Spaces (German: Räume, English: Rooms)
    if name_lower.contains("raum") || name_lower.contains("räum") || name_lower.contains("room") || name_lower.contains("space") {
        return ("[=]", Style::default().fg(Color::Green));
    }

    // Quick delivery/Fast (German: Schnell, English: Quick/Fast)
    if name_lower.contains("schnell") || name_lower.contains("quick") || name_lower.contains("fast") || name_lower.contains("express") {
        return (">>>", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
    }

    // Preferred/Recommended combinations (German: Vorzug, English: Preferred)
    if name_lower.contains("vorzug") || name_lower.contains("prefer") || name_lower.contains("recommend") || name_lower.contains("empfohl") {
        return ("[*]", Style::default().fg(Color::Yellow));
    }

    // Accessories/Add-ons (German: Zubehör, English: Accessories)
    if name_lower.contains("zubehör") || name_lower.contains("accessor") || name_lower.contains("add-on") {
        return ("(+)", Style::default().fg(Color::Cyan));
    }

    // Chairs/Seating (German: Stühle/Sitz, English: Chair/Seat)
    if name_lower.contains("stuhl") || name_lower.contains("stühl") || name_lower.contains("chair") || name_lower.contains("seat") || name_lower.contains("sitz") {
        return ("[S]", Style::default().fg(Color::Yellow));
    }

    // Tables/Desks (German: Tisch, English: Table/Desk)
    if name_lower.contains("tisch") || name_lower.contains("table") || name_lower.contains("desk") {
        return ("[T]", Style::default().fg(Color::Yellow));
    }

    // Sofas/Lounge (German: Sofa, English: Sofa/Lounge)
    if name_lower.contains("sofa") || name_lower.contains("lounge") || name_lower.contains("couch") {
        return ("[L]", Style::default().fg(Color::Yellow));
    }

    // Storage/Cabinets (German: Schrank/Regal, English: Cabinet/Storage)
    if name_lower.contains("schrank") || name_lower.contains("cabinet") || name_lower.contains("storage") || name_lower.contains("regal") {
        return ("[C]", Style::default().fg(Color::Yellow));
    }

    // Default folder icon
    ("[ ]", Style::default().fg(Color::Yellow))
}

/// Truncate a string to max length (UTF-8 safe)
fn truncate_str(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count > max_len {
        let truncated: String = s.chars().take(max_len - 1).collect();
        format!("{}...", truncated)
    } else {
        s.to_string()
    }
}
