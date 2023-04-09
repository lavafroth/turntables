use tui::{backend::Backend, Frame};
pub trait Menu {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>);
    fn next(&mut self);
    fn previous(&mut self);
    fn enter(&mut self);
    fn entered(&self) -> bool;
    fn back(&mut self);
    fn label(&self) -> String;
}
