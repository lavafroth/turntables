use std::fmt::Display;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use tui::widgets::TableState;

use super::Menu;

#[derive(Debug)]
pub struct StatefulTable<T: Display> {
    label: String,
    entered: bool,
    state: TableState,
    items: Vec<T>,
}

impl<T: Display> Display for StatefulTable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.state.selected() {
            Some(i) => write!(f, "Last selected: {}", self.items.get(i).unwrap()),
            None => write!(f, "None"),
        }
    }
}

impl<T: Display> StatefulTable<T> {
    pub fn new<S: AsRef<str>>(label: S, items: Vec<T>) -> StatefulTable<T> {
        StatefulTable {
            label: label.as_ref().to_string(),
            entered: false,
            state: TableState::default(),
            items,
        }
    }

    fn get_selected(&mut self) -> &mut T {
        self.items.get_mut(self.state.selected().unwrap()).unwrap()
    }
}

impl<T: Display + Menu> Menu for StatefulTable<T> {
    fn next(&mut self) {
        if self.entered {
            self.get_selected().next();
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.entered {
            self.get_selected().previous();
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.entered {
            self.get_selected().render(f);
            return;
        }
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let selected_style = Style::default().bg(Color::Blue);
        let rows = self.items.iter().map(|item| {
            let key = item.label();
            let height = key.chars().filter(|c| *c == '\n').count() + 1;
            let cells = vec![Cell::from(key), Cell::from(format!("{}", item))];
            Row::new(cells).height(height as u16)
        });
        let t = Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.label.as_str()),
            )
            .highlight_style(selected_style)
            .highlight_symbol("> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);

        f.render_stateful_widget(t, rects[0], &mut self.state);
    }

    fn enter(&mut self) {
        if self.state.selected().is_some() {
            self.entered = true;
        }
    }

    fn back(&mut self) {
        let child = self.get_selected();
        if child.entered() {
            child.back();
        } else {
            self.entered = false;
        }
    }

    fn entered(&self) -> bool {
        self.entered
    }
    fn label(&self) -> String {
        self.label.clone()
    }
}
