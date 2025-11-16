use crate::{app::App, config::*, util::parse_goto_expression};
use ratatui::{
    Frame,
    widgets::{Clear, Paragraph},
};

use crate::{
    config::APP_PAGE_SIZE,
    editor::UIState,
    widgets::{ErrorMessage, ErrorMessageType},
};

use ratatui::crossterm::event::{Event, KeyCode};
use std::io::Result;
use tui_input::backend::crossterm::EventHandler;

impl App {
    /// The goto() function checks if the received offset is cached, otherwise
    /// it calls .read_chunk_from_file to fill the right cache block.
    pub fn goto(&mut self, offset: usize) {
        if offset >= self.file_info.size {
            return;
        }

        // If offset is not cached, read and cache the
        // block containing the offset
        if offset > self.reader.cache_offset_last {
            let nblock = offset / APP_CACHE_SIZE;
            self.read_chunk_from_file(nblock).unwrap();
            self.reader.cache_offset_first += nblock * APP_CACHE_SIZE;
            self.reader.cache_offset_last += nblock * APP_CACHE_SIZE;
        } else if offset < self.reader.cache_offset_first {
            self.read_chunk_from_file(offset / APP_CACHE_SIZE).unwrap();
            self.reader.cache_offset_first -= APP_CACHE_SIZE;
            self.reader.cache_offset_last -= APP_CACHE_SIZE;
        }

        // If offset is zero, go to it (it should be cached anyway)
        if offset == 0 {
            self.reader.cache_offset_first = 0;
            self.reader.cache_offset_last = APP_CACHE_SIZE - 1;
            self.reader.page_offset_first = 0;
            self.reader.page_offset_last = APP_PAGE_SIZE - 1;
        } else {
            // Offset is not zero, but is cached. Just go there.
            self.reader.page_offset_first = APP_PAGE_SIZE * (offset / APP_PAGE_SIZE);
            self.reader.page_offset_last = APP_PAGE_SIZE - 1;
        }

        // Update the cursor
        self.hex_view.cursor.y =
            (offset - self.reader.page_offset_first) / self.config.hex_mode_bytes_per_line;
        self.hex_view.cursor.x =
            (offset - self.reader.page_offset_first) % self.config.hex_mode_bytes_per_line;

        // Save current offset (user can press backspace to restore it)
        self.hex_view.last_visited_offset = self.hex_view.offset;
        // Update offset
        self.hex_view.offset = offset;

        // Update offset location in cache. (offset % APP_CACHE_SIZE) / APP_PAGE_SIZE)
        // give the page number within the cache block, then I multiply it by
        // the page size to know how much I have to advance in cache to render
        self.reader.offset_location_in_cache =
            ((offset % APP_CACHE_SIZE) / APP_PAGE_SIZE) * APP_PAGE_SIZE;

        self.reader.page_current = offset / APP_PAGE_SIZE;

        let page_is_aligned = self.file_info.size.is_multiple_of(APP_PAGE_SIZE);

        self.reader.page_current_size =
            if self.reader.page_current == self.reader.page_last && !page_is_aligned {
                self.file_info.size % APP_PAGE_SIZE
            } else {
                APP_PAGE_SIZE
            };
        App::log(self, format!("goto: {:x}", offset));
    }
}

pub fn dialog_goto_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(format!(":{}", app.goto_input.value()));

    frame.render_widget(Clear, app.command_bar_area);
    frame.render_widget(para, app.command_bar_area);
    let x = app.goto_input.visual_cursor();
    frame.set_cursor_position((
        app.command_bar_area.x + 1 + x as u16,
        app.command_bar_area.y,
    ));
}

pub fn dialog_goto_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            KeyCode::Enter => {
                let v = app.goto_input.value();
                if let Ok(mut ofs) = parse_goto_expression(v) {
                    if v.starts_with('+') {
                        ofs += app.hex_view.offset;
                    }
                    if ofs < app.file_info.size {
                        app.dialog_renderer = None;
                        app.state = UIState::Normal;
                        app.goto(ofs);
                    } else {
                        app.dialog_renderer = Some(dialog_goto_error_draw);
                        app.state = UIState::DialogError;
                    }
                } else {
                    app.dialog_renderer = None;
                    app.state = UIState::Normal;
                }
            }
            KeyCode::Char(c) => {
                let v = app.goto_input.value();
                let cursor_pos = app.goto_input.cursor();
                match c {
                    '+' => {
                        if cursor_pos == 0 && !v.contains('+') {
                            app.goto_input.handle_event(event);
                        }
                    }
                    't' => {
                        if !v.is_empty() && cursor_pos == v.len() && !v.contains('t') {
                            app.goto_input.handle_event(event);
                        }
                    }
                    _ => {
                        if c.is_ascii_hexdigit() {
                            app.goto_input.handle_event(event);
                        }
                    }
                }
            }
            _ => {
                app.goto_input.handle_event(event);
            }
        }
    }
    Ok(false)
}

pub fn dialog_goto_error_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = ErrorMessage::new();
    dialog.message_type(ErrorMessageType::OKOnly);
    dialog.buffer = format!(
        "Invalid offset.\nMaximum offset for this file: {:X}",
        app.file_info.size - 1
    );
    dialog.render(app, frame);
}
