//! List widget helpers

#[cfg(feature = "tui")]
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

/// Create a styled list item for manufacturers
#[cfg(feature = "tui")]
pub fn manufacturer_item(id: &str, name: &str, article_count: usize) -> ListItem<'static> {
    let line = Line::from(vec![
        Span::styled(
            format!("{:<12}", id),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" {} ", name)),
        Span::styled(
            format!("({} Artikel)", article_count),
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    ListItem::new(line)
}

/// Create a styled list item for articles
#[cfg(feature = "tui")]
pub fn article_item(id: &str, description: &str, has_configuration: bool) -> ListItem<'static> {
    let config_indicator = if has_configuration {
        Span::styled("●", Style::default().fg(Color::Green))
    } else {
        Span::styled("○", Style::default().fg(Color::DarkGray))
    };

    let line = Line::from(vec![
        config_indicator,
        Span::raw(" "),
        Span::styled(
            format!("{:<24}", id),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" {}", description)),
    ]);
    ListItem::new(line)
}
