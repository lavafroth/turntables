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
        "Arch Oxide Installer",
        "An installer for Arch Linux in pure Rust",
        vec![
            Menu::list(
                "Profile",
                "profile",
                vec![
                    Menu::terminal_list(
                        "Awesome",
                        "awesomewm",
                        vec!["choose", "some", "property"],
                        None,
                    ),
                    Menu::terminal_list("gnome", "ssup_gnomie", vec!["woo", "hoo", "whooo"], None),
                ],
                None,
            ),
            Menu::terminal_list("foo", "bar", vec!["baz", "egg", "ham"], None),
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
