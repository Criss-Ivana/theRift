use std::io;
use crossterm::{event::{self, Event, KeyCode}, terminal};
use ratatui::{prelude::*, widgets::*};

use crate::Screen;

fn check_terminal_size(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<bool> {
    loop {
        let (cols, rows) = terminal::size()?;
        if cols >= 80 && rows >= 24 {
            return Ok(true);
        }
        
        terminal.draw(|frame| {
            let area = frame.size();
            let text = format!("Terminal too small!\nCurrent: {}x{}\nMinimum required: 80x24\n\nPlease resize your terminal.\nPress 'q' to quit.", cols, rows);
            let content = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            frame.render_widget(content, area);
        })?;
        //DEBUG
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(false);
                }
            }
        }
    }
}

pub fn run_playing(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<Screen> {
    loop {
        if !check_terminal_size(terminal)? {
            return Ok(Screen::Quit);
        }

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
            //DEBUG
            if key.code == KeyCode::Char('q') {
                return Ok(Screen::Quit);
            }
        }
    }
}