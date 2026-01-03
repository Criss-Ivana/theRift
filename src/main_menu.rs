use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};

use crate::Screen;

pub struct Menu {
    items: Vec<String>,
    state: ListState,
}

impl Menu {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            items: vec![
                "Play".into(),
                "Quit".into(),
            ],
            state,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i + 1 < self.items.len() => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(0) | None => self.items.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i));
    }


    pub fn handle_key(&mut self, key: KeyCode, screen: &mut Screen) {
        match key {
            KeyCode::Up | KeyCode::Char('w') => self.previous(),
            KeyCode::Down | KeyCode::Char('s') => self.next(),
            KeyCode::Enter => match self.selected_item() {
                Some("Play") => *screen = Screen::Playing,
                Some("Quit") => {}
                _ => {}
            },
            _ => {}
        }
    }

    fn selected_item(&self) -> Option<&str> {
        self.state
            .selected()
            .and_then(|i| self.items.get(i).map(|s| s.as_str()))
    }
}

pub fn render_main_menu(frame: &mut Frame, menu: &mut Menu) {
    let items: Vec<ListItem> = menu
        .items
        .iter()
        .map(|i| ListItem::new(i.as_str()))
        .collect();

    let area = frame.size();

    let list = List::new(items)
        .block(Block::default().title("Main Menu").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut menu.state);
}