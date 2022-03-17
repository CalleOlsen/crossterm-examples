use std::{
    error::Error,
    io::{self, BufRead, Write},
    thread,
    time::Duration,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Widget},
    Frame, Terminal,
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
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    Ok(terminal)
}

pub fn terminal_draw(terminal: &mut term_type) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Hello").borders(Borders::ALL);
        f.render_widget(block, size);
        my_draw(f);
    })?;
    Ok(())
}

pub fn my_draw<B>(frame: &mut Frame<B>)
where
    B: Backend,
{
    let sz = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .margin(1)
        //.constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(sz);

    let block_1 = Block::default().title("Fisk").borders(Borders::ALL);
    frame.render_widget(block_1, chunks[0]);
    let block_2 = Block::default().title("Pisk").borders(Borders::ALL);
    frame.render_widget(block_2, chunks[1]);
    let block_3 = Block::default().title("Lastly").borders(Borders::ALL);
    frame.render_widget(block_3, chunks[2]);

    let hello = Paragraph::new("\nHello World")
        .style(Style::default().fg(Color::Red))
        .alignment(tui::layout::Alignment::Center);

    frame.render_widget(hello, chunks[0]);

    // Gauge
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(20);

    frame.render_widget(gauge, chunks[1]);
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
//             The idea here is to have
// a rectangle (kind of a frame) and we want to
// insert text here.
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
fn make_paragraph(rect: Rect, text: &str) -> Result<(), Box<dyn Error>> {
    let txt = vec![Sp]

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

pub fn add_str(msg: &str, x: u16, y: u16, terminal: &mut term_type) -> Result<(), Box<dyn Error>> {
    terminal.set_cursor(x, y)?;
    let mut backend = terminal.backend_mut();
    //backend.write(b"Message ")?;
    //backend.flush()?;
    println!("Hoola hopp: <{}>", msg);
    Ok(())
}

pub fn run_loop(terminal: &mut term_type) -> Result<(), Box<dyn Error>> {
    for var in 0..35 {
        terminal_draw(terminal)?;
        thread::sleep(Duration::from_millis(220));
    }

    Ok(())
}
