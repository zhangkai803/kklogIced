#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    ThemeSelected(Theme),
    AddSource,
    Quit,
}