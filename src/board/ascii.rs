use {
    super::{Piece, Tile},
    colored::{ColoredString, Colorize, CustomColor},
    lazy_static::lazy_static,
    std::collections::HashMap,
};

type AsciiPiece = [ColoredString; 5];

lazy_static! {
    pub static ref PIECES_ASCII: HashMap<Tile, AsciiPiece> = {
        let mut pieces_ascii = HashMap::new();

        let board_black = CustomColor::new(0, 0, 0);
        let board_white = CustomColor::new(255, 255, 255);
        let piece_black = CustomColor::new(0, 0, 0);
        let piece_black = CustomColor::new(0, 0, 0);

        pieces_ascii.insert(
            Tile::Black(None),
            [
                "         ".on_custom_color(board_black),
                "         ".on_custom_color(board_black),
                "         ".on_custom_color(board_black),
                "         ".on_custom_color(board_black),
                "         ".on_custom_color(board_black),
            ],
        );
        pieces_ascii.insert(
            Tile::White(None),
            [
                "         ".on_custom_color(board_white),
                "         ".on_custom_color(board_white),
                "         ".on_custom_color(board_white),
                "         ".on_custom_color(board_white),
                "         ".on_custom_color(board_white),
            ],
        );

        // pieces_ascii.insert(
        //     Tile::Black(Some(Piece::Pawn)),
        //     [
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //     ],
        // );
        // pieces_ascii.insert(
        //     Tile::White(Some(Piece::Pawn)),
        //     [
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //     ],
        // );

        // pieces_ascii.insert(
        //     Tile::Black(Some(Piece::Knight)),
        //     [
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //     ],
        // );
        // pieces_ascii.insert(
        //     Tile::White(Some(Piece::Knight)),
        //     [
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //     ],
        // );

        // pieces_ascii.insert(
        //     Tile::Black(Some(Piece::Bishop)),
        //     [
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //     ],
        // );
        // pieces_ascii.insert(
        //     Tile::White(Some(Piece::Bishop)),
        //     [
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //     ],
        // );

        // pieces_ascii.insert(
        //     Tile::Black(Some(Piece::Rook)),
        //     [
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //     ],
        // );
        // pieces_ascii.insert(
        //     Tile::White(Some(Piece::Rook)),
        //     [
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //     ],
        // );

        // pieces_ascii.insert(
        //     Tile::Black(Some(Piece::Queen)),
        //     [
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //     ],
        // );
        // pieces_ascii.insert(
        //     Tile::White(Some(Piece::Queen)),
        //     [
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //     ],
        // );

        // pieces_ascii.insert(
        //     Tile::Black(Some(Piece::King)),
        //     [
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //         "         ".on_custom_color(board_black),
        //     ],
        // );
        // pieces_ascii.insert(
        //     Tile::White(Some(Piece::King)),
        //     [
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //         "         ".on_custom_color(board_white),
        //     ],
        // );

        pieces_ascii
    };
}
