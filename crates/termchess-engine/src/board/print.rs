use {
    crate::board::{self, ascii::Tile, Board, Color as ChessColor, Piece},
    // super::Game,
    ratatui::{
        style::{Color, Stylize},
        text::Text,
        widgets::Paragraph,
    },
    std::error::Error,
    termchess_common::TResult,
};

#[derive(Default)]
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
    pub fn get_board(&self, tile: Tile) -> Color {
        if tile.board_color == ChessColor::White {
            self.board_white
        } else {
            self.board_black
        }
    }

    pub fn get_piece(&self, tile: Tile) -> Option<Color> {
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
pub struct BoardOptions {
    pub size: Size,
    pub frame: Option<Frame>,
    pub axis: Axis,
    pub theme: Theme,
}

// highlight: Option<Position>
impl Board {
    pub fn print(&self) -> Text {
        todo!()
    }
}
