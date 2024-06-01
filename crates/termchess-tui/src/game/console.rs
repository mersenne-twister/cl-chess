use {
    super::Game,
    ratatui::{text::Text, widgets::Paragraph},
};

impl Game {
    pub fn console_widget(&self) -> Paragraph {
        Paragraph::new(Text::from(format!("> {}", self.console)))
    }
}
