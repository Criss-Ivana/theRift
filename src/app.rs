use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
};
use ratatui::prelude::*;

use crate::{main_menu::{Menu, render_main_menu},play::{render_playing}, Screen};

pub fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut screen = Screen::MainMenu;
    let mut main_menu = Menu::new();

    loop {
        terminal.draw(|frame| match screen {
            Screen::MainMenu => render_main_menu(frame, &mut main_menu),
            Screen::Playing => render_playing(frame),
        })?;

        if let Event::Key(key) = event::read()? {
            match screen {
                Screen::MainMenu => main_menu.handle_key(key.code, &mut screen),
                Screen::Playing => {
                    if key.code == KeyCode::Esc {
                        screen = Screen::MainMenu;
                    }
                }
            }
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}