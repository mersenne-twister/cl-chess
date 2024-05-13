use {
    // super::{BoardOptions, Size, Theme},
    crate::board::{
        print::{BoardOptions, Size, Theme},
        Color as ChessColor, Piece, PieceName,
    },
    lazy_static::lazy_static,
    ratatui::{
        style::{Color, Stylize},
        text::Span,
    },
    std::collections::HashMap,
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd)]
pub struct Tile {
    // Black(Option<Piece>),
    // White(Option<Piece>),
    pub board_color: ChessColor,
    pub piece: Option<Piece>,
}

// // done so that these are accessible from board.rs
// // pub const BOARD_BLACK: Color = Color::Rgb{r: 216, g: 135, b: 85};
// pub const BOARD_BLACK: Color = Color::Rgb(177, 97, 60);
// // pub const BOARD_WHITE: Color = Color::Rgb{r: 254, g: 210, b: 169};
// // pub const BOARD_WHITE: Color = Color::Rgb{r: 240, g: 195, b: 128};
// pub const BOARD_WHITE: Color = Color::Rgb(235, 178, 94);
// // pub const BORDER_BACKGROUND: Color = Color::Rgb{r: 122, g: 77, b: 49};
// pub const BORDER_BACKGROUND: Color = Color::Rgb(82, 46, 22);
// pub const BORDER_TEXT: Color = Color::Rgb(255, 255, 255);
// pub const PIECE_BLACK: Color = Color::Rgb(0, 0, 0);
// pub const PIECE_WHITE: Color = Color::Rgb(255, 255, 255);

// pub const BORDER_LINE_BLANK: &str =
//     "                                                                                  ";
// pub const BORDER_LINE_LETTERS: &str =
//     "         A        B        C        D        E        F        G        H         ";
// pub const BORDER_LINE_LETTERS_REVERSED: &str =
//     "         H        G        F        E        D        C        B        A         ";

impl BoardOptions {
    fn get_tile(&self, tile: Tile) -> Vec<Vec<Span<'_>>> {
        match &self.size {
            // Size::Letters {
            //     different_symbols: diff,
            // } => {
            //     if *diff {
            //         todo!()
            //     }

            //     vec![vec![self.set_colors(
            //         match tile.piece.map(|v| v.name) {
            //             None => " ",
            //             Some(PieceName::Pawn) => "P",
            //             Some(PieceName::Knight) => "N",
            //             Some(PieceName::Bishop) => "B",
            //             Some(PieceName::Rook) => "R",
            //             Some(PieceName::Queen) => "Q",
            //             Some(PieceName::King) => "K",
            //         },
            //         tile,
            //     )]]
            // }
            // Size::UnicodeSymbols {
            //     different_symbols: diff,
            // } => {
            //     if *diff {
            //         todo!()
            //     }

            //     vec![vec![self.set_colors(
            //         match tile.piece.map(|v| v.name) {
            //             None => " ",
            //             Some(PieceName::Pawn) => "♟︎",
            //             Some(PieceName::Knight) => "♞",
            //             Some(PieceName::Bishop) => "♝",
            //             Some(PieceName::Rook) => "♜",
            //             Some(PieceName::Queen) => "♛",
            //             Some(PieceName::King) => "♚",
            //         },
            //         tile,
            //     )]]
            // }

            // Size::UnicodeArt => {
            //     todo!()
            // }
            // Size::TbdLarge => todo!(),

            // match against all that share same fg & bg color
            Size::Letters { .. }
            | Size::UnicodeSymbols { .. }
            | Size::UnicodeArt
            | Size::TbdLarge => self
                .size
                .get_chars(tile.piece)
                .iter()
                .map(|str| vec![self.set_colors((*str).clone(), tile)])
                .collect(),
            Size::BlockArt => todo!(),
        }
    }

    fn set_colors(&self, str: String, tile: Tile) -> Span<'_> {
        // str.bg(self.theme.get_board(tile))
        self.theme
            .get_piece(tile)
            .map_or_else(|| Span::from(str.clone()), |c| str.clone().fg(c))
            .bg(self.theme.get_board(tile))
    }
}

impl Size {
    fn get_chars(&self, piece: Option<Piece>) -> Vec<String> {
        let name = piece.map(|v| v.name);
        match self {
            Size::Letters {
                different_symbols: diff,
            } => vec![if !diff
                || piece.map(|v| v.color).unwrap_or(ChessColor::White) == ChessColor::White
            {
                match name {
                    None => " ",
                    Some(PieceName::Pawn) => "P",
                    Some(PieceName::Knight) => "N",
                    Some(PieceName::Bishop) => "B",
                    Some(PieceName::Rook) => "R",
                    Some(PieceName::Queen) => "Q",
                    Some(PieceName::King) => "K",
                }
            } else {
                match name {
                    None => unreachable!(),
                    Some(PieceName::Pawn) => "p",
                    Some(PieceName::Knight) => "n",
                    Some(PieceName::Bishop) => "b",
                    Some(PieceName::Rook) => "r",
                    Some(PieceName::Queen) => "q",
                    Some(PieceName::King) => "k",
                }
            }
            .to_string()],
            Size::UnicodeSymbols {
                different_symbols: diff,
            } => vec![if !diff
                || piece.map(|v| v.color).unwrap_or(ChessColor::White) == ChessColor::White
            {
                match name {
                    None => " ",
                    Some(PieceName::Pawn) => "♟︎",
                    Some(PieceName::Knight) => "♞",
                    Some(PieceName::Bishop) => "♝",
                    Some(PieceName::Rook) => "♜",
                    Some(PieceName::Queen) => "♛",
                    Some(PieceName::King) => "♚",
                }
            } else {
                match name {
                    None => unreachable!(),
                    Some(PieceName::Pawn) => "♙",
                    Some(PieceName::Knight) => "♘",
                    Some(PieceName::Bishop) => "♗",
                    Some(PieceName::Rook) => "♖",
                    Some(PieceName::Queen) => "♕",
                    Some(PieceName::King) => "♔",
                }
            }
            .to_string()],
            Size::UnicodeArt => Vec::from(match name {
                // see `/chess-pieces` (in root dir)
                None => ["     ", "     ", "     ", "     "],
                Some(PieceName::Pawn) => ["     ", "  •  ", "  │  ", "  ┴  "],
                // find one that combines first and second
                Some(PieceName::Knight) => ["  ╥  ", " /╣  ", "  ║  ", "  ╨  "],
                // make shorter?
                Some(PieceName::Bishop) => ["  │  ", "  ║  ", "  │  ", "  ┴  "],
                // move top down one?
                Some(PieceName::Rook) => [" ╘╬╛ ", "  ║  ", "  ║  ", "  ╨  "],
                // use ☼ instead op •?
                Some(PieceName::Queen) => ["• • •", " \\╫/ ", "  ║  ", "  ╨  "],
                Some(PieceName::King) => ["  +  ", "  ╫  ", "  ║  ", "  ╨  "],
            })
            .iter()
            .map(|str| format!("  {}  ", str))
            .collect(),
            Size::TbdLarge => todo!(),
            // this fn only used for that fancy @ match arm above
            Size::BlockArt => unreachable!(),
        }
    }
}
