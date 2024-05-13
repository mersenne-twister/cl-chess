use {
    super::{BoardOptions, Size, Theme},
    lazy_static::lazy_static,
    ratatui::{
        style::{Color, Stylize},
        text::Span,
    },
    std::collections::HashMap,
    termchess_engine::board::{Color as ChessColor, Piece, PieceName},
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
        match self.size {
            Size::Letters {
                different_symbols: diff,
            } => {
                if diff {
                    todo!()
                }

                vec![vec![self.set_colors(
                    match tile.piece.map(|v| v.name) {
                        None => " ",
                        Some(PieceName::Pawn) => "P",
                        Some(PieceName::Knight) => "N",
                        Some(PieceName::Bishop) => "B",
                        Some(PieceName::Rook) => "R",
                        Some(PieceName::Queen) => "Q",
                        Some(PieceName::King) => "K",
                    },
                    tile,
                )]]
            }
            Size::UnicodeSymbols {
                different_symbols: diff,
            } => {
                if diff {
                    todo!()
                }

                vec![vec![self.set_colors(
                    match tile.piece.map(|v| v.name) {
                        None => " ",
                        Some(PieceName::Pawn) => "♟︎",
                        Some(PieceName::Knight) => "♞",
                        Some(PieceName::Bishop) => "♝",
                        Some(PieceName::Rook) => "♜",
                        Some(PieceName::Queen) => "♛",
                        Some(PieceName::King) => "♚",
                    },
                    tile,
                )]]
            }
            Size::UnicodeArt => {
                todo!()
            }
            Size::BlockArt => todo!(),
            Size::TbdLarge => todo!(),
        }
    }

    fn set_colors(&self, str: &'static str, tile: Tile) -> Span<'_> {
        // str.bg(self.theme.get_board(tile))
        self.theme
            .get_piece(tile)
            .map_or_else(|| Span::from(str), |c| str.fg(c))
            .bg(self.theme.get_board(tile))
    }
}

impl Piece {}

