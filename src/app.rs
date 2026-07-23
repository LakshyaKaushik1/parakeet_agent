use std::io::Write;

use vt100::{Parser};
use crate::pyt::{self, ShellHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel{
    Terminal,
    Agent,
}

pub struct App{
    active_panel : ActivePanel,
    should_quit : bool,
    fullscreen : bool,
    shell : ShellHandle,
    input_buffer : String,
    parser : vt100::Parser,
    cursor_position : usize,
    scroll_offset : usize,
}

impl App{
    pub fn new() -> App{
        App { 
            active_panel: (ActivePanel::Terminal),
            should_quit: (false),
            fullscreen: (false),
            shell : pyt::spawn_shell().expect("Failed to spawn the shell!"),
            input_buffer : String::new(),
            parser : Parser::new(24,80,10_000),
            cursor_position : 0,
            scroll_offset : 0,

        }
    }

    pub fn focus_agent_panel(&mut self){
        self.active_panel = ActivePanel::Agent;
    }
    
    pub fn focus_terminal_panel(&mut self){
        self.active_panel = ActivePanel::Terminal;
    }

    pub fn should_quit(&self) -> bool{
        self.should_quit
    }

    pub fn quit(&mut self){
        self.should_quit = true;
    }

    pub fn get_active_panel(&self) -> ActivePanel{
        self.active_panel
    }

    pub fn toggle_fullscreen(&mut self){
        self.fullscreen = !self.fullscreen;
    }

    pub fn fullscreen(&self) -> bool{
        self.fullscreen
    }

    pub fn push_char(&mut self, c:char){
        let byte_index = self.input_buffer
            .char_indices()
            .nth(self.cursor_position)
            .map(|(i,_)|i)
            .unwrap_or(self.input_buffer.len());

        self.input_buffer.insert(byte_index, c);
        self.cursor_position += 1;
    }

    pub fn backspace(&mut self){
        if self.cursor_position == 0{
            return;
        }

        let byte_index = self.input_buffer
            .char_indices()
            .nth(self.cursor_position - 1)
            .map(|(i, _)| i)
            .unwrap_or(self.input_buffer.len());

        self.input_buffer.remove(byte_index);
        self.cursor_position -= 1;
    }

    pub fn return_input_buffer(&self) -> &str{
        &self.input_buffer
    }

    pub fn screen(&self) -> &vt100::Screen{
        self.parser.screen()
    }

    pub fn submit_input(&mut self){
        let command = std::mem::take(&mut self.input_buffer);
        self.cursor_position = 0;

        let _ = self.shell.writer.write_all(command.as_bytes());
        let _ = self.shell.writer.write_all(b"\n");

        self.reset_scroll();
    }

    pub fn drain_shell_output(&mut self) {
        
        let mut received_anything = false;
        while let Ok(chunk) = self.shell.output_rx.try_recv(){
            self.parser.process(&chunk);
            received_anything = true;
        }

        if received_anything {
            self.reset_scroll();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let char_count = self.input_buffer.chars().count();
        if self.cursor_position < char_count {
            self.cursor_position += 1;
        }
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn scroll_up(&mut self){
        self.scroll_offset = self.scroll_offset + 1;
        self.parser.screen_mut().set_scrollback(self.scroll_offset);
    }

    pub fn scroll_down(&mut self){
        
        if self.scroll_offset > 0{
            self.scroll_offset = self.scroll_offset - 1;
        }
        self.parser.screen_mut().set_scrollback(self.scroll_offset);
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn reset_scroll(&mut self){
        self.scroll_offset = 0;
        self.parser.screen_mut().set_scrollback(0);
    }

    pub fn resize_terminal(&mut self, rows: u16, cols: u16) {
    if rows == 0 || cols == 0 {
        return; // avoid degenerate sizes during layout transitions
    }
    self.parser.screen_mut().set_size(rows, cols);
    let _ = self.shell.resize(rows, cols); // see below
}
}

    