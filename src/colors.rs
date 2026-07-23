// This file defines the colors that will be use all across the app
use ratatui::style::{Color, Style};

pub struct ColorPalette {
    pub main_bg: Color, // bg behind all the widgets
    pub main_border: Color, // main border color
    pub panel_border : Color, // color of the panels border
    pub directory: Color, // directory/location
    pub date_time : Color, // date and time color
    pub terminal_bg: Color, // terminal panel bg
    pub agent_bg: Color, // agent panel bg
    pub dimmed_border: Color,  // panel borders
    pub dimmed_panel : Color, // dimmed panel color
    pub input_bg: Color, // input panel bg
    pub input_text: Color, // input text color
}

/// Helper function to convert hex strings like "#1E1E2E" to Ratatui Color
pub fn hex(code: &str) -> Color {
    let code = code.trim_start_matches('#');
    if code.len() == 6 {
        let r = u8::from_str_radix(&code[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&code[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&code[4..6], 16).unwrap_or(0);
        Color::Rgb(r, g, b)
    } else {
        Color::Reset
    }
}

pub fn parakeet_mode() -> ColorPalette {
    ColorPalette {
        main_bg: hex("#060707"),        // Dark background for main canvas
        main_border: hex("#4ab618"), // Bright main border
        panel_border : hex("#dacd21"),
        directory: hex("#3fb3eb"),      // Location / Date-Time color
        date_time : hex("#ecbd82"),
        terminal_bg: hex("#15191d"),    // Dimmed terminal background
        agent_bg: hex("#15191d"),       // Dimmed agent background
        dimmed_border: hex("#2c333b"),  // Dimmed border for inner panels
        dimmed_panel : hex("#060707"),
        input_bg: hex("#12141c"),       // Distinct background for input box
        input_text: hex("#dbdbdb"),     // Text inside input
    }
}

pub fn text_style_for(is_focused: bool) -> Style {
    if is_focused {
        Style::default().fg(parakeet_mode().panel_border)
    } else {
        Style::default().fg(parakeet_mode().dimmed_panel)
    }
}

pub fn border_style_for(is_focused: bool) -> Style {
    if is_focused {
        Style::default().fg(parakeet_mode().panel_border)
    } else {
        Style::default().fg(parakeet_mode().dimmed_border)
    }
}