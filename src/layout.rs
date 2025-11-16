use ratatui::{Frame, layout::Rect, prelude::*};

use std::rc::Rc;

pub fn create_vertical_layout(frame: &Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),       // ruler
            Constraint::Percentage(100), // middle area (hex / text content)
            Constraint::Length(1),       // status bar
            Constraint::Length(1),       // command bar
        ])
        .split(frame.area())
}
