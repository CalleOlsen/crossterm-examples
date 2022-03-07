use crossterm::{
    cursor, style,
    terminal::{self, Clear},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Result, Write};

fn main() {
    let mut stdout = stdout();

    make_it_happen(&mut stdout).unwrap();
    //stdout.queue(cursor::MoveTo(5, 10));

    //stdout.flush();
    //.write_all(b"Hello World")?;
}

fn make_it_happen(stdout: &mut std::io::Stdout) -> Result<()> {
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::MoveTo(15, 10))?
        .execute(style::Print("Hello bajs\n"))?
        .execute(cursor::Show)?
        .execute(cursor::SetCursorShape(cursor::CursorShape::UnderScore));
    Ok(())
}
