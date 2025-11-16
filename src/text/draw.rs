use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Clear, Paragraph, Wrap},
};

use crate::app::App;

pub fn text_contents_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let (mut text, _, had_error) = app.text_view.table.decode(&app.buffer);

    if had_error {
        text = text
            .chars()
            .map(|c| if c.is_ascii_graphic() { c } else { ' ' })
            .collect();
    }

    app.text_view.lines_to_show = text.lines().count();

    let paragraph = Paragraph::new(text)
        .style(app.config.theme.main)
        .wrap(Wrap { trim: true })
        .scroll(app.text_view.scroll_offset);

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}
