use termchess_common::TResult;

use {
    ascii::{Tile, PIECES_ASCII},
    colored::Colorize,
    std::{
        error::Error,
        fmt::Display,
        iter::Flatten,
        mem::swap,
        slice::{Iter, IterMut},
    },
};

mod ascii;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Board {
    pieces: [[Option<(Piece, bool)>; 8]; 8],
    moves: Vec<Move>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [
                [
                    Some((Piece::Rook(PieceColor::Black), false)),
                    Some((Piece::Knight(PieceColor::Black), false)),
                    Some((Piece::Bishop(PieceColor::Black), false)),
                    Some((Piece::Queen(PieceColor::Black), false)),
                    Some((Piece::King(PieceColor::Black), false)),
                    Some((Piece::Bishop(PieceColor::Black), false)),
                    Some((Piece::Knight(PieceColor::Black), false)),
                    Some((Piece::Rook(PieceColor::Black), false)),
                ],
                [
                    Some((Piece::Pawn(PieceColor::Black), false)),
                    Some((Piece::Pawn(PieceColor::Black), false)),
                    Some((Piece::Pawn(PieceColor::Black), false)),
                    Some((Piece::Pawn(PieceColor::Black), false)),
                    Some((Piece::Pawn(PieceColor::Black), false)),
                    Some((Piece::Pawn(PieceColor::Black), false)),
                    Some((Piece::Pawn(PieceColor::Black), false)),
                    Some((Piece::Pawn(PieceColor::Black), false)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, Some((Piece::Pawn(PieceColor::White), false)), None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                // [None, None, None, None, None, None, None, None],
                // [None, None, None, None, None, None, None, None],
                [
                    Some((Piece::Pawn(PieceColor::White), false)),
                    Some((Piece::Pawn(PieceColor::White), false)),
                    Some((Piece::Pawn(PieceColor::White), false)),
                    Some((Piece::Pawn(PieceColor::White), false)),
                    Some((Piece::Pawn(PieceColor::White), false)),
                    Some((Piece::Pawn(PieceColor::White), false)),
                    Some((Piece::Pawn(PieceColor::White), false)),
                    Some((Piece::Pawn(PieceColor::White), false)),
                ],
                [
                    Some((Piece::Rook(PieceColor::White), false)),
                    Some((Piece::Knight(PieceColor::White), false)),
                    Some((Piece::Bishop(PieceColor::White), false)),
                    Some((Piece::Queen(PieceColor::White), false)),
                    Some((Piece::King(PieceColor::White), false)),
                    Some((Piece::Bishop(PieceColor::White), false)),
                    Some((Piece::Knight(PieceColor::White), false)),
                    Some((Piece::Rook(PieceColor::White), false)),
                ],
                // [
                //     Some((Piece::Rook(PieceColor::White), false)),
                //     None,
                //     None,
                //     None,
                //     Some((Piece::King(PieceColor::White), false)),
                //     None,
                //     None,
                //     Some((Piece::Rook(PieceColor::White), false)),
                // ],
                // [None, None, None, None, None, None, None, None],
                // [None, None, None, None, None, None, None, None],
                // [None, None, None, None, None, None, None, None],
                // [None, None, None, None, None, None, None, None],
            ],
            moves: Vec::new(),
        }
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
        let iter: Box<dyn Iterator<Item = (usize, &[Option<(Piece, bool)>; 8])>>;
        if let PieceColor::Black = *rotation {
            iter = Box::new(self.pieces.iter().rev().enumerate()) as Box<dyn Iterator<Item = _>>;
        } else {
            iter = Box::new(self.pieces.iter().enumerate()) as Box<dyn Iterator<Item = _>>;
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
                    // let extract_piece = |tuple: (Piece, bool)| -> Piece {tuple.0};
                    // .map(|tuple: (Piece, bool)| -> Piece {tuple.0} )
                    print!(
                        "{}",
                        PIECES_ASCII
                            .get(
                                // these literals don't coerce to usize for some reason
                                &(if (((((i % 2) == 0) && ((k % 2usize) == 0usize))
                                    || (((i % 2) != 0) && ((k % 2usize) != 0usize)))
                                    && *rotation == PieceColor::White)
                                    || (((((i % 2) == 0) && ((k % 2usize) != 0usize))
                                        || (((i % 2) != 0) && ((k % 2usize) == 0usize)))
                                        && *rotation == PieceColor::Black)
                                {
                                    Tile::White(extract_piece(val[k]))
                                } else {
                                    Tile::Black(extract_piece(val[k]))
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

    /// conditionally move a piece
    pub fn try_move(
        &mut self,
        piece_position: &Position,
        move_position: &Position,
    ) -> Result<Option<String>, String> {
        let (can_move, mut str) = self.can_move(piece_position, move_position);

        if let None = can_move {
            Err(format!("Move error: {}", str.unwrap()))
        } else {
            let piece_move = can_move.unwrap();
            self.moves.push(piece_move.clone());
            match piece_move.special_move {
                None => {
                    self.move_piece(piece_position, move_position);
                },
                Some(SpecialMove::EnPassant(captured_piece)) => {
                    *self.get_piece_mut(&captured_piece) = None;
                    self.move_piece(piece_position, move_position);
                },
                Some(SpecialMove::Castle(side)) => {
                    str = Some(format!("{} king at {} castled {} to {}", self.get_piece(piece_position).unwrap().0.color(), piece_position, side, move_position));

                    let king = *self.get_piece(piece_position);
                    let rook = *self.get_piece(move_position);

                    *self.get_piece_mut(piece_position) = None;
                    *self.get_piece_mut(move_position) = None;

                    *self.get_piece_mut(&Position::from_data(((piece_position.letter_index() as i8) + (if side == Side::Kingside {2} else {-2})) as u8, piece_position.num)) = king;
                    *self.get_piece_mut(&Position::from_data(((piece_position.letter_index() as i8) + (if side == Side::Kingside {1} else {-1})) as u8, piece_position.num)) = rook;
                },
                Some(SpecialMove::PawnPromotion) => {
                    // set move message
                    
                    println!("You've promoted a pawn!\nEnter any piece other a than pawn or king to choose it.");
                    
                    // TODO: extract this to a function
                    let mut input = String::new();
                    let chosen_piece;
                    loop {
                        super::get_input(&mut input);
                        if let Ok(piece) = Piece::from_str(&input.trim().to_ascii_lowercase(), self.get_piece(piece_position).unwrap().0.color()) {
                            match piece {
                                Piece::King(_) | Piece::Pawn(_) => 
                                    println!("Cannot promote to {}", piece),
                                
                                _ => {
                                    chosen_piece = piece;
                                    break;
                                }
                            }

                        } else {
                            println!("Could not parse string. please enter a valid piece name.");
                        }
                    }

                    str = Some(format!("{} Pawn at {} Promoted to a {} at {}", self.get_piece(piece_position).unwrap().0.color(), piece_position, chosen_piece, move_position));

                    if self.get_piece(move_position).is_some() {
                        str = Some(format!("{} and captured a {}", str.unwrap(), self.get_piece(move_position).unwrap().0))
                    }

                    *self.get_piece_mut(piece_position) = None;
                    *self.get_piece_mut(move_position) = Some((chosen_piece, true));
                }
            }


            if let Some(str) = str {
                Ok(Some(str))
            } else {
                Ok(None)
            }
            // TODO: PUSH MOVE ONTO MOVE STACK
            
        }

        // if let Some(err) = self.can_move(piece_position, move_position).1 {
        //     Err(format!("Move error: {}", err))
        // } else {
        //     self.move_piece(piece_position, move_position);

        //     Ok(())
        // }
    }

    fn can_move(
        &self,
        piece_position: &Position,
        move_position: &Position,
    ) -> (Option<Move>, Option<String>) {
        let piece = self.get_piece(piece_position);
        let move_location = self.get_piece(move_position);
        let mut move_message = None;
        let mut special_move: Option<SpecialMove> = None; // TODO: REMOVE TYPE ANNOTATION

        if piece.is_none() {
            return (None, Some(format!("No piece at {}", piece_position)));
        }
        if piece_position == move_position {
            return (None, Some("A piece cannot capture itself.".to_string()));
        }

        let piece = piece.expect("we've verified piece_position is at a piece");

        match piece.0 {
            Piece::Pawn(_) => {
                // if moving forward 1, check that there is no piece there
                // if moving forward 2, check that it's the first time that pawn has moved
                // if capturing, check that there's a capturable piece where it's moving
                // if en passanteing, do something or other I dunno the rules
                // DONE i think

                if piece_position.num == move_position.num {
                    return (None, Some("Pawns cannot move to the side".to_owned()));
                } else if ((piece_position.num > move_position.num) && (piece.0.color() == PieceColor::White))
                ||
                ((piece_position.num < move_position.num) && (piece.0.color() == PieceColor::Black)) {
                    return (None, Some("Pawns cannot move backwards".to_owned()));
                }
                // else if (move_position.num + 1) > piece_position.num {
                //     return (None, Some("Pawns can only move forward one or two spaces".to_owned()));
                // }

                if piece_position.letter_index() == move_position.letter_index()
                // pawn is directly ahead
                {
                    if move_position.num > (piece_position.num + 2) {
                        return (
                            None,
                            Some("Pawns can only move forward one or two spaces".to_owned()),
                        );
                    } else if ((move_position.num == (piece_position.num + 1))
                        && self
                            .get_piece(&Position::from_data(
                                piece_position.letter_index() as u8,
                                piece_position.num + 1,
                            ))
                            .is_some())
                        || ((move_position.num == (piece_position.num + 2))
                            && self
                                .get_piece(&Position::from_data(
                                    piece_position.letter_index() as u8,
                                    piece_position.num + 2,
                                ))
                                .is_some())
                    {
                        return (
                            None,
                            Some("Pawns cannot capture by moving vertically".to_owned()),
                        );
                    } else if (move_position.num == (piece_position.num + 2)) && piece.1 {
                        return (
                            None,
                            Some(
                                "Only a pawn that hasn't moved yet can move forward 2 tiles"
                                    .to_owned(),
                            ),
                        );
                    } else if (move_position.num == (piece_position.num + 2))
                        && self
                            .get_piece(&Position::from_data(
                                piece_position.letter_index() as u8,
                                piece_position.num + 1,
                            ))
                            .is_some()
                    {
                        return (
                            None,
                            Some(format!(
                                "Pawn must have an unobstructed path to {}",
                                move_position
                            )),
                        );
                    }
                } else if ((piece_position.letter_index() == (move_position.letter_index() + 1))
                    || (piece_position.letter_index() == (move_position.letter_index() - 1)))
                    && (move_position.num == (piece_position.num + 1))
                // pawn is either side
                {
                    // if invalid en passant, return
                    // if valid en passant, return
                    // if normal capture, do nothing

                    if move_location.is_none() {
                        let passant_pos = Position::from_literals(move_position.letter, if piece.0.color() == PieceColor::White {5} else {4});
                        if let Some((Piece::Pawn(_), _)) = self.get_piece(&passant_pos) {
                            let piece_move = self.moves.last().expect("can't reach this on the first turn");
                            if (piece_move.end_position != passant_pos) ||
                                    piece_move.moved_piece.1 {
                                        return (None, Some("Can only capture a pawn by en passant that that just moved".to_owned()));
                            } else {
                                special_move = Some(SpecialMove::EnPassant(passant_pos));
                                move_message = Some(format!("{} Pawn at {} captured a Pawn at {} via en passant and is now at {}", piece.0.color(), piece_position, passant_pos, move_position));
                                // return (Some(SpecialMove::EnPassant(passant_pos)), Some(format!("{} Pawn at {} captured a Pawn at {} via en passant and is now at {}", piece.0.color(), piece_position, passant_pos, move_position)));
                            }
                        } else {
                            return (None, Some("Pawns can only move diagonally with a capture or en passant.\n\
                                See en passant rules with `help en-passant`".to_owned()));
                        }
                    }

                    // todo!();
                } else {
                    return (
                        None,
                        Some(
                            "Pawns can only move straight ahead or diagonally to either side"
                                .to_owned(),
                        ),
                    );
                }

                // pawn promotion
                if (move_position.num == 8) || (move_position.num == 1) {
                    special_move = Some(SpecialMove::PawnPromotion);
                    // message gets set by try_move()
                }

                // todo!();
            }
            Piece::Knight(_) => {
                // verify that movement is offset by 2-1
                // aka it's position must be within 2 in both directions, and not be axial or diagonal
                // DONE

                if self.is_axial(piece_position, move_position)
                    || self.is_diagonal(piece_position, move_position)
                    || ((piece_position.num_index() as isize - move_position.num_index() as isize)
                        .abs()
                        > 2)
                    || ((piece_position.letter_index() as isize
                        - move_position.letter_index() as isize)
                        .abs()
                        > 2)
                {
                    return (
                        None,
                        Some(
                            "A knight may only move to one of the 8 squares closest \
                        to it and not diagonal or up/down"
                                .to_string(),
                        ),
                    );
                }
            }
            Piece::Bishop(_) => {
                // verify that movement is diagonal of it, and no pieces in the way
                // DONE

                if !self.is_diagonal(piece_position, move_position) {
                    return (None, Some("Bishop can only move diagonally".to_string()));
                }

                if !self.can_move_diagonally(piece_position, move_position) {
                    return (
                        None,
                        Some(format!(
                            "Bishop must have an unobstructed path to {}",
                            move_position
                        )),
                    );
                }
            }
            Piece::Rook(_) => {
                // verify movement is on same row or same column,
                // and there are no pieces in between
                // if castling, say must castle with king
                // DONE

                if !self.is_axial(piece_position, move_position) {
                    return (
                        None,
                        Some("Rook can only move vertically/horizontally".to_string()),
                    );
                }

                // if let guards/chaining currently unstable unfortunately
                match move_location {
                    Some((Piece::King(color), _)) if *color == piece.0.color() => {
                        return (
                            None,
                            Some(
                                "Caslting is a king move, and must be done with the king"
                                    .to_string(),
                            ),
                        )
                    }
                    _ => (),
                }

                if !self.can_move_axially(piece_position, move_position) {
                    return (
                        None,
                        Some(format!(
                            "Rook must have an unobstructed path to {}",
                            move_position
                        )),
                    );
                }
            }
            Piece::Queen(_) => {
                // verify that movement is either diagonal or vertical/horizontal,
                // and no pieces in way
                // DONE

                if !self.is_diagonal(piece_position, move_position)
                    && !self.is_axial(piece_position, move_position)
                {
                    return (
                        None,
                        Some(
                            "Queen's can only move diagonally or vertically/horizontally"
                                .to_string(),
                        ),
                    );
                }

                if !self.can_move_diagonally(piece_position, move_position)
                    || !self.can_move_axially(piece_position, move_position)
                {
                    return (
                        None,
                        Some(format!(
                            "Queen must have an unobstructed path to {}",
                            move_position
                        )),
                    );
                }
            }
            Piece::King(_) => {
                // verify that movement is either diagonal or horizontal,
                // and not more than one piece
                // if caslting swap and don't move somehow

                // detect if castling
                if Some(Piece::Rook(piece.0.color())) == move_location.map(|t| t.0) {
                    // attempting to castle

                    if piece.1 || move_location.unwrap().1 {
                        return (None, Some("Can only castle with unmoved king and rook".to_owned()));
                    } else if !self.can_move_axially(piece_position, move_position) {
                        return (None, Some("Can't castle with any pieces between king and rook".to_owned()));
                    } else {
                        // valid castle
                        let side = if move_position.letter_index() > piece_position.letter_index() {
                            Side::Kingside
                        } else {
                            Side::Queenside
                        };

                        special_move = Some(SpecialMove::Castle(side));
                        // move message gets set by try_move
                        // move_message = Some(format!("{} King at {} castled {}", piece.0.color(), piece_position, side));
                    }
                    
                    // todo!();
                } else if 
                ((piece_position.num as i8 - move_position.num as i8).abs() > 1)
                ||
                ((piece_position.letter_index() as i8 - move_position.letter_index() as i8).abs() > 1) {
                    // invalid king movement
                    return (None, Some("King can only move one space at a time".to_owned()));
                }


                // todo!();
            }
        }

        if (move_location.is_some() && (piece.0.color() == move_location.unwrap().0.color())) && special_move.is_none() {
            return (
                None,
                Some("A piece cannot capture a piece of its own color".to_string()),
            );
        }

        // don't overwrite more specific messages
        if move_location.is_some() && move_message.is_none() {
            move_message = Some(format!(
                "{} {} at {} Captured {} at {}",
                piece.0.color(),
                piece.0,
                piece_position,
                move_location.unwrap().0,
                move_position
            ));
        }

        let piece_move = Move {
            moved_piece: piece,
            start_position: *piece_position,
            end_position: *move_position,
            captured_piece: move_location.map(|tuple| tuple.0), // why does *this* work???
            special_move,
        };

        (Some(piece_move), move_message)
    }

    fn is_diagonal(&self, piece_position: &Position, move_position: &Position) -> bool {
        let vertical_difference =
            (piece_position.num_index() as isize - move_position.num_index() as isize).abs();
        let horizontal_difference =
            (piece_position.letter_index() as isize - move_position.letter_index() as isize).abs();

        (vertical_difference - horizontal_difference) == 0
    }

    fn is_axial(&self, piece_position: &Position, move_position: &Position) -> bool {
        (piece_position.num_index() == move_position.num_index())
            || (piece_position.letter_index() == move_position.letter_index())
    }

    fn can_move_diagonally(&self, piece_position: &Position, move_position: &Position) -> bool {
        let (piece_num_ind, piece_let_ind) =
            (piece_position.num_index(), piece_position.letter_index());
        let (move_num_ind, move_let_ind) =
            (move_position.num_index(), move_position.letter_index());

        for position in self.positions().iter() {
            let (p_num_ind, p_let_ind) = (position.num_index(), position.letter_index());

            if (((p_num_ind > piece_num_ind) && (p_num_ind < move_num_ind))
                || ((p_num_ind < piece_num_ind) && (p_num_ind > move_num_ind)))
                && (((p_let_ind > piece_let_ind) && (p_let_ind < move_let_ind))
                    || ((p_let_ind < piece_let_ind) && (p_let_ind > move_let_ind)))

            && self.is_diagonal(piece_position, position)
            && self.is_diagonal(move_position, position)
            // ^^^ checks that position is diagonally between the two

            && self.has_piece(position)
            // since it has a piece, must be invalid
            {
                return false;
            }
        }

        true
    }

    pub fn can_move_axially(&self, piece_position: &Position, move_position: &Position) -> bool {
        let (piece_num_ind, piece_let_ind) =
            (piece_position.num_index(), piece_position.letter_index());
        let (move_num_ind, move_let_ind) =
            (move_position.num_index(), move_position.letter_index());

        for position in self.positions().iter() {
            let (p_num_ind, p_let_ind) = (position.num_index(), position.letter_index());

            if (((p_num_ind == piece_num_ind)
                && (p_num_ind == move_num_ind)
                && (((p_let_ind > piece_let_ind) && (p_let_ind < move_let_ind))
                    || ((p_let_ind < piece_let_ind) && (p_let_ind > move_let_ind))))
                || 
                ((p_let_ind == piece_let_ind) && (p_let_ind == move_let_ind) && 
        (((p_num_ind > piece_num_ind) && (p_num_ind < move_num_ind))
        || ((p_num_ind < piece_num_ind) && (p_num_ind > move_num_ind)))))

        && self.is_axial(piece_position, position)
        && self.is_axial(move_position, position)
        // ^^^ checks that piece is axially between the two

        && self.has_piece(position)
            // since it has a piece, must be invalid
            {
                return false;
            }
        }

        true
    }

    /// unconditionally move a piece
    fn move_piece(&mut self, piece_position: &Position, move_position: &Position) {
        if let Some((Piece::King(_), _)) = self.get_piece(move_position) {
            panic!("Check invariant was not upheld, attempted to capture King.");
        }
        
        *self.get_piece_mut(move_position) =
            Some((self.get_piece(piece_position).unwrap().0, true));
        *self.get_piece_mut(piece_position) = None;
    } // TODO: return points

    pub fn get_piece(&self, position: &Position) -> &Option<(Piece, bool)> {
        &self.pieces[position.num_index()][position.letter_index()]
    }

    fn get_piece_mut(&mut self, position: &Position) -> &mut Option<(Piece, bool)> {
        &mut self.pieces[position.num_index()][position.letter_index()]
    }

    fn find_piece_position(&self) -> Position {
        todo!();
    }

    pub fn has_piece(&self, position: &Position) -> bool {
        // dbg!(self.get_piece(position));
        self.get_piece(position).is_some()
    }

    fn positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                // TODO: flatten
                positions.push(Position::from_indices(i, j));
            }
        }

        positions
    }
}

fn extract_piece(piece: Option<(Piece, bool)>) -> Option<Piece> {
    if let Some(tuple) = piece {
        Some(tuple.0)
    } else {
        None
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Move {
    pub moved_piece: (Piece, bool),
    pub start_position: Position,
    pub end_position: Position,
    pub captured_piece: Option<Piece>,
    pub special_move: Option<SpecialMove>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum SpecialMove {
    Castle(Side),
    EnPassant(Position),
    PawnPromotion,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum Side {
    Kingside,
    Queenside,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Side::Kingside => write!(f, "kingside"),
            Side::Queenside => write!(f, "queenside"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    letter: char,
    num: u8,
}

impl Position {
    fn letter_index(&self) -> usize {
        ((self.letter as u8) - b'a') as usize
    }

    fn num_index(&self) -> usize {
        ((self.num as i8) - 8).unsigned_abs() as usize
    }

    /// returns the number that corresponds to where the NO IDEA
    fn num_position(&self) -> u8 {
        self.num - 1
    }

    /// num must correspond to board's numbering
    pub fn from_literals(letter: char, num: u8) -> Self {
        Self {
        letter,
        num
        }
    }

    /// num must correspond to board's numbering
    pub fn from_data(letter: u8, num: u8) -> Self {
        Self {
            letter: (letter + b'a') as char,
            num,
        }
    }

    pub fn from_indices(vert_index: usize, horiz_index: usize) -> Self {
        assert!((vert_index < 8) && (horiz_index < 8));

        Self {
            num: (vert_index as i8 - 8).unsigned_abs(),
            letter: (horiz_index as u8 + b'a') as char,
        } // TODO: VERIFY THIS WORKS
    }

    pub fn from_piece(board: &Board, piece: &Piece) -> Result<Self, BoardError> {
        todo!();
        // Err("Could not unambigously identify piece.\n use `move [position] [position]` format.")
        Err(BoardError::AmbigousMatch) // TODO: implement this
    }

    pub fn from_str(str: &str) -> Result<Self, Box<dyn Error>> {
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

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Piece {
    // ordered by point amount
    Pawn(PieceColor), // TODO: add has_moved
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor), // TODO: add has_moved
    Queen(PieceColor),
    King(PieceColor), // TODO: add has_moved
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

    fn color(&self) -> PieceColor {
        match *self {
            Piece::Pawn(color) => color,
            Piece::Knight(color) => color,
            Piece::Bishop(color) => color,
            Piece::Rook(color) => color,
            Piece::Queen(color) => color,
            Piece::King(color) => color,
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
