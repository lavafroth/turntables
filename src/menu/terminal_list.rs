use crate::menu::style;
use std::fmt;

use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    text::Span,
    widgets::{List, ListItem, ListState},
    Frame,
};

pub struct TerminalList {
    pub label: String,
    pub description: String,
    pub items: Vec<String>,
    pub state: Option<usize>,
}

impl fmt::Display for TerminalList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: selected: {}",
            self.label,
            match self.state.and_then(|i| self.items.get(i)) {
                None => "None".to_string(),
                Some(m) => m.to_string(),
            }
        )
    }
}

impl TerminalList {
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| ListItem::new(Span::raw(item.to_string())))
            .collect();

        let items = List::new(items)
            .block(style::block(&self.description))
            .highlight_style(*style::HIGHLIGHT_STYLE)
            .highlight_symbol("> ");

        let mut state = ListState::default();
        state.select(self.state);
        f.render_stateful_widget(items, chunks[0], &mut state);
        self.state = state.selected();
    }

    pub fn previous(&mut self) {
        self.state = match self.state {
            Some(i) => {
                if i == 0 {
                    Some(self.items.len() - 1)
                } else {
                    Some(i - 1)
                }
            }
            None => Some(0),
        }
    }

    pub fn next(&mut self) {
        self.state = match self.state {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    Some(0)
                } else {
                    Some(i + 1)
                }
            }
            None => Some(0),
        }
    }
}
