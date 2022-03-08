use std::{io, thread, time::Duration};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ct_ex2::add_str;
use tui::backend::Backend;

fn main() {
    enable_raw_mode().unwrap();
    let stdout = io::stdout();
    let mut terminal = ct_ex2::create_terminal().expect("Terminal unusable");
    terminal
        .show_cursor()
        .expect("Unable to use the terminal properly");
    ct_ex2::terminal_draw(&mut terminal).expect("Could not figure out terminal_draw");
    for var in 0..200 {
        terminal.clear();
        add_str("Hello", var + 2, 15, &mut terminal).unwrap();
        add_str("Erik", 15, var + 2, &mut terminal).unwrap();
        thread::sleep(Duration::from_millis(120));
    }
    ct_ex2::restore_terminal(&mut terminal).expect("Unable to restore terminal");
}
