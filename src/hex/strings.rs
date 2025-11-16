use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::Alignment,
    symbols,
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph},
};

use tui_input::backend::crossterm::EventHandler;

use std::io::Result;

use crate::{
    app::{App, FoundString},
    config::APP_CACHE_SIZE,
    editor::UIState,
    util::center_widget,
};

use regex::{Regex, RegexBuilder};

impl App {
    pub fn load_strings(&mut self, force_read: bool) {
        // If the string list is already filled, just reuse it
        if force_read {
            self.strings.clear();
        }

        if !self.strings.is_empty() {
            return;
        }

        let mut siz = 0;
        let mut candidate = String::new();

        // Save the original loaded block number (will be restored later)
        let original_block_number = self.reader.cache_block_number;

        // Read the entire file by blocks and find strings in them

        let default_regex = Regex::new(".*").unwrap();
        // let re = Regex::new(&self.string_regex).unwrap_or(default_regex);
        let re = RegexBuilder::new(&self.string_regex)
            .case_insensitive(true)
            .build()
            .unwrap_or(default_regex);

        'outer: for block in 0..self.reader.cache_blocks {
            let _ = self.read_chunk_from_file(block);
            for (i, byte) in self.buffer.iter().enumerate() {
                if byte.is_ascii_graphic() || *byte == b' ' {
                    candidate.push(*byte as char);
                    siz += 1;
                } else {
                    if siz >= self.config.minimum_string_length && re.is_match(&candidate) {
                        let ofs = i + APP_CACHE_SIZE * block - siz;
                        self.strings.push(FoundString {
                            offset: ofs,
                            content: candidate.clone(),
                            size: siz,
                        });
                        if self.strings.len() >= self.config.maximum_strings_to_show {
                            // too many strings :(
                            break 'outer;
                        }
                    }
                    candidate.clear();
                    siz = 0;
                }
            }
        }

        // Restore previously loaded block
        let _ = self.read_chunk_from_file(original_block_number);
    }
}

pub fn dialog_strings_draw(app: &mut App, frame: &mut Frame) {
    let mut items = Vec::new();

    for i in &app.strings {
        let ofs = i.offset;
        let content = i.content.clone();
        let siz = i.size;
        if siz >= app.config.minimum_string_length {
            items.push(ListItem::from(format!("{ofs:08X}  {content}")))
        }
    }

    let title_bottom = format!(" Minimun length = {} ", app.config.minimum_string_length);

    let strings_count = if app.strings.len() == app.config.maximum_strings_to_show {
        format!("{}+", app.config.maximum_strings_to_show)
    } else {
        format!("{}", app.strings.len())
    };

    let list = List::new(items)
        .style(app.config.theme.dialog)
        .block(
            Block::bordered()
                .title(format!(" Strings ({}) ", strings_count))
                .title_bottom(title_bottom)
                .title_alignment(Alignment::Center)
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(app.config.theme.highlight)
        .repeat_highlight_symbol(true);

    let width = frame.area().width / 2;
    let height = frame.area().height / 2 + 4;
    let dialog_area = center_widget(width, height, frame.area());

    frame.render_widget(Clear, dialog_area);
    frame.render_stateful_widget(list, dialog_area, &mut app.list_state);
}

pub fn dialog_strings_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.dialog_renderer = None;
            app.state = UIState::Normal;
        }
        KeyCode::Down => {
            app.list_state.select_next();
        }
        KeyCode::Up => {
            app.list_state.select_previous();
        }
        KeyCode::PageDown => {
            app.list_state.scroll_down_by(30);
        }
        KeyCode::PageUp => {
            app.list_state.scroll_up_by(30);
        }
        KeyCode::Home => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                app.list_state.select_first();
            } else if let Some(n) = app.list_state.selected() {
                // we show 30 strings at a time, so this will select
                // the string at the top of the list
                let new_index = n.saturating_sub(29);
                app.list_state.select(Some(new_index));
            }
        }
        KeyCode::End => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                app.list_state.select_last();
            } else if let Some(n) = app.list_state.selected() {
                let new_index = n + 29;
                app.list_state.select(Some(new_index));
            }
        }
        KeyCode::Enter => {
            if let Some(choice) = app.list_state.selected() {
                if choice > app.strings.len() {
                    App::log(
                        app,
                        "wtf {choice} is greater than `app.strings.len()`, dunno how".to_string(),
                    );
                    return Ok(true);
                }
                app.goto(app.strings[choice].offset);
                app.state = UIState::Normal;
                app.dialog_renderer = None;
            }
        }
        KeyCode::Char('+') => {
            app.config.minimum_string_length += 1;
            app.load_strings(true);
        }
        KeyCode::Char('-') => {
            if app.config.minimum_string_length > 1 {
                app.config.minimum_string_length -= 1;
                app.load_strings(true);
            }
        }
        KeyCode::Char('R') => {
            app.load_strings(true);
        }
        KeyCode::Char('f') => {
            app.state = UIState::DialogStringsRegex;
            app.dialog_2nd_renderer = Some(dialog_strings_regex_draw);
        }
        _ => {}
    }
    Ok(false)
}

pub fn dialog_strings_regex_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(app.hex_view.strings_regex_input.value());

    let dialog_area = center_widget(frame.area().width / 3, 3, frame.area());

    let block = Block::new()
        .title(" Filter regex ")
        .borders(Borders::ALL)
        .border_set(symbols::border::PLAIN)
        .style(app.config.theme.main)
        .padding(Padding::horizontal(1));

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(para.block(block), dialog_area);
    let x = app.hex_view.strings_regex_input.visual_cursor();
    frame.set_cursor_position((dialog_area.x + 2 + x as u16, dialog_area.y + 1));
}

pub fn dialog_strings_regex_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_2nd_renderer = None;
                app.state = UIState::DialogStrings;
            }
            KeyCode::Enter => {
                app.string_regex = String::from(app.hex_view.strings_regex_input.value());
                app.dialog_2nd_renderer = None;
                app.state = UIState::DialogStrings;
                app.load_strings(true);
            }
            _ => {
                app.hex_view.strings_regex_input.handle_event(event);
            }
        }
    }
    Ok(false)
}
