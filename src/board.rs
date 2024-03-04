use {
    ascii::{Tile, PIECES_ASCII},
    colored::Colorize,
    std::{error::Error, fmt::Display, mem::swap},
};

mod ascii;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
        let border_line_blank =
            ascii::BORDER_LINE_BLANK.on_custom_color(ascii::BORDER_BACKGROUND.into());
        let border_line_letters = (if *rotation == PieceColor::White {
            ascii::BORDER_LINE_LETTERS
        } else {
            ascii::BORDER_LINE_LETTERS_REVERSED
        })
        .custom_color(ascii::BORDER_TEXT.into())
        .on_custom_color(ascii::BORDER_BACKGROUND.into());

        println!(
            "{}\n{}\n{}",
            border_line_blank, border_line_letters, border_line_blank
        );

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
                let number = if j == 2 {
                    if *rotation == PieceColor::Black {
                        (i + 1).to_string()
                    } else {
                        ((i as isize) - 8).abs().to_string()
                    }
                } else {
                    " ".to_string()
                };
                print!(
                    "{}",
                    format!("{}{}{}", "  ", number, "  ")
                        .on_custom_color(ascii::BORDER_BACKGROUND.into())
                );
                // for k in 0..8 {
                for k in if PieceColor::Black == *rotation {
                    Box::new((0..8usize).rev()) as Box<dyn Iterator<Item = _>>
                } else {
                    Box::new(0..8usize) as Box<dyn Iterator<Item = _>>
                } {
                    print!(
                        "{}",
                        PIECES_ASCII
                            .get(
                                // these literals don't coerce to usize for some reason
                                &(if (((i % 2) == 0) && ((k % 2usize) == 0usize))
                                    || (((i % 2) != 0) && ((k % 2usize) != 0usize))
                                {
                                    Tile::White(val[k])
                                } else {
                                    Tile::Black(val[k])
                                })
                            )
                            .ok_or("Failed to retrieve data (001)")?[j]
                    );
                }
                println!(
                    "{}",
                    format!("{}{}{}", "  ", number, "  ")
                        .custom_color(ascii::BORDER_TEXT.into())
                        .on_custom_color(ascii::BORDER_BACKGROUND.into())
                );
            }
        }
        println!(
            "{}\n{}\n{}",
            border_line_blank, border_line_letters, border_line_blank
        );

        Ok(())
    }

    pub fn swap(&mut self, first: &Position, second: &Position) {
        if first.num_index() == second.num_index() {
            self.0[first.num_index()].swap(first.letter_index(), second.letter_index());
        } else {
            let (first, second) = if first.num_index() > second.num_index() {
                let arr = self.0.split_at_mut(first.num_index());
                (
                    &mut arr.1[0][first.letter_index()],
                    &mut arr.0[second.num_index()][second.letter_index()],
                )
            } else {
                let arr = self.0.split_at_mut(second.num_index());
                (
                    &mut arr.0[first.num_index()][first.letter_index()],
                    &mut arr.1[0][second.letter_index()],
                )
            };

            swap(first, second);
        }
    }

    pub fn get_piece(&self, position: &Position) -> &Option<Piece> {
        &self.0[position.num_index()][position.letter_index()]
    }

    pub fn has_piece(&self, position: &Position) -> bool {
        self.get_piece(position).is_some()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    letter: char,
    num: u8,
}

impl Position {
    pub fn letter_index(&self) -> usize {
        ((self.letter as u8) - b'a') as usize
    }

    pub fn num_index(&self) -> usize {
        ((self.num as i8) - 8).unsigned_abs() as usize
    }
}

impl<'a> Position {
    pub fn from_piece(board: &Board, piece: &Piece) -> Result<Self, BoardError> {
        // Err("Could not unambigously identify piece.\n use `move [position] [position]` format.")
        Err(BoardError::AmbigousMatch) // TODO
    }

    pub fn from_str(board: &'a Board, str: &str) -> Result<Self, Box<dyn Error>> {
        if str.len() == 2 {
            let str = str.split_at(1);
            let (letter, num) = (str.0.to_ascii_lowercase().parse::<char>()?, str.1.parse()?);

            // must be lowercase, must be alphabetic, therefore must be >= to 'a'
            if letter.is_ascii_alphabetic() && (letter <= 'h') && (1..=8).contains(&num) {
                return Ok(Self { letter, num });
            }
        }
        Err(Box::new(BoardError::ParseError))
    }

    // pub fn has_piece(&self) -> bool {
    //     self.piece.is_some()
    // }

    // pub fn swap(&mut self, position: &mut Position) {
    //     swap(self.piece, position.piece);
    // }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.num)
    }
}

#[derive(Debug)]
pub enum BoardError {
    ParseError,
    AmbigousMatch,
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BoardError::ParseError => write!(f, "Parse error"),
            BoardError::AmbigousMatch => write!(f, "Ambigous match"),
        }
    }
}

impl Error for BoardError {
    fn cause(&self) -> Option<&dyn Error> {
        todo!();
    }

    fn description(&self) -> &str {
        todo!();
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!();
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub enum Piece {
    // ordered by point amount
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl Piece {
    fn from_str(str: &str, color: PieceColor) -> Result<Self, BoardError> {
        match str.to_ascii_lowercase().as_str() {
            "pawn" => Ok(Piece::Pawn(color)),
            "knight" => Ok(Piece::Knight(color)),
            "bishop" => Ok(Piece::Bishop(color)),
            "rook" => Ok(Piece::Rook(color)),
            "queen" => Ok(Piece::Queen(color)),
            "king" => Ok(Piece::King(color)),
            _ => Err(BoardError::ParseError),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Piece::Pawn(_) => write!(f, "pawn"),
            Piece::Knight(_) => write!(f, "knight"),
            Piece::Bishop(_) => write!(f, "bishop"),
            Piece::Rook(_) => write!(f, "rook"),
            Piece::Queen(_) => write!(f, "queen"),
            Piece::King(_) => write!(f, "king"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum PieceColor {
    Black,
    White,
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PieceColor::White => write!(f, "White"),
            PieceColor::Black => write!(f, "Black"),
        }
    }
}
