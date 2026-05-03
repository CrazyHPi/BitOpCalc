use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Bit Visualization ")
        .style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let bits = app.bit_width.num_bits();
    let value = app.result.map(|r| r.value).unwrap_or(0);
    let rows = bits / 8;

    let mut lines: Vec<Line> = Vec::with_capacity(rows + 2);

    // Column header
    let mut header_spans = vec![Span::raw("  ")];
    for row_idx in 0..rows {
        let high_bit = (rows - 1 - row_idx) * 8 + 7;
        let low_bit = (rows - 1 - row_idx) * 8;
        header_spans.push(Span::styled(
            format!("{:>2}..{:<2} ", high_bit, low_bit),
            Style::default().fg(Color::DarkGray),
        ));
    }
    lines.push(Line::from(header_spans));

    // Bit rows
    for _row_idx in 0..rows {
        let mut spans = Vec::new();
        spans.push(Span::raw("  "));

        for byte_idx in 0..rows {
            let byte_row = rows - 1 - byte_idx;
            let base_bit = byte_row * 8;

            for bit_in_byte in (0..8).rev() {
                let bit_pos = base_bit + bit_in_byte;
                let bit = (value >> bit_pos) & 1;
                let ch = if bit == 1 { '1' } else { '0' };
                let color = if bit == 1 { Color::Green } else { Color::DarkGray };
                spans.push(Span::styled(
                    format!("{}", ch),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            }

            if byte_idx < rows - 1 {
                spans.push(Span::raw(" "));
            }
        }
        lines.push(Line::from(spans));
    }

    // Legend
    lines.push(Line::from(vec![
        Span::styled("  1", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::styled(" = set    ", Style::default().fg(Color::DarkGray)),
        Span::styled("0", Style::default().fg(Color::DarkGray)),
        Span::styled(" = unset", Style::default().fg(Color::DarkGray)),
    ]));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
