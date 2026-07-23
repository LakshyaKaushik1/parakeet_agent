// This file contains all the constants that is used throughout the app

use std::{env, path::PathBuf};
use chrono::Local;

use crossterm::event::KeyCode::Home;
// Fetch the app version from cargo.toml 
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const PROMPT_GLYPH: &str = "❯";

// Get current working directory
pub fn get_working_directory() -> std::io::Result<String>{ // Result<String> makes sure that it returns a string type
    let cwd = env::current_dir()?; // Gets current directory

    let mut cwd_string = cwd.to_string_lossy().into_owned(); //converts the PathBuf type to String

    if let Ok(home) = env::var("HOME"){ //Removes /home/{username} from the string
        if cwd_string.starts_with(&home){
            cwd_string = cwd_string.replacen(&home, "~", 1);
        }
    }
    Ok(cwd_string) //return the final location
}

// Gets the Date, day and time
pub fn get_date_date_time() -> std::io::Result<String>{
    let now = Local::now(); // Fetches pcs date and time

    let formatted_date_and_time = now.format("%a, %d-%b-%Y %H:%M").to_string(); // A = full weekday name, Y=year, b = abbrevated month name, d id the day of the month and HMS is the 24 hrs time format
    Ok(formatted_date_and_time)
}