use {ascii::PIECES_ASCII, std::error::Error};

mod ascii;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Board([[Tile; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        Self([
            [
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
            ],
            [
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
            ],
            [
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
            ],
            [
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
            ],
            [
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
            ],
            [
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
            ],
            [
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
            ],
            [
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
                Tile::White(None),
                Tile::Black(None),
            ],
        ])
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        for val in self.0 {
            for i in 0..5 {
                for j in 0..8 {
                    print!(
                        "{}",
                        PIECES_ASCII
                            .get(&val[j])
                            .ok_or("Failed to retrieve data (001)")?[i]
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
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Tile {
    Black(Option<Piece>),
    White(Option<Piece>),
}
