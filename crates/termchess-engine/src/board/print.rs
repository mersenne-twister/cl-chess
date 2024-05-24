/// All board *rendering* logic goes in here, all logic concerned with ascii
/// or colouring goes in `ascii`
// rustfmt wants to remove this line so just ignore this
use {
    crate::board::{
        self,
        ascii::{self, FrameChar, HorizSide, Tile, VertSide},
        Board, Co as ChessColor, Piece, Position,
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
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Axis {
    // settings options:
    // all, none, bottom & left ("normal", and the default one)
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
        Self::new(true, true, true, true)
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
    pub piece_white_highlight: Color,
    pub piece_black: Color,
    pub piece_black_highlight: Color,
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

    pub fn get_piece(&self, tile: Tile, highlight: bool) -> Option<Color> {
        tile.piece.map(|v| match (v.color, highlight) {
            (ChessColor::White, false) => self.piece_white,
            (ChessColor::White, true) => self.piece_white_highlight,
            (ChessColor::Black, false) => self.piece_black,
            (ChessColor::Black, true) => self.piece_black_highlight,
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            board_white: Color::Indexed(216),
            board_black: Color::Indexed(208),
            piece_white: Color::White,
            piece_white_highlight: Color::Indexed(250),
            // piece_white_highlight: Color::Red,
            piece_black: Color::Black,
            piece_black_highlight: Color::Indexed(240),
            // piece_black_highlight: Color::Red,
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
    pub fn border_width(&self, side: HorizSide) -> usize {
        3 + if side == HorizSide::Left {
            // 1 * 2 if true, 0 * 2 if false
            usize::from(self.axis.left) * 2
        } else {
            usize::from(self.axis.right) * 2
        }
    }

    /// get width of one of the strings
    pub fn tile_width(&self) -> usize {
        self.size.get_chars(None)[0].len()
    }

    pub fn width(&self) -> usize {
        self.tile_width() * 8
            + self.border_width(HorizSide::Left)
            + self.border_width(HorizSide::Right)
    }

    pub fn border_height(&self, side: VertSide) -> usize {
        // 1 + usize::from(self.axis.top) + usize::from(self.axis.bottom)
        1 + if side == VertSide::Top {
            usize::from(self.axis.top)
        } else {
            usize::from(self.axis.bottom)
        }
    }

    /// get number of strings
    pub fn tile_height(&self) -> usize {
        self.size.get_chars(None).len()
    }

    pub fn height(&self) -> usize {
        self.tile_height() * 8
            + self.border_height(VertSide::Top)
            + self.border_height(VertSide::Bottom)
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
    pub fn print<'a>(
        &self,
        options: &'a BoardOptions,
        rotation: ChessColor,
        // TODO: use `Position`
        highlight: Option<(usize, usize)>,
    ) -> Text<'a> {
        // ENSURE THAT POSITION WORKS AS EXPECTED

        let mut text = Vec::new();

        if options.axis.top {
            text.push(Line::from(options.get_axis(VertSide::Top)));
        }
        text.push(Line::from(options.get_frame(VertSide::Top)));

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

                spans.push(options.vert_axis_char(i, j, HorizSide::Left));
                spans.push(options.vert_frame_char());

                for k in if ChessColor::Black == rotation {
                    Box::new((0..8usize).rev()) as Box<dyn Iterator<Item = _>>
                } else {
                    Box::new(0..8usize) as Box<dyn Iterator<Item = _>>
                } {
                    spans.push(
                        options.get_tile(
                            Tile::new(
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
                            ),
                            highlight.map_or_else(
                                || false,
                                |v| (v.1 as usize == i) && (v.0 as usize == k),
                            ),
                        )[j]
                            .clone(),
                    );
                }

                spans.push(options.vert_frame_char());
                spans.push(options.vert_axis_char(i, j, HorizSide::Right));

                // push line from vec of spans
                text.push(Line::from(spans));
            }
        }

        if options.axis.bottom {
            text.push(Line::from(options.get_frame(VertSide::Bottom)));
        }
        text.push(Line::from(options.get_axis(VertSide::Bottom)));

        Text::from(text)
    }
}
