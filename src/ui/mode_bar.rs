use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, Mode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mode_label = match app.mode {
        Mode::RealTime => "Real-Time",
        Mode::Expression => "Expression",
    };

    let hints = if app.show_help {
        "[q] Quit  [Tab] Mode  [1-4] Bit Width  [?] Close Help"
    } else {
        "[q] Quit  [Tab] Mode  [1-4] Width  [?] Help"
    };

    let spans = vec![
        Span::styled(
            format!(" Mode: {}  ", mode_label),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(hints, Style::default().fg(Color::DarkGray)),
    ];

    let paragraph = Paragraph::new(Line::from(spans));
    frame.render_widget(paragraph, area);
}
