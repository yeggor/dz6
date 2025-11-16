use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, editor::UIState, global};

use std::io::Result;

pub fn handle_global_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        // quit
        KeyCode::Esc => app.running = false,
        KeyCode::Enter => app.switch_editor_mode(),
        KeyCode::Char('l') => {
            if key.modifiers.contains(KeyModifiers::ALT) {
                app.state = UIState::DialogLog;
                app.dialog_renderer = Some(global::log::dialog_log_draw);
            }
        }
        KeyCode::Char('?') => {
            app.state = UIState::DialogCalculator;
            app.dialog_renderer = Some(global::calculator::dialog_calculator_draw);
        }
        _ => {}
    }
    Ok(false)
}
