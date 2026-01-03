use std::io;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};

use crate::Screen;

pub fn run_playing(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<Screen> {
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let content = Paragraph::new("Playing... Press Esc to return").block(
                Block::default()
                    .title("Playing")
                    .borders(Borders::ALL),
            );
            frame.render_widget(content, area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                return Ok(Screen::MainMenu);
            }
            //for faster debugging, remove later
            if key.code == KeyCode::Char('q') {
                return Ok(Screen::Quit);
            }
        }
    }
}