use std::io;
use ratatui::prelude::*;

use crate::{screens::main_menu::run_main_menu, screens::play::run_playing, Screen};

pub fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut screen = Screen::MainMenu;

    loop {
        screen = match screen {
            Screen::MainMenu => run_main_menu(terminal)?,
            Screen::Playing => run_playing(terminal)?,
            Screen::Quit => return Ok(()),
        };
    }
}