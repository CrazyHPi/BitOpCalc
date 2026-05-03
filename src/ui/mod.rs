use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;

mod layout;
mod header;
mod mode_bar;
mod input_area;
mod bit_visualizer;
mod result_panel;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let rects = layout::compute_layout(area);

    header::render(frame, rects.header, app);
    mode_bar::render(frame, rects.mode_bar, app);
    input_area::render(frame, rects.input, app);
    bit_visualizer::render(frame, rects.bit_visualizer, app);
    result_panel::render(frame, rects.result, app);

    // Error line
    if let Some(ref error) = app.error {
        let error_widget = Paragraph::new(Span::styled(
            format!(" Error: {}", error),
            Style::default().fg(Color::Red),
        ))
        .block(Block::default().borders(ratatui::widgets::Borders::ALL).title(" Error "));
        frame.render_widget(error_widget, rects.error);
    }

    // Help popup
    if app.show_help {
        render_help(frame, area);
    }
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help_area = Rect::new(
        area.width / 2 - 20,
        area.height / 2 - 6,
        40,
        12,
    );

    let lines = vec![
        Line::from(Span::styled(" Key Bindings ", Style::default().fg(Color::Yellow))),
        Line::from(""),
        Line::from(vec![
            Span::styled(" q     ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit"),
        ]),
        Line::from(vec![
            Span::styled(" Tab   ", Style::default().fg(Color::Cyan)),
            Span::raw("Switch mode"),
        ]),
        Line::from(vec![
            Span::styled(" F1-F4 ", Style::default().fg(Color::Cyan)),
            Span::raw("8/16/32/64 bit"),
        ]),
        Line::from(vec![
            Span::styled(" ←/→  ", Style::default().fg(Color::Cyan)),
            Span::raw("Move focus (real-time)"),
        ]),
        Line::from(vec![
            Span::styled(" ↑/↓  ", Style::default().fg(Color::Cyan)),
            Span::raw("Cycle operator (real-time)"),
        ]),
        Line::from(vec![
            Span::styled(" Enter ", Style::default().fg(Color::Cyan)),
            Span::raw("Evaluate (expression)"),
        ]),
        Line::from(vec![
            Span::styled(" Esc   ", Style::default().fg(Color::Cyan)),
            Span::raw("Clear input"),
        ]),
        Line::from(vec![
            Span::styled(" ?     ", Style::default().fg(Color::Cyan)),
            Span::raw("Toggle help"),
        ]),
        Line::from(""),
        Line::from(Span::styled(" Press ? to close ", Style::default().fg(Color::DarkGray))),
    ];

    let block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .title(" Help ")
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(Clear, help_area);
    frame.render_widget(paragraph, help_area);
}
