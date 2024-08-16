use std::{
    fs::File,
    io::{self, Read, Write},
    thread,
    time::Duration,
};

const BLINKING_UNDERLINE: &str = "\x1b[3 q";
const STEADY_UNDERLINE: &str = "\x1b[4 q";

fn main() -> io::Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    let mut tty = File::open("/dev/tty")?;
    print!("{STEADY_UNDERLINE}");
    io::stdout().flush()?;

    for c in content.chars() {
        if c == '\n' {
            print!("{BLINKING_UNDERLINE}");
            io::stdout().flush()?;
            tty.read_exact(&mut [0])?;
            print!("{STEADY_UNDERLINE}");
            io::stdout().flush()?;
            continue;
        }
        print!("{}", c);
        io::stdout().flush()?;
        thread::sleep(Duration::from_millis(40));
    }

    Ok(())
}
