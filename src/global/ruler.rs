use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::app::App;

pub fn ruler_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let ruler = "          00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F";
    let ruler_para = Paragraph::new(ruler).style(app.config.theme.offsets);

    frame.render_widget(ruler_para, area);
}
