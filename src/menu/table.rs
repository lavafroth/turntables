use crate::menu::style;
use crate::menu::Menu;
use std::fmt;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    widgets::{self, Cell, Row, TableState},
    Frame,
};

pub struct Table {
    pub label: String,
    pub description: String,
    pub items: Vec<Menu>,
    pub state: Option<usize>,
    pub entered: bool,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}: last used: {}",
            self.description,
            match self.state {
                Some(i) => format!("{}", self.items.get(i).unwrap()),
                None => "unselected".to_string(),
            }
        )
    }
}

impl Table {
    pub fn entered(&self) -> bool {
        self.entered
    }
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.entered {
            self.get_selected().render(f);
            return;
        }
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let rows = self.items.iter().map(|item| {
            let key = item.label();
            let height = key.chars().filter(|c| *c == '\n').count() + 1;
            let cells = vec![Cell::from(key), Cell::from(format!("{}", item))];
            Row::new(cells).height(height as u16)
        });
        let t = widgets::Table::new(rows)
            .block(style::block(&self.label))
            .highlight_style(*style::HIGHLIGHT_STYLE)
            .highlight_symbol("> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(40),
                Constraint::Min(10),
            ]);

        let mut state = TableState::default();
        state.select(self.state);
        f.render_stateful_widget(t, rects[0], &mut state);
        self.state = state.selected();
    }

    fn get_selected(&mut self) -> &mut Menu {
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
