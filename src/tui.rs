use ratatui::{
    Terminal, backend::{self, CrosstermBackend}, layout::{Alignment, Constraint, Direction, Layout}, style::{Style, Styled, Stylize}, text::Line, widgets::{
        Block, Borders, Clear, Paragraph,
    },
};

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::{default, fmt::format, io::{self, Stdout}};
use std::path::PathBuf;

// Local file imports
use crate::critical::{
    APP_VERSION,
    get_working_directory,
    get_date_date_time,
};

use crate::colors;

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
pub fn draw(frame: &mut ratatui::Frame) {
    
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
        .style(Style::default())
        .bg(theme.agent_bg)
        .border_style(Style::default().fg(theme.main_border))
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
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_area[0]); // Give it top area in the vertical section.
        
    // Defining the terminal panel's heading 
    let terminal_panel = Block::default()
        .title(" (Terminal) ")
        .borders(Borders::ALL)
        .style(Style::default())
        .border_style(Style::default().fg(theme.main_border))
        .bg(theme.terminal_bg);
    frame.render_widget(terminal_panel, top_chunks[0]); //rendering the terminal panel

    // Defining the agent panel's heading
    let agent_panel = Block::default()
        .title(" (Agent) ")
        .borders(Borders::ALL)
        .style(Style::default())
        .border_style(Style::default().fg(theme.main_border))
        .bg(theme.agent_bg);
    frame.render_widget(agent_panel, top_chunks[1]); //rendering the agent panel

    // Bottom Input Box
    let input_box = Paragraph::new("Type your query here...")
        .block(
            Block::default()
                .title(" Input ").style(Style::default().fg(theme.input_text))
                .borders(Borders::ALL)
                .style(Style::default())
                .bg(theme.input_bg)
                .border_style(Style::default().fg(theme.main_border))

        );
    frame.render_widget(input_box, main_area[1]);
    

}
