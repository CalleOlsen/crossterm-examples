use std::{io, thread, time::Duration};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::backend::Backend;

fn main() {
    enable_raw_mode().unwrap();
    let stdout = io::stdout();
    let mut terminal = ct_ex2::create_terminal().expect("Terminal unusable");
    terminal
        .show_cursor()
        .expect("Unable to use the terminal properly");

    ct_ex2::terminal_draw(&mut terminal).expect("Could not figure out terminal_draw");
    thread::sleep(Duration::from_millis(5000));
    ct_ex2::restore_terminal(&mut terminal).expect("Unable to restore terminal");
}
