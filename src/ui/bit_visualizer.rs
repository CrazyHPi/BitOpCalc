use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use ratatui::layout::Rect;

use crate::app::App;
use crate::app::Mode;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Bits (A | B \u{2192} R) ")
        .style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let bits = app.bit_width.num_bits();
    let rows = bits / 8;

    let mut lines: Vec<Line> = Vec::with_capacity(rows * 3 + 4);

    // Column header
    let mut header_spans = vec![Span::raw("   ")];
    for row_idx in 0..rows {
        let high_bit = (rows - 1 - row_idx) * 8 + 7;
        let low_bit = (rows - 1 - row_idx) * 8;
        header_spans.push(Span::styled(
            format!("{:>2}..{:<2} ", high_bit, low_bit),
            Style::default().fg(Color::DarkGray),
        ));
    }
    lines.push(Line::from(header_spans));

    match app.mode {
        Mode::RealTime => {
            if let Some((a_val, b_val)) = app.real_time.get_operand_values(&app.bit_width) {
                let r_val = app.result.map(|r| r.value).unwrap_or(0);

                lines.push(build_bit_row(a_val, &app.bit_width, 'A', Color::Cyan));
                lines.push(build_bit_row(b_val, &app.bit_width, 'B', Color::Yellow));

                // Separator
                let sep_len = rows * 10 + 3;
                lines.push(Line::from(vec![
                    Span::styled(
                        "  ".to_string() + &"-".repeat(sep_len),
                        Style::default().fg(Color::DarkGray),
                    )
                ]));

                lines.push(build_result_row(r_val, &app.bit_width));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  (no input)", Style::default().fg(Color::DarkGray))
                ]));
            }
        }
        Mode::Expression => {
            // Expression mode: only show result
            if let Some(result) = app.result {
                lines.push(build_result_row(result.value, &app.bit_width));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  (no result)", Style::default().fg(Color::DarkGray))
                ]));
            }
        }
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

fn build_bit_row(value: u64, bit_width: &crate::calc::bit_width::BitWidth, label: char, label_color: Color) -> Line<'static> {
    let bits = bit_width.num_bits();
    let rows = bits / 8;
    let hex_str = bit_width.format_hex(value);

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled(
        format!("{} ", label),
        Style::default().fg(label_color).add_modifier(Modifier::BOLD),
    ));

    for byte_idx in (0..rows).rev() {
        let base_bit = byte_idx * 8;
        for bit_in_byte in (0..8).rev() {
            let bit_pos = base_bit + bit_in_byte;
            let bit = (value >> bit_pos) & 1;
            let ch = if bit == 1 { '1' } else { '0' };
            let color = if bit == 1 { label_color } else { Color::DarkGray };
            spans.push(Span::styled(
                format!("{}", ch),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        }
        if byte_idx > 0 {
            spans.push(Span::raw(" "));
        }
    }

    spans.push(Span::styled(
        format!("   = {}", hex_str),
        Style::default().fg(Color::DarkGray),
    ));

    Line::from(spans)
}

fn build_result_row(value: u64, bit_width: &crate::calc::bit_width::BitWidth) -> Line<'static> {
    let bits = bit_width.num_bits();
    let rows = bits / 8;
    let hex_str = bit_width.format_hex(value);

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled(
        "R ",
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    ));

    for byte_idx in (0..rows).rev() {
        let base_bit = byte_idx * 8;
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
        if byte_idx > 0 {
            spans.push(Span::raw(" "));
        }
    }

    spans.push(Span::styled(
        format!("   = {}", hex_str),
        Style::default().fg(Color::DarkGray),
    ));

    Line::from(spans)
}
