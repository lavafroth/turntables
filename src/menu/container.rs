use crate::menu::List;
use crate::menu::Table;
use crate::menu::TerminalList;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Frame, Terminal};

pub enum Menu {
    Table(Table),
    List(List),
    TerminalList(TerminalList),
}

use std::fmt;

impl Menu {
    pub fn table<S: AsRef<str>>(
        label: S,
        description: S,
        items: Vec<Menu>,
        default_state: Option<usize>,
    ) -> Self {
        Self::Table(Table {
            entered: false,
            label: label.as_ref().to_string(),
            description: description.as_ref().to_string(),
            items,
            state: default_state,
        })
    }

    pub fn list<S: AsRef<str>>(
        label: S,
        description: S,
        items: Vec<Menu>,
        default_state: Option<usize>,
    ) -> Self {
        Self::List(List {
            entered: false,
            label: label.as_ref().to_string(),
            description: description.as_ref().to_string(),
            items,
            state: default_state,
        })
    }

    pub fn terminal_list<S: AsRef<str>>(
        label: S,
        description: S,
        items: Vec<S>,
        default_state: Option<usize>,
    ) -> Self {
        Self::TerminalList(TerminalList {
            label: label.as_ref().to_string(),
            description: description.as_ref().to_string(),
            items: items.iter().map(|s| s.as_ref().to_string()).collect(),
            state: default_state,
            selected: None,
        })
    }

    pub fn entered(&self) -> bool {
        match self {
            Self::Table(t) => t.entered(),
            Self::List(t) => t.entered(),
            _ => false,
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        match self {
            Menu::Table(t) => t.render(f),
            Menu::List(t) => t.render(f),
            Menu::TerminalList(t) => t.render(f),
        }
    }

    pub fn label(&self) -> String {
        match self {
            Menu::Table(t) => t.label(),
            Menu::List(t) => t.label(),
            Menu::TerminalList(t) => t.label(),
        }
    }

    pub fn previous(&mut self) {
        match self {
            Menu::Table(t) => t.previous(),
            Menu::List(t) => t.previous(),
            Menu::TerminalList(t) => t.previous(),
        }
    }

    pub fn next(&mut self) {
        match self {
            Menu::Table(t) => t.next(),
            Menu::List(t) => t.next(),
            Menu::TerminalList(t) => t.next(),
        }
    }

    pub fn enter(&mut self) {
        match self {
            Menu::Table(t) => t.enter(),
            Menu::List(t) => t.enter(),
            Self::TerminalList(t) => t.select(),
        }
    }

    pub fn back(&mut self) {
        match self {
            Menu::Table(t) => t.back(),
            Menu::List(t) => t.back(),
            _ => {}
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => self.previous(),
                    KeyCode::Down => self.next(),
                    KeyCode::Right | KeyCode::Enter => self.enter(),
                    KeyCode::Left => self.back(),
                    _ => {}
                }
            }
        }
    }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Menu::Table(t) => t.fmt(f),
            Menu::List(t) => t.fmt(f),
            Menu::TerminalList(t) => t.fmt(f),
        }
    }
}
