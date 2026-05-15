use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::calc::parse_number;
use crate::input::real_time::OperandField;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Input ")
        .style(Style::default().fg(Color::White));

    let lines = match app.mode {
        crate::app::Mode::RealTime => render_real_time(app),
        crate::app::Mode::Expression => render_expression(app),
    };

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn render_real_time(app: &App) -> Vec<Line<'static>> {
    let state = &app.real_time;
    let mask = app.bit_width.mask();

    let a_val: u64 = parse_number(&state.operand_a).unwrap_or(0) & mask;
    let b_val: u64 = parse_number(&state.operand_b).unwrap_or(0) & mask;

    let a_hex = app.bit_width.format_hex(a_val);
    let b_hex = app.bit_width.format_hex(b_val);
    let a_focus = state.focused_field == OperandField::A;
    let b_focus = state.focused_field == OperandField::B;

    let op_focus = state.focused_field == OperandField::Operator;

    let line1 = format!(
        "  {:<18}   Op: {:<6}   {:<18}",
        if a_focus {
            format!("> {} <", state.operand_a)
        } else if state.operand_a.is_empty() {
            "(empty)".to_string()
        } else {
            state.operand_a.clone()
        },

        if op_focus {
            format!("( {} )", state.operator.symbol())
        } else {
            format!("{}", state.operator.symbol())
        },

        if b_focus {
            format!("> {} <", state.operand_b)
        } else if state.operand_b.is_empty() {
            "(empty)".to_string()
        } else {
            state.operand_b.clone()
        },
    );

    let line2 = format!(
        "  {:<18}              {:<18}",
        a_hex, b_hex,
    );

    let focus_hint = match state.focused_field {
        OperandField::A => " [←→] Focus: Op1",
        OperandField::Operator => " [←→] Focus: Op",
        OperandField::B => " [←→] Focus: Op2",
    };

    vec![
        Line::from(Span::raw(line1)),
        Line::from(Span::styled(line2, Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled(focus_hint, Style::default().fg(Color::Yellow))),
    ]
}

fn render_expression(app: &App) -> Vec<Line<'static>> {
    let cursor = if app.cursor_visible { "\u{2588}" } else { " " };
    let input = &app.expression.input;

    let line1 = format!("  Expr: {}{}", input, cursor);
    let line2 = Span::styled("  [Enter] Evaluate  [Esc] Clear", Style::default().fg(Color::DarkGray));

    vec![
        Line::from(Span::raw(line1)),
        Line::from(line2),
        Line::from(Span::raw("")),
    ]
}
