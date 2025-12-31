use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
};
use ratatui::{prelude::*, widgets::*, layout::*, style::*, text::*,};


pub fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let text = vec![
                Line::from(vec![
                    Span::raw("First"),
                    Span::styled("line", Style::new().green().italic()),
                    ".".into(),
                    ]),
                Line::from("Second line".red()),
                "Third line".into(),
            ];
            let paragraph = Paragraph::new(text)
                .block(Block::bordered().title("Paragraph"))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, size);
        })?;



        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}