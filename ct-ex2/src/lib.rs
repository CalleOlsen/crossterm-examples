use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
//Result<mut tui::Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>>
type result_type = Terminal<CrosstermBackend<io::Stdout>>;

pub fn create_terminal() -> Result<result_type, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // stdout
    //     .execute(EnterAlternateScreen)?
    //     .execute(EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    Ok(terminal)
}
