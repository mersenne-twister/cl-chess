use {
    super::Game,
    // ascii::Tile,
    ratatui::{
        style::{Color, Stylize},
        widgets::Paragraph,
    },
    std::error::Error,
    termchess_common::TResult,
    termchess_engine::board::{self, print::BoardOptions, Board, Color as ChessColor, Piece},
};

fn board_widget(game: Game, options: BoardOptions) -> Paragraph<'static> {
    // use unicode characters tempoarily
    todo!()
}
