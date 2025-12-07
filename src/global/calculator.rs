use crate::{app::App, util::center_widget};

use ratatui::{
    Frame,
    layout::Rect,
    symbols,
    widgets::{Block, Borders, Clear, Padding, Paragraph},
};

use crate::editor::UIState;

use ratatui::crossterm::event::{Event, KeyCode};
use std::{collections::HashSet, io::Result};
use tui_input::{Input, backend::crossterm::EventHandler};

use evalexpr::*;

#[derive(Default)]
pub struct Calculator {
    pub input: Input,
    pub context: HashMapContext,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    // history_set is a HashSet to avoid duplicates, although users
    // can bypass that with something like "1+1" != "1 + 1"
    pub history_set: HashSet<String>,
    pub result: i64,
}

impl Calculator {
    pub fn push_history(&mut self, entry: String) {
        if !entry.trim().is_empty() && self.history_set.insert(entry.clone()) {
            self.history.push(entry);
        }
        self.history_index = None;
    }
    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let len = self.history.len();

        let new_index = match self.history_index {
            None => len - 1,
            Some(0) => 0,
            Some(i) => i - 1,
        };

        self.history_index = Some(new_index);
        self.input = Input::new(self.history[new_index].clone());
    }
    pub fn history_down(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let len = self.history.len();

        let new_index = match self.history_index {
            None => 0,
            Some(i) => (i + 1).min(len - 1),
        };

        self.history_index = Some(new_index);
        self.input = Input::new(self.history[new_index].clone());
    }
}

pub fn dialog_calculator_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(app.calculator.input.value());

    let dialog_area = center_widget(68, 8, frame.area());

    let block = Block::new()
        .title(" Calculator ")
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE)
        .style(app.config.theme.dialog)
        .padding(Padding::horizontal(1));

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(para.block(block), dialog_area);
    let x = app.calculator.input.visual_cursor();
    frame.set_cursor_position((dialog_area.x + 2 + x as u16, dialog_area.y + 1));

    let para_result = Paragraph::new(format!(
        "{:<18} {:<21} {:<21}\n{:<18X} {:<21} {:<21}\n\n{}\n{:064b}",
        "Hex",
        "Unsigned",
        "Signed",
        app.calculator.result,
        app.calculator.result,
        app.calculator.result,
        "Bin",
        app.calculator.result
    ))
    .style(app.config.theme.main);
    let result_area = Rect {
        x: dialog_area.x + 2,
        y: dialog_area.y + 2,
        width: dialog_area.width - 4,
        height: dialog_area.height - 3,
    };

    frame.render_widget(Clear, result_area);
    frame.render_widget(para_result, result_area);
}

fn load_variables(app: &mut App) {
    // constants
    app.calculator
        .context
        .set_value("i64::MAX".to_string(), Value::from_int(i64::MAX))
        .unwrap();

    app.calculator
        .context
        .set_value("i64::MIN".to_string(), Value::from_int(i64::MIN))
        .unwrap();

    // comments
    for cmt in &app.hex_view.comment_name_list {
        app.calculator
            .context
            .set_value(cmt.comment.clone(), Value::from_int(cmt.offset as i64))
            .unwrap();
        // what happens if an offset is greater than i64::MAX?
    }

    // @B -> unsigned byte
    if let Some(b) = app.read_u8(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@B".to_string(), Value::from_int(b as i64))
            .unwrap();
    }

    // @b -> signed byte
    if let Some(b) = app.read_i8(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@b".to_string(), Value::from_int(b as i64))
            .unwrap();
    }

    // @W -> usigned word
    if let Some(b) = app.read_u16(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@W".to_string(), Value::from_int(b as i64))
            .unwrap();
    }

    // @w -> signed word
    if let Some(b) = app.read_i16(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@w".to_string(), Value::from_int(b as i64))
            .unwrap();
    }

    // @D -> usigned dword
    if let Some(b) = app.read_u32(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@D".to_string(), Value::from_int(b as i64))
            .unwrap();
    }

    // @d -> signed dword
    if let Some(b) = app.read_i32(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@d".to_string(), Value::from_int(b as i64))
            .unwrap();
    }

    // @Q -> usigned word
    if let Some(b) = app.read_u64(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@Q".to_string(), Value::from_int(b as i64))
            .unwrap();
    }

    // @q -> signed word
    if let Some(b) = app.read_i64(app.hex_view.offset) {
        app.calculator
            .context
            .set_value("@q".to_string(), Value::from_int(b))
            .unwrap();
    }

    // @o -> current offset
    app.calculator
        .context
        .set_value(
            "@o".to_string(),
            Value::from_int(app.hex_view.offset as i64),
        )
        .unwrap();

    // @O -> previous offset
    app.calculator
        .context
        .set_value(
            "@O".to_string(),
            Value::from_int(app.hex_view.last_visited_offset as i64),
        )
        .unwrap();

    app.calculator
        .context
        .set_function(
            String::from("fu8"),
            Function::new(|arg| {
                if let Ok(ofs) = arg.as_int() {
                    Ok(Value::Int(ofs + 10))
                } else if let Ok(float) = arg.as_number() {
                    Ok(Value::Float(float / 2.0))
                } else {
                    Err(EvalexprError::expected_number(arg.clone()))
                }
            }),
        )
        .unwrap();
}

pub fn dialog_calculator_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            KeyCode::Enter => {
                let input_expr = app.calculator.input.value().to_string();

                load_variables(app);

                let result = eval_with_context_mut(&input_expr, &mut app.calculator.context);

                app.calculator.push_history(input_expr);

                match result {
                    Ok(v) => {
                        if let Ok(a) = v.as_int() {
                            app.calculator.result = a;
                        }
                    }
                    Err(_e) => {
                        // app.calculator.history.push(format!("Error: {}", e));
                    }
                }
            }
            KeyCode::Up => {
                app.calculator.history_up();
            }
            KeyCode::Down => {
                app.calculator.history_down();
            }
            _ => {
                app.calculator.input.handle_event(event);
            }
        }
    }
    Ok(false)
}
