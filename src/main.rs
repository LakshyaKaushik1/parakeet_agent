// main.rs
// This file is the "runner". It doesn't know HOW things are drawn,
// it just knows WHEN to draw and when to quit.

mod tui; // tells Rust: "there's a tui.rs file, treat it as a module"
mod critical;
mod colors;
mod app;
mod pyt;

use crossterm::event::{self, Event, KeyCode, KeyModifiers, KeyEventKind};
use std::io;

fn main() -> io::Result<()> {
    // Step 1: set up the terminal (raw mode + alternate screen)
    let mut terminal = tui::init()?;

    let mut app = app::App::new();

    // Step 2: run our app loop. We keep the result so we can restore
    // the terminal EVEN IF something went wrong (see below).
    let app_result = run(&mut terminal, &mut app);

    // Step 3: ALWAYS restore the terminal, whether run() succeeded or not.
    // If we skipped this on error, your terminal would stay broken.
    tui::restore()?;

    app_result
}

/// The main application loop.
fn run(terminal: &mut tui::Tui, app: &mut app::App) -> io::Result<()> {
    loop {
        // terminal.draw() takes a closure. Inside, we get `frame`,
        // and we hand it off to our tui.rs draw function.
        app.drain_shell_output();
        terminal.draw(|frame| tui::draw(frame, app))?;

        // Check if a key was pressed. event::poll lets us check
        // "is there an event waiting?" without blocking forever.
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        app.quit();
                    }
                    (KeyCode::Right, KeyModifiers::CONTROL) => {
                        app.focus_agent_panel();
                    }
                    (KeyCode::Left, KeyModifiers::CONTROL) => {
                        app.focus_terminal_panel();
                    }
                    (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                        app.toggle_fullscreen();
                    }
                    (KeyCode::Enter, _) => {
                        app.submit_input();
                    }
                    (KeyCode::Backspace, _) => {
                        app.backspace();
                    }
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        app.backspace();
                    }
                    (KeyCode::Left, _) => {
                        app.move_cursor_left();
                    }
                    (KeyCode::Right, _) => {
                        app.move_cursor_right();
                    }
                    (KeyCode::Char(c), _) => {
                        app.push_char(c);
                    }
                    (KeyCode::Up, _) => {
                        app.scroll_up();
                    }
                    (KeyCode::Down, _) => {
                        app.scroll_down();
                    }
                    _ => {}
                }
                }
            }
        }
        if app.should_quit(){
            break;
        }
    }
    Ok(())
}