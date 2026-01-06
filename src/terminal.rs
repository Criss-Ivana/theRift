use std::io;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, widgets::{Block, Borders}};

pub const VIEW_WIDTH: u16 = 160;
pub const VIEW_HEIGHT: u16 = 50;

pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> 
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,) -> io::Result<()> 
{
    terminal.show_cursor()?; 
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn viewport_rect(cols: u16, rows: u16) -> Option<Rect> {
    if cols < VIEW_WIDTH || rows < VIEW_HEIGHT {
        return None;
    }

    Some(Rect {
        x: (cols - VIEW_WIDTH) / 2,
        y: (rows - VIEW_HEIGHT) / 2,
        width: VIEW_WIDTH,
        height: VIEW_HEIGHT,
    })
}

pub fn terminal_size_error(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, cols: u16, rows: u16) -> io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.size();
        let text = format!(
            "Terminal too small!\nCurrent: {}x{}\nMinimum required: {}x{}\n\nPlease resize your terminal window or zoom out with Ctrl + '-'.\nPress 'q' to quit.",
            cols,
            rows,
            VIEW_WIDTH,
            VIEW_HEIGHT,
        );
        let content = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Error"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        frame.render_widget(content, area);
    }).map(|_| ())
}