#[derive(PartialEq)]
pub enum AppView {
    Text,
    Hex,
}

impl AppView {
    pub fn next(&mut self) {
        match self {
            AppView::Text => *self = AppView::Hex,
            AppView::Hex => *self = AppView::Text,
        }
    }
}

#[derive(PartialEq)]
pub enum UIState {
    DialogCalculator,
    DialogComment,
    DialogEncoding,
    DialogError,
    DialogGoto,
    DialogHelp,
    DialogLog,
    DialogNames,
    DialogNamesRegex,
    DialogSearch,
    DialogStrings,
    DialogStringsRegex,
    HexEditing,
    Normal,
}
