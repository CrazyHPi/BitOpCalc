use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::calc::bit_width::BitWidth;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let title = Span::styled(" BitOpCalc ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let widths = [(BitWidth::B8, "8"), (BitWidth::B16, "16"), (BitWidth::B32, "32"), (BitWidth::B64, "64")];
    let width_spans: Vec<Span> = widths
        .iter()
        .map(|(w, label)| {
            if app.bit_width == *w {
                Span::styled(
                    format!("[{}] ", label),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(format!(" {}  ", label))
            }
        })
        .collect();

    let right = Line::from(width_spans);
    let lines = vec![Line::from(vec![title]), right];

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
