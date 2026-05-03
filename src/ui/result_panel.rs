use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Result ")
        .style(Style::default().fg(Color::White));

    let lines = match app.result {
        Some(result) => {
            let binary = app.bit_width.format_binary(result.value);
            let hex = app.bit_width.format_hex(result.value);
            let decimal = result.value.to_string();
            let signed = app.bit_width.signed_value(result.value).to_string();

            let overflow_hint = if result.overflow {
                Span::styled(" (overflow)", Style::default().fg(Color::Red))
            } else {
                Span::raw("")
            };

            vec![
                Line::from(vec![
                    Span::styled("Binary:  ", Style::default().fg(Color::Cyan)),
                    Span::raw(binary),
                ]),
                Line::from(vec![
                    Span::styled("Hex:     ", Style::default().fg(Color::Cyan)),
                    Span::raw(hex),
                ]),
                Line::from(vec![
                    Span::styled("Decimal: ", Style::default().fg(Color::Cyan)),
                    Span::raw(decimal),
                ]),
                Line::from(vec![
                    Span::styled("Signed:  ", Style::default().fg(Color::Cyan)),
                    Span::raw(signed),
                ]),
                Line::from(vec![overflow_hint]),
            ]
        }
        None => {
            vec![Line::from(Span::styled(
                "  ---",
                Style::default().fg(Color::DarkGray),
            ))]
        }
    };

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
