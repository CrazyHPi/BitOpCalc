use ratatui::layout::{Constraint, Direction, Layout};

pub struct UiRects {
    pub header: ratatui::layout::Rect,
    pub mode_bar: ratatui::layout::Rect,
    pub input: ratatui::layout::Rect,
    pub bit_visualizer: ratatui::layout::Rect,
    pub result: ratatui::layout::Rect,
    pub error: ratatui::layout::Rect,
}

pub fn compute_layout(area: ratatui::layout::Rect) -> UiRects {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Length(1),  // mode bar
            Constraint::Length(3),  // input
            Constraint::Min(4),     // bit visualizer
            Constraint::Length(5),  // result panel
            Constraint::Length(1),  // error line
        ])
        .split(area);

    UiRects {
        header: chunks[0],
        mode_bar: chunks[1],
        input: chunks[2],
        bit_visualizer: chunks[3],
        result: chunks[4],
        error: chunks[5],
    }
}
