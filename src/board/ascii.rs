use {
    super::{Piece, PieceColor},
    colored::{ColoredString, Colorize, CustomColor},
    lazy_static::lazy_static,
    std::collections::HashMap,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Tile {
    Black(Option<Piece>),
    White(Option<Piece>),
}

fn make_hashmap() -> HashMap<Tile, [String; 5]> {
    let mut pieces_ascii = HashMap::new();

    let board_black = CustomColor::new(216, 135, 85);
    let board_white = CustomColor::new(254, 210, 169);
    let piece_black = CustomColor::new(0, 0, 0);
    let piece_white = CustomColor::new(255, 255, 255);

    let blank = [
        vec!["         "],
        vec!["         "],
        vec!["         "],
        vec!["         "],
        vec!["         "],
    ];

    let pawn = [
        vec!["         "],
        vec!["    ", " ", "    "],
        vec!["   ", "   ", "   "],
        vec!["    ", " ", "    "],
        vec!["   ", "   ", "   "],
    ];

    let knight = [
        vec!["    ", "  ", "   "],
        vec!["  ", "     ", "  "],
        vec![" ", "  ", " ", "    ", " "],
        vec!["   ", "    ", "  "],
        vec!["  ", "     ", "  "],
    ];

    let bishop = [
        vec!["    ", " ", "    "],
        vec!["   ", "   ", "   "],
        vec!["   ", "   ", "   "],
        vec!["    ", " ", "    "],
        vec!["  ", "     ", "  "],
    ];

    let rook = [
        vec!["  ", " ", " ", " ", " ", " ", "  "],
        vec!["  ", "     ", "  "],
        vec!["   ", "   ", "   "],
        vec!["   ", "   ", "   "],
        vec!["  ", "     ", "  "],
    ];

    let queen = [
        vec![" ", " ", "  ", " ", "  ", " ", " "],
        vec!["  ", " ", " ", " ", " ", " ", "  "],
        vec!["   ", "   ", "   "],
        vec!["   ", "   ", "   "],
        vec!["  ", "     ", "  "],
    ];

    let king = [
        vec!["    ", " ", "    "],
        vec![" ", "  ", " ", " ", " ", "  ", " "],
        vec!["", " ", "  ", " ", " ", " ", "  ", " "],
        vec![" ", " ", "  ", " ", "  ", " ", " "],
        // vec!["", "    ", " ", "    "],
        // vec![" ", "       ", " "],
        vec!["  ", "     ", "  "],
    ];

    pieces_ascii.insert(Tile::Black(None), make_piece(&blank, board_black, None));
    pieces_ascii.insert(Tile::White(None), make_piece(&blank, board_white, None));

    pieces_ascii.insert(
        Tile::Black(Some(Piece::Pawn(PieceColor::White))),
        make_piece(&pawn, board_black, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Pawn(PieceColor::White))),
        make_piece(&pawn, board_white, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::Black(Some(Piece::Pawn(PieceColor::Black))),
        make_piece(&pawn, board_black, Some(piece_black)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Pawn(PieceColor::Black))),
        make_piece(&pawn, board_white, Some(piece_black)),
    );

    pieces_ascii.insert(
        Tile::Black(Some(Piece::Knight(PieceColor::White))),
        make_piece(&knight, board_black, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Knight(PieceColor::White))),
        make_piece(&knight, board_white, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::Black(Some(Piece::Knight(PieceColor::Black))),
        make_piece(&knight, board_black, Some(piece_black)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Knight(PieceColor::Black))),
        make_piece(&knight, board_white, Some(piece_black)),
    );

    pieces_ascii.insert(
        Tile::Black(Some(Piece::Bishop(PieceColor::White))),
        make_piece(&bishop, board_black, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Bishop(PieceColor::White))),
        make_piece(&bishop, board_white, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::Black(Some(Piece::Bishop(PieceColor::Black))),
        make_piece(&bishop, board_black, Some(piece_black)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Bishop(PieceColor::Black))),
        make_piece(&bishop, board_white, Some(piece_black)),
    );

    pieces_ascii.insert(
        Tile::Black(Some(Piece::Rook(PieceColor::White))),
        make_piece(&rook, board_black, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Rook(PieceColor::White))),
        make_piece(&rook, board_white, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::Black(Some(Piece::Rook(PieceColor::Black))),
        make_piece(&rook, board_black, Some(piece_black)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Rook(PieceColor::Black))),
        make_piece(&rook, board_white, Some(piece_black)),
    );

    pieces_ascii.insert(
        Tile::Black(Some(Piece::Queen(PieceColor::White))),
        make_piece(&queen, board_black, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Queen(PieceColor::White))),
        make_piece(&queen, board_white, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::Black(Some(Piece::Queen(PieceColor::Black))),
        make_piece(&queen, board_black, Some(piece_black)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::Queen(PieceColor::Black))),
        make_piece(&queen, board_white, Some(piece_black)),
    );

    pieces_ascii.insert(
        Tile::Black(Some(Piece::King(PieceColor::White))),
        make_piece(&king, board_black, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::King(PieceColor::White))),
        make_piece(&king, board_white, Some(piece_white)),
    );
    pieces_ascii.insert(
        Tile::Black(Some(Piece::King(PieceColor::Black))),
        make_piece(&king, board_black, Some(piece_black)),
    );
    pieces_ascii.insert(
        Tile::White(Some(Piece::King(PieceColor::Black))),
        make_piece(&king, board_white, Some(piece_black)),
    );

    pieces_ascii
}

lazy_static! {
    // pub static ref PIECES_ASCII: HashMap<Tile, [ColoredString; 5]> = {
        pub static ref PIECES_ASCII: HashMap<Tile, [String; 5]> = {
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
    tile_colour: CustomColor,
    piece_colour: Option<CustomColor>,
) -> [String; 5] {
    let mut piece: [String; 5] = Default::default();

    for (i, line) in widths.iter().enumerate() {
        let mut section = String::new();
        for (j, string) in line.iter().enumerate() {
            section.push_str(
                &(if j & 1 == 0 {
                    format!("{}", string.on_custom_color(tile_colour))
                } else {
                    format!("{}", string.on_custom_color(piece_colour.unwrap()))
                }),
            );
        }
        piece[i].push_str(&section);
    }

    piece
}
