use {
    super::Game,
    ascii::Tile,
    ratatui::{
        style::{Color, Stylize},
        widgets::Paragraph,
    },
    std::error::Error,
    termchess_common::TResult,
    termchess_engine::board::{self, Board, Color as ChessColor, Piece},
};

pub mod ascii;

#[derive(Default)]
pub enum Size {
    Letters {
        different_symbols: bool,
    },
    UnicodeSymbols {
        different_symbols: bool,
    },
    #[default]
    UnicodeArt,
    BlockArt,
    TbdLarge,
}

// hold Option<Frame>
#[derive(Default)]
pub enum Frame {
    Ascii,
    #[default]
    Unicode,
}

impl Frame {
    fn get_char(&self, ch: FrameChar) -> char {
        match *self {
            Self::Ascii => match ch {
                FrameChar::TopLeft => '-',
                FrameChar::TopRight => '-',
                FrameChar::BotLeft => '-',
                FrameChar::BotRight => '-',
                FrameChar::OuterVertical => '|',
                FrameChar::OuterHoriz => '-',
                FrameChar::TopIntersection => '-',
                FrameChar::BotIntersection => '-',
                FrameChar::LeftIntersection => '|',
                FrameChar::RightIntersection => '|',
                FrameChar::InnerVertical => '|',
                FrameChar::InnerHoriz => '-',
                FrameChar::InnerIntersection => '|',
            },
            Self::Unicode => match ch {
                FrameChar::TopLeft => '╔',
                FrameChar::TopRight => '╗',
                FrameChar::BotLeft => '╚',
                FrameChar::BotRight => '╝',
                FrameChar::OuterVertical => '║',
                FrameChar::OuterHoriz => '═',
                FrameChar::TopIntersection => '╤',
                FrameChar::BotIntersection => '╧',
                FrameChar::LeftIntersection => '╟',
                FrameChar::RightIntersection => '╢',
                FrameChar::InnerVertical => '│',
                FrameChar::InnerHoriz => '─',
                FrameChar::InnerIntersection => '┼',
            },
        }
    }
}

enum FrameChar {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
    OuterVertical,
    OuterHoriz,
    TopIntersection,
    BotIntersection,
    LeftIntersection,
    RightIntersection,
    InnerVertical,
    InnerHoriz,
    InnerIntersection,
}

pub struct Axis {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Axis {
    fn new_all() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }

    fn new_none() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }

    fn new_left_bottom() -> Self {
        Self {
            top: false,
            bottom: true,
            left: true,
            right: false,
        }
    }
}

impl Default for Axis {
    fn default() -> Self {
        Self::new_all()
    }
}

pub struct Theme {
    board_white: Color,
    board_black: Color,
    piece_white: Color,
    piece_black: Color,
    frame_fg: Color,
    frame_bg: Color,
    axis_fg: Color,
    axis_bg: Color,
}

impl Theme {
    fn get_board(&self, tile: Tile) -> Color {
        if tile.board_color == ChessColor::White {
            self.board_white
        } else {
            self.board_black
        }
    }

    fn get_piece(&self, tile: Tile) -> Option<Color> {
        // if tile.piece.color == ChessColor::White {
        //     self.piece_white
        // } else {
        //     self.piece_black
        // }.
        tile.piece.map(|v| {
            if v.color == ChessColor::White {
                self.board_white
            } else {
                self.board_black
            }
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Default)]
struct BoardOptions {
    size: Size,
    frame: Option<Frame>,
    axis: Axis,
    theme: Theme,
}

fn board_widget(game: Game, options: BoardOptions) -> Paragraph<'static> {
    // use unicode characters tempoarily
    todo!()
}

// impl Board {
//     pub fn print_to_string(&self, rotation: &PieceColor) -> Result<String, Box<dyn Error>> {
//         let mut str = String::new();

//         let border_line_blank = ascii::BORDER_LINE_BLANK.bg(ascii::BORDER_BACKGROUND);
//         let border_line_letters = (if *rotation == PieceColor::White {
//             ascii::BORDER_LINE_LETTERS
//         } else {
//             ascii::BORDER_LINE_LETTERS_REVERSED
//         })
//         .fg(ascii::BORDER_TEXT)
//         .bg(ascii::BORDER_BACKGROUND);

//         str.push_str(&format!(
//             "{}\n{}\n{}\n",
//             border_line_blank, border_line_letters, border_line_blank
//         ));

//         // casting to a trait object is required because both possible values
//         // must have the same type
//         let iter: Box<dyn Iterator<Item = (usize, &[Option<(Piece, bool)>; 8])>>;
//         if let PieceColor::Black = *rotation {
//             iter = Box::new(self.pieces.iter().rev().enumerate()) as Box<dyn Iterator<Item = _>>;
//         } else {
//             iter = Box::new(self.pieces.iter().enumerate()) as Box<dyn Iterator<Item = _>>;
//         }
//         for (i, val) in iter {
//             for j in 0..5 {
//                 let number = if j == 2 {
//                     if *rotation == PieceColor::Black {
//                         (i + 1).to_string()
//                     } else {
//                         ((i as isize) - 8).abs().to_string()
//                     }
//                 } else {
//                     " ".to_string()
//                 };
//                 str.push_str(&format!(
//                     "{}",
//                     format!("{}{}{}", "  ", number, "  ").bg(ascii::BORDER_BACKGROUND)
//                 ));
//                 // for k in 0..8 {
//                 for k in if PieceColor::Black == *rotation {
//                     Box::new((0..8usize).rev()) as Box<dyn Iterator<Item = _>>
//                 } else {
//                     Box::new(0..8usize) as Box<dyn Iterator<Item = _>>
//                 } {
//                     // let extract_piece = |tuple: (Piece, bool)| -> Piece {tuple.0};
//                     // .map(|tuple: (Piece, bool)| -> Piece {tuple.0} )
//                     str.push_str(&format!(
//                         "{}",
//                         PIECES_ASCII
//                             .get(
//                                 // these literals don't coerce to usize for some reason
//                                 &(if (((((i % 2) == 0) && ((k % 2usize) == 0usize))
//                                     || (((i % 2) != 0) && ((k % 2usize) != 0usize)))
//                                     && *rotation == PieceColor::White)
//                                     || (((((i % 2) == 0) && ((k % 2usize) != 0usize))
//                                         || (((i % 2) != 0) && ((k % 2usize) == 0usize)))
//                                         && *rotation == PieceColor::Black)
//                                 {
//                                     Tile::White(board::extract_piece(val[k]))
//                                 } else {
//                                     Tile::Black(board::extract_piece(val[k]))
//                                 })
//                             )
//                             .ok_or("Failed to retrieve data (001)")?[j]
//                     ));
//                 }
//                 // the double format is necessary to force the StyledContent
//                 // into a string
//                 str.push_str(&format!(
//                     "{}",
//                     format!("{}{}{}\n", "  ", number, "  ")
//                         .fg(ascii::BORDER_TEXT)
//                         .bg(ascii::BORDER_BACKGROUND)
//                 ));
//             }
//         }
//         str.push_str(&format!(
//             "{}\n{}\n{}\n",
//             border_line_blank, border_line_letters, border_line_blank
//         ));

//         Ok(str)
//     }
// }
