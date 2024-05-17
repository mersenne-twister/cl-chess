use {
    super::Game,
    // ascii::Tile,
    ratatui::{
        style::{Color, Stylize},
        widgets::Paragraph,
    },
    std::error::Error,
    termchess_common::TResult,
    termchess_engine::board::{
        self,
        print::{BoardOptions, Size},
        Board, Color as ChessColor, Piece,
    },
};

impl Game {
    pub fn board_widget(&self) -> TResult<Paragraph> {
        // calc if one should be highlighted

        let board = self
            .board
            .print(&self.board_options, ChessColor::White, None);

        Ok(Paragraph::new(board))
    }
}