fn make_hashmap() -> HashMap<(Tile, Size), Vec<Vec<Span<'static>>>> {
    // let mut pieces_ascii = HashMap::new();

    // let board_black = BOARD_BLACK.into();
    // let board_white = BOARD_WHITE.into();
    // let piece_black = PIECE_BLACK.into();
    // let piece_white = PIECE_WHITE.into();

    // let blank = [
    //     vec!["         "],
    //     vec!["         "],
    //     vec!["         "],
    //     vec!["         "],
    //     vec!["         "],
    // ];

    // let pawn = [
    //     vec!["         "],
    //     vec!["    ", " ", "    "],
    //     vec!["   ", "   ", "   "],
    //     vec!["    ", " ", "    "],
    //     vec!["   ", "   ", "   "],
    // ];

    // let knight = [
    //     vec!["    ", "  ", "   "],
    //     vec!["  ", "     ", "  "],
    //     vec![" ", "  ", " ", "    ", " "],
    //     vec!["   ", "    ", "  "],
    //     vec!["  ", "     ", "  "],
    // ];

    // let bishop = [
    //     vec!["    ", " ", "    "],
    //     vec!["   ", "   ", "   "],
    //     vec!["   ", "   ", "   "],
    //     vec!["    ", " ", "    "],
    //     vec!["  ", "     ", "  "],
    // ];

    // let rook = [
    //     vec!["  ", " ", " ", " ", " ", " ", "  "],
    //     vec!["  ", "     ", "  "],
    //     vec!["   ", "   ", "   "],
    //     vec!["   ", "   ", "   "],
    //     vec!["  ", "     ", "  "],
    // ];

    // let queen = [
    //     vec![" ", " ", "  ", " ", "  ", " ", " "],
    //     vec!["  ", " ", " ", " ", " ", " ", "  "],
    //     vec!["   ", "   ", "   "],
    //     vec!["   ", "   ", "   "],
    //     vec!["  ", "     ", "  "],
    // ];

    // let king = [
    //     vec!["    ", " ", "    "],
    //     vec![" ", "  ", " ", " ", " ", "  ", " "],
    //     vec!["", " ", "  ", " ", " ", " ", "  ", " "],
    //     vec![" ", " ", "  ", " ", "  ", " ", " "],
    //     // vec!["", "    ", " ", "    "],
    //     // vec![" ", "       ", " "],
    //     vec!["  ", "     ", "  "],
    // ];

    // // create an array of the pieces we need to iterate over
    // // let pieces = [
    // //     &[None][..],
    // //     &[pawn, knight, bishop, rook, queen, king].map(|piece| Some(piece)),
    // // ]
    // // .concat();
    // let pieces = [pawn, knight, bishop, rook, queen, king];

    // for piece in pieces {
    //     pieces_ascii.insert(
    //         Tile::Black(Some(Piece::new(PieceName::Pawn, PieceColor::White))),
    //         make_piece(&piece, board_black, Some(piece_white)),
    //     );
    //     pieces_ascii.insert(
    //         Tile::White(Some(Piece::new(PieceName::Pawn, PieceColor::White))),
    //         make_piece(&piece, board_white, Some(piece_white)),
    //     );
    //     pieces_ascii.insert(
    //         Tile::Black(Some(Piece::new(PieceName::Pawn, PieceColor::Black))),
    //         make_piece(&piece, board_black, Some(piece_black)),
    //     );
    //     pieces_ascii.insert(
    //         Tile::White(Some(Piece::new(PieceName::Pawn, PieceColor::Black))),
    //         make_piece(&piece, board_white, Some(piece_black)),
    //     );
    // }

    // // pieces_ascii.insert(Tile::Black(None), make_piece(&blank, board_black, None));
    // // pieces_ascii.insert(Tile::White(None), make_piece(&blank, board_white, None));

    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Pawn(PieceColor::White))),
    // //     make_piece(&pawn, board_black, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Pawn(PieceColor::White))),
    // //     make_piece(&pawn, board_white, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Pawn(PieceColor::Black))),
    // //     make_piece(&pawn, board_black, Some(piece_black)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Pawn(PieceColor::Black))),
    // //     make_piece(&pawn, board_white, Some(piece_black)),
    // // );

    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Knight(PieceColor::White))),
    // //     make_piece(&knight, board_black, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Knight(PieceColor::White))),
    // //     make_piece(&knight, board_white, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Knight(PieceColor::Black))),
    // //     make_piece(&knight, board_black, Some(piece_black)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Knight(PieceColor::Black))),
    // //     make_piece(&knight, board_white, Some(piece_black)),
    // // );

    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Bishop(PieceColor::White))),
    // //     make_piece(&bishop, board_black, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Bishop(PieceColor::White))),
    // //     make_piece(&bishop, board_white, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Bishop(PieceColor::Black))),
    // //     make_piece(&bishop, board_black, Some(piece_black)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Bishop(PieceColor::Black))),
    // //     make_piece(&bishop, board_white, Some(piece_black)),
    // // );

    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Rook(PieceColor::White))),
    // //     make_piece(&rook, board_black, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Rook(PieceColor::White))),
    // //     make_piece(&rook, board_white, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Rook(PieceColor::Black))),
    // //     make_piece(&rook, board_black, Some(piece_black)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Rook(PieceColor::Black))),
    // //     make_piece(&rook, board_white, Some(piece_black)),
    // // );

    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Queen(PieceColor::White))),
    // //     make_piece(&queen, board_black, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Queen(PieceColor::White))),
    // //     make_piece(&queen, board_white, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::Queen(PieceColor::Black))),
    // //     make_piece(&queen, board_black, Some(piece_black)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::Queen(PieceColor::Black))),
    // //     make_piece(&queen, board_white, Some(piece_black)),
    // // );

    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::King(PieceColor::White))),
    // //     make_piece(&king, board_black, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::King(PieceColor::White))),
    // //     make_piece(&king, board_white, Some(piece_white)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::Black(Some(Piece::King(PieceColor::Black))),
    // //     make_piece(&king, board_black, Some(piece_black)),
    // // );
    // // pieces_ascii.insert(
    // //     Tile::White(Some(Piece::King(PieceColor::Black))),
    // //     make_piece(&king, board_white, Some(piece_black)),
    // // );

    // pieces_ascii

    todo!()
}

lazy_static! {
    // pub static ref PIECES_ASCII: HashMap<Tile, [ColoredString; 5]> = {
        pub static ref PIECES_ASCII: HashMap<(Tile, Size), Vec<Vec<Span<'static>>>> = {
            make_hashmap()
    };
}

/// Construct a piece from string literals, and the colour of the background and piece
///
/// # Invariants:
/// Assumes that the piece begins with white.
/// * To get around this, have the first value in the vec be `""`
///
/// # Panics:
/// Will panic if piece_colour is `None` and the len of any of the widths is higher than 1
fn make_piece(
    widths: &[Vec<&str>; 5],
    tile_colour: Color,
    piece_colour: Option<Color>,
) -> [String; 5] {
    let mut piece: [String; 5] = Default::default();

    for (i, line) in widths.iter().enumerate() {
        let mut section = String::new();
        for (j, string) in line.iter().enumerate() {
            section.push_str(
                &(if j & 1 == 0 {
                    format!("{}", string.bg(tile_colour))
                } else {
                    format!("{}", string.bg(piece_colour.unwrap()))
                }),
            );
        }
        piece[i].push_str(&section);
    }

    piece
}
