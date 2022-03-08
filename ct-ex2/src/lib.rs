use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
//Result<mut tui::Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>>
pub type term_type = Terminal<CrosstermBackend<io::Stdout>>;

pub fn create_terminal() -> Result<term_type, Box<dyn Error>> {
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

pub fn terminal_draw(terminal: &mut term_type) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Hello").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    Ok(())
}

pub fn restore_terminal(terminal: &mut term_type) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    let backend = terminal.backend_mut();
    backend
        .execute(LeaveAlternateScreen)?
        .execute(DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
