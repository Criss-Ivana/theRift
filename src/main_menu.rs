use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
};
use ratatui::{prelude::*, widgets::*, layout::*, style::*, text::*,};

struct Menu {
    items: Vec<String>,
    state: ListState,
}

impl Menu {
    fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            items: vec![
                "Apple".into(),
                "Banana".into(),
                "Orange".into(),
            ],
            state,
        }
    }

    fn next(&mut self) {
    let i = match self.state.selected() {
        Some(i) if i + 1 < self.items.len() => i + 1,
        _ => 0,
    };
    self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(0) | None => self.items.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i));
    }
}


pub fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut mainMenu = Menu::new();

    loop {
        terminal.draw(|frame| {
            let items: Vec<ListItem> = mainMenu
                .items
                .iter()
                .map(|i| ListItem::new(i.as_str()))
                .collect();

            let area = frame.size();

            let list = List::new(items)
                .block(Block::default().title("Fruits").borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Blue))
                .highlight_symbol(">> ");

            frame.render_stateful_widget(list, area, &mut mainMenu.state);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('w') {
                mainMenu.previous();
            }
            if key.code == KeyCode::Char('s') {
                mainMenu.next();
            }
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
