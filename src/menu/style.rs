use lazy_static::lazy_static;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};

lazy_static! {
    pub static ref HIGHLIGHT_STYLE: Style = Style::default()
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD);
}

pub fn block<'a, S: AsRef<str>>(title: S) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title.as_ref()))
}
