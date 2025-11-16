use ratatui::{
    Frame, symbols,
    widgets::{Block, Borders, Clear, Padding, Paragraph},
};

use crate::{app::App, util::center_widget};

pub fn dialog_help_draw(app: &mut App, frame: &mut Frame) {
    let text = "Coming late";
    let para = Paragraph::new(text);

    let dialog_area = center_widget(
        frame.area().width / 2,
        frame.area().height / 2,
        frame.area(),
    );

    let block = Block::new()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE)
        .style(app.config.theme.dialog)
        .padding(Padding::horizontal(1));

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(para.block(block), dialog_area);
}
