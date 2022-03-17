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
        .hide_cursor()
        //.show_cursor()
        .expect("Unable to use the terminal properly");
    ct_ex2::terminal_draw(&mut terminal).expect("Could not figure out terminal_draw");
    // for var in 0..35 {
    //     terminal.clear();
    //     ct_ex2::terminal_draw(&mut terminal).expect("Could not figure out terminal_draw");
    //     add_str("Hello", var + 2, 15, &mut terminal).unwrap();
    //     add_str("Erik", 15, var + 2, &mut terminal).unwrap();
    //     thread::sleep(Duration::from_millis(220));
    // }
    ct_ex2::run_loop(&mut terminal).expect("Unable to run loop");
    ct_ex2::restore_terminal(&mut terminal).expect("Unable to restore terminal");
}
