use std::io;
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
    Terminal,
};
use std::collections::HashMap;

fn assets_load() -> HashMap<String, Vec<String>> {
    let ascii_files = vec![
    "assets/small_car.txt".to_string()
    ];
    let ascii_assets = crate::utils::assets::load_ascii_art(ascii_files);
    ascii_assets
}

fn render_car(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, car: &Vec<String>, x: u16, y: u16) -> io::Result<()> {
    terminal.draw(|f| {
        // Create a Rect at position (x, y) with width/height of car
        let width = car.iter().map(|line| line.len()).max().unwrap_or(1) as u16;
        let height = car.len() as u16;
        let area = Rect { x, y, width, height };

        // Convert car lines to Line
        let lines: Vec<Line> = car.iter().map(|line| Line::raw(line.as_str())).collect();

        let paragraph = Paragraph::new(lines)
            .style(Style::default().fg(Color::Yellow)); // optional color

        f.render_widget(paragraph, area);
    })?;
    Ok(())
}

pub fn lesgo(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()>
{
    let ascii_assets = assets_load();

    render_car(terminal, &ascii_assets["small_car"], 10, 5)?;
    Ok(())
}