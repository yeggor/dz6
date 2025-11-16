use std::num::ParseIntError;

use ratatui::layout::Rect;

/// This function is used returns the right offset
/// for goto(). Hexa is the default. Add 't' suffix for decimal
pub fn parse_goto_expression(expr: &str) -> Result<usize, ParseIntError> {
    if expr.ends_with("t") {
        expr[0..expr.len() - 1].parse()
    } else {
        usize::from_str_radix(expr, 16)
    }
}

pub fn center_widget(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.width / 2 - width / 2,
        y: area.height / 2 - height / 2 - 1,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_expr_test() {
        assert_eq!(Ok(255), parse_goto_expression("ff"));
        assert_eq!(Ok(16), parse_goto_expression("10"));
        assert_eq!(Ok(255), parse_goto_expression("ff"));
        assert_eq!(Ok(255), parse_goto_expression("255t"));
        // Errors
        assert!(parse_goto_expression("255th").is_err());
        assert!(parse_goto_expression("255ht").is_err());
        assert!(parse_goto_expression("ht").is_err());
        assert!(parse_goto_expression("h3").is_err());
        assert!(parse_goto_expression("-5").is_err());
        assert!(parse_goto_expression("4h4h").is_err());
    }
}
