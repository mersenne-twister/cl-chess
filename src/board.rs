use {
    ascii::{Tile, PIECES_ASCII},
    std::error::Error,
};

mod ascii;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Board([[Option<Piece>; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        Self([
            [
                Some(Piece::Rook(PieceColor::Black)),
                Some(Piece::Knight(PieceColor::Black)),
                Some(Piece::Bishop(PieceColor::Black)),
                Some(Piece::Queen(PieceColor::Black)),
                Some(Piece::King(PieceColor::Black)),
                Some(Piece::Bishop(PieceColor::Black)),
                Some(Piece::Knight(PieceColor::Black)),
                Some(Piece::Rook(PieceColor::Black)),
            ],
            [
                Some(Piece::Pawn(PieceColor::Black)),
                Some(Piece::Pawn(PieceColor::Black)),
                Some(Piece::Pawn(PieceColor::Black)),
                Some(Piece::Pawn(PieceColor::Black)),
                Some(Piece::Pawn(PieceColor::Black)),
                Some(Piece::Pawn(PieceColor::Black)),
                Some(Piece::Pawn(PieceColor::Black)),
                Some(Piece::Pawn(PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::Pawn(PieceColor::White)),
                Some(Piece::Pawn(PieceColor::White)),
                Some(Piece::Pawn(PieceColor::White)),
                Some(Piece::Pawn(PieceColor::White)),
                Some(Piece::Pawn(PieceColor::White)),
                Some(Piece::Pawn(PieceColor::White)),
                Some(Piece::Pawn(PieceColor::White)),
                Some(Piece::Pawn(PieceColor::White)),
            ],
            [
                Some(Piece::Rook(PieceColor::White)),
                Some(Piece::Knight(PieceColor::White)),
                Some(Piece::Bishop(PieceColor::White)),
                Some(Piece::Queen(PieceColor::White)),
                Some(Piece::King(PieceColor::White)),
                Some(Piece::Bishop(PieceColor::White)),
                Some(Piece::Knight(PieceColor::White)),
                Some(Piece::Rook(PieceColor::White)),
            ],
            // [None, None, None, None, None, None, None, None],
            // [None, None, None, None, None, None, None, None],
            // [None, None, None, None, None, None, None, None],
            // [None, None, None, None, None, None, None, None],
        ])
    }

    pub fn print(&self, rotation: &PieceColor) -> Result<(), Box<dyn Error>> {
        // casting to a trait object is required because both possible values
        // must have the same type
        let iter: Box<dyn Iterator<Item = (usize, &[Option<Piece>; 8])>>;
        if let PieceColor::Black = *rotation {
            iter = Box::new(self.0.iter().rev().enumerate()) as Box<dyn Iterator<Item = _>>;
        } else {
            iter = Box::new(self.0.iter().enumerate()) as Box<dyn Iterator<Item = _>>;
        }

        for (i, val) in iter {
            for j in 0..5 {
                for k in 0..8 {
                    print!(
                        "{}",
                        PIECES_ASCII
                            .get(
                                &(if (((i % 2) == 0) && ((k % 2) == 0))
                                    || (((i % 2) != 0) && ((k % 2) != 0))
                                {
                                    Tile::White(val[k])
                                } else {
                                    Tile::Black(val[k])
                                })
                            )
                            .ok_or("Failed to retrieve data (001)")?[j]
                    );
                }
                println!();
            }
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Piece {
    // ordered by point amount
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum PieceColor {
    Black,
    White,
}
