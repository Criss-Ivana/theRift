use std::{io, thread, time::Duration};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use std::collections::HashMap;

fn assets_load() -> HashMap<String, Vec<String>> {
    let ascii_files = vec![
    "assets/small_car.txt".to_string()
    ];
    let ascii_assets = crate::utils::assets::load_ascii_art(ascii_files);
    ascii_assets
}

fn render_car<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, car: &Vec<String>, x: u16, y: u16) -> io::Result<()> {
    terminal.draw(|f| {
        let width = car.iter().map(|line| line.len()).max().unwrap_or(1) as u16;
        let height = car.len() as u16;
        let area = Rect { x, y, width, height };
        let lines: Vec<Line> = car.iter().map(|line| Line::from(Span::raw(line.as_str()))).collect();
        let paragraph = Paragraph::new(lines).style(Style::default().fg(Color::Yellow));
        f.render_widget(paragraph, area);
    })?;
    Ok(())
}

fn car_driving(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, car: &Vec<String>) -> io::Result<()> {
    let car_height = car.len() as u16;
    let car_width = car.iter().map(|line| line.len()).max().unwrap_or(1) as u16;

    let mut x: u16 = 0;
    let (_, rows) = crossterm::terminal::size()?;
    let mut y = rows / 2 - car_height / 2;

    'drive: loop {
        render_car(terminal, &car, x, y)?;

        x += 1;

        let (cols, lines) = crossterm::terminal::size()?;
        if x + car_width >= cols {
            break;
        }

        // Reset pending events because the car had slippery physics lol
        let mut move_up = false;
        let mut move_down = false;

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Up | KeyCode::Char('w') => move_up = true,
                    KeyCode::Down | KeyCode::Char('s') => move_down = true,
                    KeyCode::Esc => break 'drive,
                    _ => {}
                }
            }
        }

        if move_up && y > 0 {
            y -= 1;
        }
        if move_down && y + car_height < lines {
            y += 1;
        }

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

pub fn run_scene1(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()>
{
    let ascii_assets = assets_load();

    car_driving(terminal, &ascii_assets["small_car"])?;
    Ok(())
}