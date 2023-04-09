use std::fmt::{Display, Formatter};

use tui::widgets::{Borders, ListState};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, List, ListItem},
    Frame,
};

use super::Menu;

#[derive(Debug)]
pub struct StatefulList<T> {
    label: String,
    state: ListState,
    items: Vec<T>,
    entered: bool,
}

impl<T: Display> Display for StatefulList<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.state.selected() {
            Some(i) => write!(f, "{}", self.items.get(i).unwrap()),
            None => write!(f, "None"),
        }
    }
}

impl<T: Display> StatefulList<T> {
    pub fn new<S: AsRef<str>>(label: S, items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            label: label.as_ref().to_string(),
            state: ListState::default(),
            items,
            entered: false,
        }
    }
}

impl<T: Display> Menu for StatefulList<T> {
    fn next(&mut self) {
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
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| ListItem::new(Span::raw(format!("{}", item))))
            .collect();

        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.label.as_str()),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(items, chunks[0], &mut self.state);
    }

    // TODO: Actually do something with the list navigation.
    // For example: Profile selection has a list of lists.
    // We would need a TerminalList type that would do nothing
    // for entering into it. It would only be able to select a
    // list item.
    fn enter(&mut self) {
        self.entered = false;
    }

    fn entered(&self) -> bool {
        self.entered
    }
    fn back(&mut self) {}
    fn label(&self) -> String {
        self.label.clone()
    }
}
