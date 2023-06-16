use crate::menu::style;
use crate::menu::Menu;
use std::fmt;

use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    text::Span,
    widgets::{self, ListItem, ListState},
    Frame,
};

pub struct List {
    pub label: String,
    pub description: String,
    pub items: Vec<Menu>,
    pub state: Option<usize>,
    pub entered: bool,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.state.and_then(|i| self.items.get(i)) {
                None => "Unselected".to_string(),
                Some(m) => format!("last used: {}", m.label()),
            }
        )
    }
}

impl List {
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.entered {
            self.get_selected().render(f);
            return;
        }
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| ListItem::new(Span::raw(format!("{} {}", item.label(), item))))
            .collect();

        let items = widgets::List::new(items)
            .block(style::block(&self.description))
            .highlight_style(*style::HIGHLIGHT_STYLE)
            .highlight_symbol("> ");

        let mut state = ListState::default();
        state.select(self.state);
        f.render_stateful_widget(items, chunks[0], &mut state);
        self.state = state.selected();
    }

    pub fn get_selected(&mut self) -> &mut Menu {
        self.state.and_then(|i| self.items.get_mut(i)).unwrap()
    }

    pub fn enter(&mut self) {
        if self.entered {
            self.get_selected().enter();
            return;
        }

        if self.state.is_some() {
            self.entered = true;
        }
    }

    pub fn entered(&self) -> bool {
        self.entered
    }

    pub fn back(&mut self) {
        if self.entered && self.get_selected().entered() {
            self.get_selected().back();
        } else {
            self.entered = false;
        }
    }

    pub fn previous(&mut self) {
        if self.entered {
            self.get_selected().previous();
            return;
        }
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
        if self.entered {
            self.get_selected().next();
            return;
        }
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
