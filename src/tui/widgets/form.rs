//! Form widget for property editing

#[cfg(feature = "tui")]
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[cfg(feature = "tui")]
use crate::property::{PropertyDef, PropertyType, PropertyValue};

/// Property form input state
#[cfg(feature = "tui")]
#[derive(Debug, Clone)]
pub struct PropertyFormState {
    /// Current input buffer
    pub input: String,
    /// Cursor position
    pub cursor: usize,
    /// Whether in edit mode
    pub editing: bool,
    /// Validation error
    pub error: Option<String>,
}

#[cfg(feature = "tui")]
impl Default for PropertyFormState {
    fn default() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            editing: false,
            error: None,
        }
    }
}

#[cfg(feature = "tui")]
impl PropertyFormState {
    /// Create a new form state with initial value
    pub fn new(value: &PropertyValue) -> Self {
        let input = match value {
            PropertyValue::Int(i) => i.to_string(),
            PropertyValue::Float(f) => f.to_string(),
            PropertyValue::Bool(b) => if *b { "ja" } else { "nein" }.to_string(),
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Symbol(s) => s.clone(),
        };
        let cursor = input.len();
        Self {
            input,
            cursor,
            editing: false,
            error: None,
        }
    }

    /// Handle character input
    pub fn insert(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += 1;
        self.error = None;
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.input.remove(self.cursor);
            self.error = None;
        }
    }

    /// Handle delete
    pub fn delete(&mut self) {
        if self.cursor < self.input.len() {
            self.input.remove(self.cursor);
            self.error = None;
        }
    }

    /// Move cursor left
    pub fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right
    pub fn right(&mut self) {
        if self.cursor < self.input.len() {
            self.cursor += 1;
        }
    }

    /// Validate input against property definition
    pub fn validate(&mut self, def: &PropertyDef) -> Result<PropertyValue, String> {
        match &def.prop_type {
            PropertyType::Int { min, max } => {
                let value: i64 = self
                    .input
                    .parse()
                    .map_err(|_| "Ungültige Ganzzahl".to_string())?;
                if let Some(min_val) = min {
                    if value < *min_val {
                        return Err(format!("Wert muss mindestens {} sein", min_val));
                    }
                }
                if let Some(max_val) = max {
                    if value > *max_val {
                        return Err(format!("Wert darf höchstens {} sein", max_val));
                    }
                }
                Ok(PropertyValue::Int(value))
            }
            PropertyType::Float { min, max } => {
                let value: f64 = self
                    .input
                    .parse()
                    .map_err(|_| "Ungültige Dezimalzahl".to_string())?;
                if let Some(min_val) = min {
                    if value < *min_val {
                        return Err(format!("Wert muss mindestens {} sein", min_val));
                    }
                }
                if let Some(max_val) = max {
                    if value > *max_val {
                        return Err(format!("Wert darf höchstens {} sein", max_val));
                    }
                }
                Ok(PropertyValue::Float(value))
            }
            PropertyType::Bool => match self.input.to_lowercase().as_str() {
                "ja" | "true" | "1" | "yes" => Ok(PropertyValue::Bool(true)),
                "nein" | "false" | "0" | "no" => Ok(PropertyValue::Bool(false)),
                _ => Err("Wert muss 'ja' oder 'nein' sein".to_string()),
            },
            PropertyType::Choice { options } => {
                if options.contains(&self.input) {
                    Ok(PropertyValue::Symbol(self.input.clone()))
                } else {
                    Err(format!("Wert muss einer von {:?} sein", options))
                }
            }
            PropertyType::String => Ok(PropertyValue::String(self.input.clone())),
        }
    }

    /// Cycle through choice options
    pub fn cycle_choice(&mut self, def: &PropertyDef, forward: bool) {
        if let PropertyType::Choice { options } = &def.prop_type {
            let current_idx = options.iter().position(|o| o == &self.input);
            let new_idx = match current_idx {
                Some(idx) => {
                    if forward {
                        (idx + 1) % options.len()
                    } else {
                        if idx == 0 {
                            options.len() - 1
                        } else {
                            idx - 1
                        }
                    }
                }
                None => 0,
            };
            self.input = options[new_idx].clone();
            self.cursor = self.input.len();
        }
    }

    /// Toggle boolean value
    pub fn toggle_bool(&mut self) {
        match self.input.to_lowercase().as_str() {
            "ja" | "true" | "1" | "yes" => {
                self.input = "nein".to_string();
            }
            _ => {
                self.input = "ja".to_string();
            }
        }
        self.cursor = self.input.len();
    }
}

/// Render a property input field
#[cfg(feature = "tui")]
pub fn render_property_input(
    frame: &mut Frame,
    state: &PropertyFormState,
    def: &PropertyDef,
    area: Rect,
    focused: bool,
) {
    let style = if focused {
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let type_hint = match &def.prop_type {
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
        PropertyType::Choice { options } => format!("[{}]", options.join(",")),
        PropertyType::Bool => "[ja/nein]".to_string(),
        PropertyType::String => "[Text]".to_string(),
    };

    let content = if state.editing && focused {
        // Show cursor
        let before = &state.input[..state.cursor];
        let after = &state.input[state.cursor..];
        vec![Line::from(vec![
            Span::raw(before),
            Span::styled("│", Style::default().add_modifier(Modifier::RAPID_BLINK)),
            Span::raw(after),
        ])]
    } else {
        vec![Line::from(state.input.clone())]
    };

    let error_style = Style::default().fg(Color::Red);
    let content = if let Some(ref err) = state.error {
        vec![
            content.into_iter().next().unwrap_or_default(),
            Line::from(Span::styled(err.as_str(), error_style)),
        ]
    } else {
        content
    };

    let input = Paragraph::new(content).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} {} ", def.label, type_hint)),
    );

    frame.render_widget(input, area);
}
