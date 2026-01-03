use ratatui::{prelude::*, widgets::*};

pub fn render_playing(frame: &mut Frame) {
    let area = frame.size();
    let content = Paragraph::new("esc to return").block(
        Block::default()
            .title("Playing")
            .borders(Borders::ALL),
    );
    frame.render_widget(content, area);
}