use std::io;
use crossterm::{event::{self, Event, KeyCode}};
use ratatui::prelude::*;

use crate::Screen;

pub fn run_playing(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<Screen> {
    loop {
        crate::screens::acts::act1::scene1::run_scene1(terminal)?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                return Ok(Screen::MainMenu);
            }
            //DEBUG
            if key.code == KeyCode::Char('q') {
                return Ok(Screen::Quit);
            }
        }
    }
}