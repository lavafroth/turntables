use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
mod widgets;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use widgets::{Menu, StatefulList, StatefulTable};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut menu: impl Menu) -> io::Result<()> {
    loop {
        terminal.draw(|f| menu.render(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up => menu.previous(),
                KeyCode::Down => menu.next(),
                KeyCode::Right => menu.enter(),
                KeyCode::Left => menu.back(),
                _ => {}
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = StatefulTable::new(
        "Arch Oxide Installer",
        vec![
            StatefulList::new("Language", vec!["English", "German", "Turkish"]),
            StatefulList::new("Drives", vec!["/dev/sda", "/dev/lmao", "/dev/wtf"]),
        ],
    );
    let res = run_app(&mut terminal, app);

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
