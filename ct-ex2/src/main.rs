use std::io;

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

    disable_raw_mode();
}
