use crate::{app::App, editor::UIState};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Alignment;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, layout::Rect, widgets::Clear};
use std::io::Result;

impl App {
    pub fn log(&mut self, text: String) {
        self.logs.push(text)
    }
}

pub fn dialog_log_draw(app: &mut App, frame: &mut Frame) {
    let text = format!("{:?}\n\n{}", app.reader, &app.logs.join("\n"));

    let para = Paragraph::new(text)
        .style(app.config.theme.dialog)
        .wrap(Wrap { trim: true })
        .block(
            Block::bordered()
                .title(" Log ")
                .title_alignment(Alignment::Center),
        )
        .scroll(app.log_scroll_offset);

    let dialog_area = Rect {
        x: frame.area().x + 2,
        y: frame.area().y + 2,
        width: frame.area().width - 5,
        height: frame.area().height - 5,
    };

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(para, dialog_area);
}

pub fn dialog_log_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        // close log dialog
        KeyCode::Esc => {
            app.dialog_renderer = None;
            app.state = UIState::Normal;
        }
        KeyCode::Down => {
            app.log_scroll_offset.0 += 1;
        }
        KeyCode::Up => {
            app.log_scroll_offset.0 = app.log_scroll_offset.0.saturating_sub(1);
        }
        _ => {}
    }
    Ok(false)
}
