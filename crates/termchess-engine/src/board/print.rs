/// All board *rendering* logic goes in here, all logic concerned with ascii
/// or colouring goes in `ascii`
// rustfmt wants to remove this line so just ignore this
use {
    crate::board::{
        self,
        ascii::{self, FrameChar, Side, Tile},
        Board, Color as ChessColor, Piece,
    },
    ratatui::{
        prelude::*,
        style::{Color, Stylize},
        text::{Line, Text},
        widgets::Paragraph,
    },
    std::error::Error,
    termchess_common::TResult,
};

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Size {
    /// bool makes black lowercase
    Letters {
        different_symbols: bool,
    },
    /// bool makes black chars hollow
    UnicodeSymbols {
        different_symbols: bool,
    },
    #[default]
    UnicodeArt,
    BlockArt,
    TbdLarge,
}

// hold Option<Frame>
#[derive(Default, Clone, Copy)]
pub enum Frame {
    Single,
    #[default]
    Double,
    Quadruple,
}

#[derive(Clone, Copy, Default)]
pub enum Thickness {
    Thin,
    #[default]
    Thick,
}

#[derive(Clone, Copy)]
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

    // are the other ones necessary?
    fn new(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}

impl Default for Axis {
    fn default() -> Self {
        Self::new_left_bottom()
    }
}

#[derive(Clone, Copy)]
pub enum ThemeName {
    // what themes do I actually want
    WhiteBlue,
    WoodBrown,
    GruvBox,
    Dracula,
    Solarized,
    Doom, // black and red, see if I can force it to be super black
    Trans,
    Enby,
    Bi,
    Ace,
}

#[derive(Clone)]
pub struct Theme {
    pub board_white: Color,
    pub board_black: Color,
    pub piece_white: Color,
    pub piece_black: Color,
    pub border_fg: Color,
    pub border_bg: Color,
    pub axis_fg: Color,
    pub axis_bg: Color,
}

impl Theme {
    pub fn new(name: ThemeName) -> Self {
        match name {
            _ => todo!(),
        }
    }

    pub fn get_board(&self, tile: Tile) -> Color {
        if tile.board_color == ChessColor::White {
            self.board_white
        } else {
            self.board_black
        }
    }

    pub fn get_piece(&self, tile: Tile) -> Option<Color> {
        tile.piece.map(|v| {
            if v.color == ChessColor::White {
                self.piece_white
            } else {
                self.piece_black
            }
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            board_white: Color::Indexed(216),
            board_black: Color::Indexed(208),
            piece_white: Color::White,
            piece_black: Color::Black,
            border_fg: Color::Indexed(215),
            border_bg: Color::Indexed(52),
            axis_fg: Color::default(),
            axis_bg: Color::default(),
        }
    }
}

#[derive(Default, Clone)]
pub struct BoardOptions {
    pub size: Size,
    // do I want to make this changeable, or should it be constant?
    // pub frame: Option<Frame>,
    /// a physical separation between squares so one can play without colour,
    /// and other compatibility
    pub compat: bool,
    pub axis: Axis,
    pub theme: Theme,
}

impl BoardOptions {
    /// return width|height of board
    pub fn width(&self) -> usize {
        // get width of one of the strings
        let tile_width = self.size.get_chars(None)[0].len();

        // TODO: account for axis

        // TODO: remove magic number
        tile_width * 8 + 6
    }

    pub fn height(&self) -> usize {
        // get number of strings
        let tile_height = self.size.get_chars(None).len();

        tile_height * 8 + 2
    }

    ///  return the lines in one tile
    pub fn tile_lines(&self) -> usize {
        match self.size {
            Size::Letters { .. } => 1,
            Size::UnicodeSymbols { .. } => 1,
            Size::UnicodeArt => 4,
            Size::BlockArt => 5,
            Size::TbdLarge => todo!(),
        }
    }

    /// return cols in one tile
    pub fn tile_cols(&self) -> usize {
        todo!()
    }
}

//

// highlight: Option<Position>
impl Board {
    pub fn print<'a>(&self, options: &'a BoardOptions, rotation: ChessColor) -> Text<'a> {
        let mut text = Vec::new();

        text.push(Line::from(options.get_frame(Side::Top)));

        // casting to a trait object is required because both possible values
        // must have the same type
        let iter: Box<dyn Iterator<Item = (usize, &[Option<(Piece, bool)>; 8])>>;
        if let ChessColor::Black = rotation {
            iter = Box::new(self.pieces.iter().rev().enumerate()) as Box<dyn Iterator<Item = _>>;
        } else {
            iter = Box::new(self.pieces.iter().enumerate()) as Box<dyn Iterator<Item = _>>;
        }
        for (i, row) in iter {
            for j in 0..options.tile_lines() {
                // make vec of spans
                let mut spans = Vec::new();

                spans.push(options.vert_frame_char());

                for k in if ChessColor::Black == rotation {
                    Box::new((0..8usize).rev()) as Box<dyn Iterator<Item = _>>
                } else {
                    Box::new(0..8usize) as Box<dyn Iterator<Item = _>>
                } {
                    spans.extend_from_slice(
                        &options.get_tile(Tile::new(
                            if (((((i % 2) == 0) && ((k % 2usize) == 0usize))
                                || (((i % 2) != 0) && ((k % 2usize) != 0usize)))
                                && rotation == ChessColor::White)
                                || (((((i % 2) == 0) && ((k % 2usize) != 0usize))
                                    || (((i % 2) != 0) && ((k % 2usize) == 0usize)))
                                    && rotation == ChessColor::Black)
                            {
                                ChessColor::White
                            } else {
                                ChessColor::Black
                            },
                            Option::map(row[k], |v: (Piece, bool)| v.0),
                        ))[j]
                            .clone(),
                    );
                }

                spans.push(options.vert_frame_char());

                // push line from vec of spans
                text.push(Line::from(spans));
            }
        }

        text.push(Line::from(options.get_frame(Side::Bottom)));

        Text::from(text)
    }
}
