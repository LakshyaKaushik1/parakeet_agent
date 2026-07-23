use chrono::format::Pad::Space;
use ratatui::{
    Terminal, backend::{self, CrosstermBackend}, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Style, Styled, Stylize}, text::Line, widgets::{
        Block, Borders, Clear, Paragraph, Padding
    }, Frame
};

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use tui_term::widget::{Cursor, PseudoTerminal};

use std::{default, fmt::format, io::{self, Stdout}};
use std::path::PathBuf;

// Local file imports
use crate::critical::{
    self, APP_VERSION, get_date_date_time, get_working_directory,
};

use crate::colors;
use crate::app::{App, ActivePanel};


pub type Tui = Terminal<CrosstermBackend<Stdout>>; // Define the type of the Tui

// Initalize to enable the raw mode
pub fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

// Disable the raw mode (close the alternate screen)
pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}


// Draws the Main Box
pub fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    
    // Gets the active tab mode
    let mode = match app.get_active_panel() {
        ActivePanel::Terminal => " (Terminal) ",
        ActivePanel::Agent => " (Agent) ",
    };

    // define variable area and the theme of the app
    let area = frame.area();
    let theme = colors::parakeet_mode();

    // Defining necessary fields for the app
    let title_center = format!(" Parakeet v{} ", APP_VERSION);

    // Call the function directly and fall back to "." if it fails
    let cwd_left = get_working_directory().unwrap_or_else(|_| String::from("."));

    // Date and Time variable
    let date_time_day_right = get_date_date_time().unwrap_or_else(|_| String::from("Time Not Found!"));

    // Creating the main block. In this block, the panels will be created.
    let main_block = Block::default() //Creates a bordered box. This is the main block
        .title_top(Line::from(format!(" ({}) ",cwd_left)).left_aligned().style(Style::default().fg(theme.directory)))
        .title_top(Line::from(title_center).centered())
        .title_top(Line::from(format!(" ({}) ",date_time_day_right)).right_aligned().style(Style::default().fg(theme.date_time)))
        .bg((theme.agent_bg))
        .border_style(Style::default().fg(theme.main_border))
        .padding(Padding { left: (1), right: (1), top: (1), bottom: (0) }) // adds padding inside the main block.
        .borders(Borders::ALL);

    // Defining the inner blocks.
    let inner_block = main_block.inner(area);
    frame.render_widget(main_block, area); // render the main block

    // Dividing the main area into 2 parts. One is above(consisting panels) and one is below (input box)
    let main_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(5),
        ])
        .split(inner_block); 

    // Define that the top chunk is divided into 2 parts
    if app.fullscreen(){   
        match app.get_active_panel(){
            ActivePanel::Terminal => render_terminal_panel(frame, app, main_area[0]),
            ActivePanel::Agent => render_agent_panel(frame, app, main_area[0]), 
        } 
    }
    
    else {
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_area[0]); // Give it top area in the vertical section.

        render_terminal_panel(frame, app, top_chunks[0]);
        render_agent_panel(frame, app, top_chunks[1]);
    }

    // Bottom Input Box
    // let input_box = Paragraph::new("Type your query here...")
    //     .block(
    //         Block::default()
    //             .title(" Input ").style(Style::default().fg(theme.input_text))
    //             .borders(Borders::ALL)
    //             .style(Style::default())
    //             .bg(theme.agent_bg)
    //             .border_style(Style::default().fg(theme.main_border))

    //     );
    input_area(frame, app, main_area[1]);
    // frame.render_widget(input_box, main_area[1]);
}

fn render_terminal_panel(frame: &mut Frame, app: &mut App, area : Rect){

    // Defines the focused mode is terminal
    let is_focused = app.get_active_panel() == ActivePanel::Terminal;

    // Defines text style for it
    let text_style = colors::text_style_for(is_focused);

    // Defines border style for it
    let border_style = colors::border_style_for(is_focused);

    let inner_width = area.width.saturating_sub(2);
    let inner_height = area.height.saturating_sub(2);
    app.resize_terminal(inner_height, inner_width);

    // Defining the terminal panel's heading 
    let terminal_panel = Block::default()
        .title_top(Line::from(" (Terminal) ").right_aligned())
        .borders(Borders::ALL)
        .style(Style::default())
        .border_style(border_style)
        .bg(colors::parakeet_mode().terminal_bg);

    let pseudo_terminal = PseudoTerminal::new(app.screen())
    .block(terminal_panel)
    .cursor(Cursor::default().symbol("|"));

    frame.render_widget(pseudo_terminal, area);
}

fn render_agent_panel(frame: &mut Frame, app: &App, area : Rect){

    // Defines the focused mode is terminal
    let is_focused = app.get_active_panel() == ActivePanel::Agent;

    // Defines text style for it
    let text_style = colors::text_style_for(is_focused);

    // Defines border style for it
    let border_style = colors::border_style_for(is_focused);

    // Defining the terminal panel's heading 
    let agent_panel = Block::default()
        .title_top(Line::from(" (Agent) ").right_aligned())
        .borders(Borders::ALL)
        .style(Style::default())
        .border_style(border_style)
        .bg(colors::parakeet_mode().terminal_bg);

    frame.render_widget(agent_panel, area);
}

fn input_area(frame : &mut Frame, app : &App, area : Rect) {
    let block = Block::default()
        .title_top(Line::from(" Input Panel ").left_aligned())
        .borders(Borders::ALL)
        .border_style(colors::parakeet_mode().main_border);
    
    let prompt_prefix = format!("{} ", critical::PROMPT_GLYPH);

    let prompt_text = format!("{}{}", prompt_prefix, app.return_input_buffer());

    let paragraph = Paragraph::new(prompt_text).style(Style::default().fg(colors::parakeet_mode().input_text))
    .block(block);

    frame.render_widget(paragraph, area);

    let prefix_width = prompt_prefix.chars().count() as u16;
    let cursor_x = area.x + 1 + prefix_width + app.cursor_position() as u16;
    let cursor_y = area.y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

}