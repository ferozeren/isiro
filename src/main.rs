// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod shared_logic;

/// Main function boby for Desktop Window
fn main() -> Result<(), Box<dyn Error>> {
    shared_logic::run()?;
    Ok(())
}
