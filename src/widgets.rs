use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, Paragraph},
};

use crate::app::App;
use crate::util::center_widget;

#[derive(PartialEq)]
pub enum ErrorMessageType {
    OKOnly,
    _RetryAbort,
}

pub struct ErrorMessage {
    message_type: ErrorMessageType,
    pub buffer: String,
}

impl ErrorMessage {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(50),
            message_type: ErrorMessageType::OKOnly,
        }
    }

    pub fn render(&mut self, app: &mut App, frame: &mut Frame) {
        let area = frame.area();
        let dialog_area = center_widget(area.width / 2, 5, area);

        let block = Block::new()
            .title(" Error ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(app.config.theme.error);

        let text = self.buffer.clone();

        let paragraph = Paragraph::new(text).block(block);

        frame.render_widget(Clear, dialog_area);
        frame.render_widget(paragraph, dialog_area);
    }

    pub fn message_type(&mut self, new_message_type: ErrorMessageType) {
        self.message_type = new_message_type;
    }
}

pub struct ListChoice {
    pub choices: Vec<String>,
    title: String,
}

impl ListChoice {
    pub fn new() -> Self {
        Self {
            choices: vec![],
            title: String::with_capacity(50),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn render(&mut self, app: &mut App, frame: &mut Frame) {
        let area = frame.area();
        let dialog_area = center_widget(area.width / 3, area.height / 4, area);

        let block = Block::new()
            .title(Line::raw(self.title.clone()).centered())
            .borders(Borders::ALL)
            .style(app.config.theme.dialog);

        let lines: Vec<Line> = self
            .choices
            .iter()
            .map(|s| Line::raw(s).style(app.config.theme.dialog).centered())
            .collect();

        let list = List::new(lines)
            .block(block)
            .style(app.config.theme.dialog)
            .highlight_style(app.config.theme.highlight)
            .repeat_highlight_symbol(true);

        frame.render_widget(Clear, dialog_area);
        frame.render_stateful_widget(list, dialog_area, &mut app.list_state);
    }
}
