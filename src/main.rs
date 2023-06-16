mod menu;
use menu::Menu;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app: Menu = Menu::table(
        "Table name",
        "Table description",
        vec![
            Menu::list(
                "Profile",
                "profile",
                vec![
                    Menu::terminal_list("A", "a", vec!["aa", "ab", "ac"], None),
                    Menu::terminal_list("B", "b", vec!["ba", "bb", "bc"], None),
                ],
                None,
            ),
            Menu::terminal_list("C", "c", vec!["ca", "cb", "cc"], None),
        ],
        None,
    );

    let res = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
