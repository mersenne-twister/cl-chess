use {
    super::Game,
    ratatui::{
        style::Style,
        text::{Line, Text},
        widgets::{Block, Borders, Padding, Paragraph},
    },
};

pub struct Message {
    message_type: MessageType,
    content: String,
}

impl Message {
    pub fn new(message_type: MessageType, content: String) -> Self {
        Self {
            message_type,
            content,
        }
    }
}

pub enum MessageType {
    Chat,
    Network,
    CannotMove,
    // and so on
    // this exists so they can have separate styling
    // exampli gratia:
    // Cannot Move:
    // | Cannot move
    // | such and such
    // | because such
    // | and such.
}

impl MessageType {
    pub fn title_style(&self) -> Style {
        let style = Style::default();

        match *self {
            MessageType::Chat => style,
            MessageType::Network => style,
            MessageType::CannotMove => style,
        }
    }

    pub fn content_style(&self) -> Style {
        let style = Style::default();

        match *self {
            MessageType::Chat => style,
            MessageType::Network => style,
            MessageType::CannotMove => style,
        }
    }
}

impl Game {
    pub fn messages_widget(&self) -> Paragraph {
        let messages = 
// Text::from(
            self.messages
                .iter()
                .map(|v| Line::from(format!(" {}", v.content.clone())))
                .collect::<Vec<_>>();
        // );

        let barrier = vec![Line::from("─".repeat( self.messages_width as usize - 2))];

        Paragraph::new([messages, barrier].concat())
        .block(
            Block::default().borders(Borders::ALL), // .padding(Padding::left(1)),
        )
    }
}
