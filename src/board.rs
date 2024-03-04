use {
    ascii::{Tile, PIECES_ASCII},
    colored::Colorize,
    std::{error::Error, fmt::Display, mem::swap},
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
        // swap(self.get(first), self.get(second));
        // self.get(first)

        // self.0[first.num as usize][first.letter as usize].swap
        // self.0.swap()

        if first.num == second.num {
            self.0[first.num as usize].swap(first.letter as usize, second.letter as usize);
        } else {
            // let (first, second) = if first.num > second.num {
            //     // let arr = self.0.split_at_mut(second.num as usize);
            // } else {
            //     // self.0.split_at_mut(first.num as usize)
            // };

            let (first, second) = if first.num > second.num {
                let arr = self.0.split_at_mut(first.num as usize);
                (&mut arr.1[0][first.letter as usize], &mut arr.0[second.num as usize][second.letter as usize])
            } else {
                let arr = self.0.split_at_mut(second.num as usize);
                (&mut arr.0[first.num as usize][first.letter as usize], &mut arr.1[0][second.letter as usize])
            };

            swap(first, second);

            // let (first_arr, second_arr) = 
            //     self.0.split_at_mut(second.num as usize);


        }

        // swap(first_arr[first.num])
        // swap(x, y)
    }

    pub fn get_piece(&self, position: &Position) -> &Option<Piece> {
        &self.0[position.num as usize][position.letter as usize]
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

fn alpha_to_num(alpha: char) -> u8 {

}

pub struct Position {
    letter: char,
    num: u8,
    // piece: &'a Option<Piece>, //TODO: remove
}

impl Position {
    pub fn letter(&self) -> u8 {
        (self.letter as u8) - ('a' as u8)
    }
}

impl<'a> Position<> {
    pub fn from_piece(board: &Board, piece: &Piece) -> Result<Self, BoardError> {
        // Err("Could not unambigously identify piece.\n use `move [position] [position]` format.")
        Err(BoardError::AmbigousMatch) // TODO
    }

    pub fn from_str(board: &'a Board, str: &str) -> Result<Self, Box<dyn Error>> {
        if str.len() == 2 {
            let str = str.split_at(1);
            let (letter, num) = (str.0.to_ascii_lowercase().parse::<char>()?, str.1.parse()?);

            if letter.is_ascii_alphabetic() && (letter < 'h') && (1..=8).contains(&num) {
                return Ok(Self {
                    letter,
                    num,
                    // piece: &board.0[letter as usize][num as usize], // TODO: use board.get()
                });
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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
