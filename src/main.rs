mod app;
mod config;
mod draw;
mod editor;
mod events;
mod global;
mod hex;
mod layout;
mod reader;
mod text;
mod themes;
mod util;
mod widgets;

use clap::Parser;

use app::App;

/// vim-like hexadecimal editor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to edit
    file: String,

    /// Initial file offset (hexadecimal is the default; add a `t` suffix for decimal)
    #[arg(short, long, default_value = "0")]
    offset: String,
}

fn main() {
    let args = Args::parse();
    let mut app = App::new();
    let initial_offset = util::parse_goto_expression(&args.offset).unwrap_or_default();

    app.load_file(&args.file, initial_offset)
        .expect("cannot open target file");
    app.list_state.select_first();

    let mut terminal = ratatui::init();

    while app.running {
        terminal
            .draw(|f| draw::draw(f, &mut app))
            .expect("failed to draw frame");

        events::handle_events(&mut app).expect("unable to read events");
    }

    ratatui::restore();
}
