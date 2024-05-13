use {
    crate::board::{
        self,
        ascii::{FrameChar, Tile},
        Board, Color as ChessColor, Piece,
    },
    // super::Game,
    ratatui::{
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
    Ascii,
    #[default]
    Unicode,
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
}

impl Default for Axis {
    fn default() -> Self {
        Self::new_all()
    }
}

// themename struct, with lists of themes, and you pass a variant to theme::new?
#[derive(Clone, Copy)]
pub enum ThemeName {
    Foo,
    Bar,
    Baz,
}

#[derive(Clone)]
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
    pub fn new(name: ThemeName) -> Self {
        match name {
            ThemeName::Foo => todo!(),
            ThemeName::Bar => todo!(),
            ThemeName::Baz => todo!(),
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
                self.board_white
            } else {
                self.board_black
            }
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            board_white: Color::Indexed(208),
            board_black: Color::Indexed(216),
            piece_white: Color::White,
            piece_black: Color::Black,
            frame_fg: Color::default(),
            frame_bg: Color::default(),
            axis_fg: Color::default(),
            axis_bg: Color::default(),
        }
    }
}

#[derive(Default, Clone)]
pub struct BoardOptions {
    pub size: Size,
    pub frame: Option<Frame>,
    pub axis: Axis,
    pub theme: Theme,
}

impl BoardOptions {
    /// return width|height of board
    pub fn breadth(&self) -> usize {
        todo!()
    }

    ///  return the lines in one tile
    pub fn tile_lines(&self) -> usize {
        // todo!()
        1
    }

    /// return cols in one tile
    pub fn tile_cols(&self) -> usize {
        todo!()
    }
}

// highlight: Option<Position>
impl Board {
    pub fn print(&self, options: BoardOptions, rotation: &ChessColor) -> Text {
        assert!(options.frame.is_none());
        assert!(
            options.size
                == Size::Letters {
                    different_symbols: false
                }
        );

        let mut text = Vec::new();

        // casting to a trait object is required because both possible values
        // must have the same type
        let iter: Box<dyn Iterator<Item = (usize, &[Option<(Piece, bool)>; 8])>>;
        if let ChessColor::Black = *rotation {
            iter = Box::new(self.pieces.iter().rev().enumerate()) as Box<dyn Iterator<Item = _>>;
        } else {
            iter = Box::new(self.pieces.iter().enumerate()) as Box<dyn Iterator<Item = _>>;
        }
        for (i, row) in iter {
            for j in 0..options.tile_lines() {
                for k in if ChessColor::Black == *rotation {
                    Box::new((0..8usize).rev()) as Box<dyn Iterator<Item = _>>
                } else {
                    Box::new(0..8usize) as Box<dyn Iterator<Item = _>>
                } {
                    text.push(Line::from(
                        //
                        options.clone().get_tile(Tile::new(
                            if (((((i % 2) == 0) && ((k % 2usize) == 0usize))
                                || (((i % 2) != 0) && ((k % 2usize) != 0usize)))
                                && *rotation == ChessColor::White)
                                || (((((i % 2) == 0) && ((k % 2usize) != 0usize))
                                    || (((i % 2) != 0) && ((k % 2usize) == 0usize)))
                                    && *rotation == ChessColor::Black)
                            {
                                ChessColor::White
                            } else {
                                ChessColor::Black
                            },
                            // row[k].map(|v| v.0),
                            Option::map(row[k], |v: (Piece, bool)| v.0),
                        ))[j]
                            .clone(),
                    ));
                }
            }
        }

        Text::from(text)
    }
}
