// main.rs
// This file is the "runner". It doesn't know HOW things are drawn,
// it just knows WHEN to draw and when to quit.

mod tui; // tells Rust: "there's a tui.rs file, treat it as a module"
mod critical;
mod colors;

use crossterm::event::{self, Event, KeyCode};
use std::io;

fn main() -> io::Result<()> {
    // Step 1: set up the terminal (raw mode + alternate screen)
    let mut terminal = tui::init()?;

    // Step 2: run our app loop. We keep the result so we can restore
    // the terminal EVEN IF something went wrong (see below).
    let app_result = run(&mut terminal);

    // Step 3: ALWAYS restore the terminal, whether run() succeeded or not.
    // If we skipped this on error, your terminal would stay broken.
    tui::restore()?;

    app_result
}

/// The main application loop.
fn run(terminal: &mut tui::Tui) -> io::Result<()> {
    loop {
        // terminal.draw() takes a closure. Inside, we get `frame`,
        // and we hand it off to our tui.rs draw function.
        terminal.draw(|frame| tui::draw(frame))?;

        // Check if a key was pressed. event::poll lets us check
        // "is there an event waiting?" without blocking forever.
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Press 'q' to quit
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}