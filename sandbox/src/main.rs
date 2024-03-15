use std::{thread, time::Duration};
use std::io::{self, Write};

static SLEEP_TIME: u64 = 50;

fn print_and_flush(i: u32) {
    // https://en.wikipedia.org/wiki/ANSI_escape_code
    // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
    // For formatting https://doc.rust-lang.org/std/fmt/ ( the {:0>4} indicates we format our argument to pad with 0's, right align, 4 spaces)
    
    // To format colors, we use ANSI control characters
    // Specifically ESC [ n m
    // \x1b is the escape character
    // [ is the control sequence introducer
    // The combindation of [ n m (where n is 32 in this case) indicates this is Select Graphic Rendition
    // The 32 indicates the foreground color to be green, we can chain options with ;
    // \x1b[0m Resets the graphics
    // \r is returning to the start of the line instead of adding a newline, since we're using print, we need to follow up with a flush
    print!("\x1b[32m{:0>4}\x1b[0m\r",i);
    io::stdout().flush().unwrap();
}

fn main() {
    for i in 1u32..=20 {
        print_and_flush(i);
        thread::sleep(Duration::from_millis(SLEEP_TIME));
    }
}
