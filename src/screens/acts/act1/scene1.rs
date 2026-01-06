use std::{io, thread, time::Duration};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Block, Borders}
};
use std::collections::HashMap;
use crate::utils::assets::{*,AsciiOp::*};
use crate::terminal::{viewport_rect, terminal_size_error, VIEW_WIDTH, VIEW_HEIGHT};

fn assets_load() -> HashMap<String, Vec<String>> {
    let ascii_files = vec![
    "assets/small_car.txt".to_string(),
    "assets/fence.txt".to_string(),
    ];
    let ascii_assets = crate::utils::assets::load_ascii_art(ascii_files);
    ascii_assets
}

fn render_asset(f: &mut ratatui::Frame, asset: &Vec<String>, x: u16, y: u16) {
    let width = asset.iter().map(|line| line.len()).max().unwrap_or(1) as u16;
    let height = asset.len() as u16;
    let area = Rect { x, y, width, height };

    let lines: Vec<Line> = asset
        .iter()
        .map(|line| Line::from(Span::raw(line)))
        .collect();

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(paragraph, area);
}

fn car_driving(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, ascii_assets: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    let car = &ascii_assets["small_car"];
    let car_height = car.len() as u16;
    let car_width = car.iter().map(|line| line.len()).max().unwrap_or(1) as u16;

    let mut fence = ascii_assets["fence"].clone();

    let x = (VIEW_WIDTH - car_width) / 2;
    let mut y = (VIEW_HEIGHT - car_height) / 2;

    'drive: loop {
        let (cols, rows) = crossterm::terminal::size()?;

        if let Some(viewport) = viewport_rect(cols, rows) {
            let fence_y_top = viewport.y + (viewport.height / 2).saturating_sub(6);
            let fence_y_bottom = viewport.y + viewport.height / 2 + 6;

            terminal.draw(|f| {
                let border = Block::default().borders(Borders::ALL).title("Game");
                f.render_widget(border, viewport);

                render_asset(f, &fence, viewport.x + 1, fence_y_top);
                render_asset(f, &fence, viewport.x + 1, fence_y_bottom);
                render_asset(f, &car, viewport.x + x, viewport.y + y);
            })?;
        } else {
            terminal_size_error(terminal, cols, rows)?;
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
        if move_down && y + car_height < rows {
            y += 1;
        }

        fence = ascii_op(&fence, PermutateX(-1));

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

pub fn run_scene1(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()>
{
    let mut ascii_assets = assets_load();

    car_driving(terminal, &mut ascii_assets)?;
    Ok(())
}