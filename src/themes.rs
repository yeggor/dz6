use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub main: Style,
    pub zeroes: Style,
    pub offsets: Style,
    pub changed_bytes: Style,
    pub highlight: Style,
    pub byte_highlight: Style,
    pub topbar: Style,
    pub error: Style,
    pub editing: Style,
    pub dialog: Style,
}

pub const VSCODE: Theme = Theme {
    offsets: Style::new()
        .fg(Color::from_u32(0x569cd6))
        .bg(Color::from_u32(0x1e1e1e))
        .add_modifier(Modifier::BOLD),
    main: Style::new()
        .fg(Color::from_u32(0xd4d4d4))
        .bg(Color::from_u32(0x1e1e1e))
        .add_modifier(Modifier::BOLD),
    zeroes: Style::new()
        .fg(Color::from_u32(0x949494))
        .bg(Color::from_u32(0x1e1e1e))
        .add_modifier(Modifier::BOLD),
    dialog: Style::new()
        .fg(Color::Rgb(204, 204, 204))
        .bg(Color::from_u32(0x081e32))
        .add_modifier(Modifier::BOLD),
    changed_bytes: Style::new()
        .fg(Color::Rgb(255, 215, 0))
        .bg(Color::from_u32(0x1e1e1e)),
    highlight: Style::new()
        .fg(Color::Rgb(255, 255, 255))
        .bg(Color::Rgb(38, 79, 120)),
    byte_highlight: Style::new().fg(Color::White).bg(Color::Red),
    topbar: Style::new()
        .fg(Color::Rgb(204, 204, 204))
        .bg(Color::from_u32(0x3c3c3c)),
    error: Style::new()
        .fg(Color::Rgb(255, 85, 85))
        .bg(Color::from_u32(0x400000)),
    editing: Style::new()
        .fg(Color::from_u32(0x1e1e1e))
        .bg(Color::Rgb(255, 215, 0))
        .add_modifier(Modifier::RAPID_BLINK),
};
