use std::{
    io::{self, Read, Write},
    thread,
    time::Duration,
};
use termion::{
    cursor::{BlinkingBlock, SteadyBlock},
    get_tty,
    raw::IntoRawMode,
};

fn main() -> io::Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    let sentences: Vec<&str> = content
        .split_inclusive(&['.', '!', '?', ';', '\n'])
        .collect();

    let mut stdout = io::stdout().lock();

    for sentence in sentences {
        write!(stdout, "{SteadyBlock}")?;
        for c in sentence.chars() {
            write!(stdout, "{}", c)?;
            stdout.flush()?;

            let wait_duration = Duration::from_millis(45);
            thread::sleep(wait_duration);
        }
        write!(stdout, "{BlinkingBlock}")?;

        // Wait for a keypress
        let mut tty = get_tty()?.into_raw_mode()?;

        let mut buf = [0; 1];
        tty.read_exact(&mut buf)?;
        if buf[0] == b'q' {
            break;
        }
    }

    Ok(())
}
