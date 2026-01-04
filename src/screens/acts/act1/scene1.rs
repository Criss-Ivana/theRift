use std::io;
use std::fs;
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
    Terminal,
};

fn load_car() -> Vec<String> {
    let content = fs::read_to_string("assets/ascii/small_car.txt")
        .expect("Failed to read small_car.txt");
    content.lines().map(|line| line.to_string()).collect()
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
    let car = load_car();
    render_car(terminal, &car, 10, 5)?;
    Ok(())
}