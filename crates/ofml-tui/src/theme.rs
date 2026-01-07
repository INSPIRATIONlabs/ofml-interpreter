//! Theme support for TUI
//!
//! Provides Light and Dark themes for the configurator interface.

use ratatui::style::{Color, Style};

/// Theme variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeVariant {
    #[default]
    Dark,
    Light,
}

impl ThemeVariant {
    /// Toggle between light and dark
    pub fn toggle(self) -> Self {
        match self {
            ThemeVariant::Dark => ThemeVariant::Light,
            ThemeVariant::Light => ThemeVariant::Dark,
        }
    }

    /// Get the theme for this variant
    pub fn theme(self) -> Theme {
        match self {
            ThemeVariant::Dark => Theme::dark(),
            ThemeVariant::Light => Theme::light(),
        }
    }
}

/// Color theme for the TUI
#[derive(Debug, Clone)]
pub struct Theme {
    // Base colors
    pub background: Color,
    pub foreground: Color,
    pub muted: Color,
    pub border: Color,

    // Selection/highlight
    pub selection_bg: Color,
    pub selection_fg: Color,

    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Accent colors
    pub accent: Color,
    pub accent_secondary: Color,

    // List items
    pub list_active: Color,
    pub list_inactive: Color,

    // Title bar
    pub title_bg: Color,
    pub title_fg: Color,

    // Search bar
    pub search_bg: Color,
    pub search_fg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Dark theme (default)
    pub fn dark() -> Self {
        Theme {
            // Base colors
            background: Color::Reset,
            foreground: Color::White,
            muted: Color::Gray,
            border: Color::DarkGray,

            // Selection/highlight
            selection_bg: Color::Blue,
            selection_fg: Color::White,

            // Status colors
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,

            // Accent colors
            accent: Color::Cyan,
            accent_secondary: Color::Magenta,

            // List items
            list_active: Color::Green,
            list_inactive: Color::DarkGray,

            // Title bar
            title_bg: Color::Blue,
            title_fg: Color::White,

            // Search bar
            search_bg: Color::Blue,
            search_fg: Color::White,
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Theme {
            // Base colors
            background: Color::White,
            foreground: Color::Black,
            muted: Color::DarkGray,
            border: Color::Gray,

            // Selection/highlight
            selection_bg: Color::Blue,
            selection_fg: Color::White,

            // Status colors
            success: Color::Green,
            warning: Color::Rgb(200, 150, 0), // Darker yellow for visibility
            error: Color::Red,
            info: Color::Blue,

            // Accent colors
            accent: Color::Blue,
            accent_secondary: Color::Magenta,

            // List items
            list_active: Color::Green,
            list_inactive: Color::Gray,

            // Title bar
            title_bg: Color::Blue,
            title_fg: Color::White,

            // Search bar
            search_bg: Color::Blue,
            search_fg: Color::White,
        }
    }

    // Helper methods for common styles

    /// Default text style
    pub fn text(&self) -> Style {
        Style::default().fg(self.foreground)
    }

    /// Muted text style
    pub fn text_muted(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Border style
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Selection/highlight style
    pub fn selection(&self) -> Style {
        Style::default().bg(self.selection_bg).fg(self.selection_fg)
    }

    /// Success text style
    pub fn text_success(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Warning text style
    pub fn text_warning(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// Error text style
    pub fn text_error(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// Info text style
    pub fn text_info(&self) -> Style {
        Style::default().fg(self.info)
    }

    /// Accent text style
    pub fn text_accent(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// Title bar style
    pub fn title(&self) -> Style {
        Style::default().bg(self.title_bg).fg(self.title_fg)
    }

    /// Search bar style
    pub fn search(&self) -> Style {
        Style::default().bg(self.search_bg).fg(self.search_fg)
    }

    /// Active list item indicator style
    pub fn list_active_style(&self) -> Style {
        Style::default().fg(self.list_active)
    }

    /// Inactive list item indicator style
    pub fn list_inactive_style(&self) -> Style {
        Style::default().fg(self.list_inactive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_toggle() {
        let variant = ThemeVariant::Dark;
        assert_eq!(variant.toggle(), ThemeVariant::Light);
        assert_eq!(variant.toggle().toggle(), ThemeVariant::Dark);
    }

    #[test]
    fn test_dark_theme_colors() {
        let theme = Theme::dark();
        assert_eq!(theme.foreground, Color::White);
        assert_eq!(theme.selection_bg, Color::Blue);
    }

    #[test]
    fn test_light_theme_colors() {
        let theme = Theme::light();
        assert_eq!(theme.foreground, Color::Black);
        assert_eq!(theme.background, Color::White);
    }
}
