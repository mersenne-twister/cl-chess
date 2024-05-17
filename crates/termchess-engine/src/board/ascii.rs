use {
    // super::{BoardOptions, Size, Theme},
    crate::board::{
        print::{BoardOptions, Frame, Size, Theme},
        Color as ChessColor, Piece, PieceName,
    },
    lazy_static::lazy_static,
    ratatui::{
        style::{Color, Stylize},
        text::Span,
    },
    std::collections::HashMap,
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd)]
pub struct Tile {
    // Black(Option<Piece>),
    // White(Option<Piece>),
    pub board_color: ChessColor,
    pub piece: Option<Piece>,
}

impl Tile {
    pub const fn new(board_color: ChessColor, piece: Option<Piece>) -> Self {
        Self { board_color, piece }
    }
}

// done so that these are accessible from board.rs
// // pub const BOARD_BLACK: Color = Color::Rgb{r: 216, g: 135, b: 85};
// pub const BOARD_BLACK: Color = Color::Rgb(177, 97, 60);
// // pub const BOARD_WHITE: Color = Color::Rgb{r: 254, g: 210, b: 169};
// // pub const BOARD_WHITE: Color = Color::Rgb{r: 240, g: 195, b: 128};
// pub const BOARD_WHITE: Color = Color::Rgb(235, 178, 94);
// // pub const BORDER_BACKGROUND: Color = Color::Rgb{r: 122, g: 77, b: 49};
// pub const BORDER_BACKGROUND: Color = Color::Rgb(82, 46, 22);
// pub const BORDER_TEXT: Color = Color::Rgb(255, 255, 255);
// pub const PIECE_BLACK: Color = Color::Rgb(0, 0, 0);
// pub const PIECE_WHITE: Color = Color::Rgb(255, 255, 255);

// pub const BORDER_LINE_BLANK: &str =
//     "                                                                                  ";
// pub const BORDER_LINE_LETTERS: &str =
//     "         A        B        C        D        E        F        G        H         ";
// pub const BORDER_LINE_LETTERS_REVERSED: &str =
//     "         H        G        F        E        D        C        B        A         ";

pub enum FrameChar {
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

impl Frame {
    const fn get_char(&self, ch: FrameChar) -> char {
        match *self {
            Self::Double => match ch {
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
            _ => todo!(),
        }
    }
}

impl BoardOptions {
    pub fn get_tile(&self, tile: Tile) -> Vec<Vec<Span<'_>>> {
        self.size
            .get_chars(tile.piece)
            .iter()
            .map(|str| vec![self.set_colors((*str).clone(), tile)])
            .collect()
    }

    fn set_colors(&self, str: String, tile: Tile) -> Span<'_> {
        // str.bg(self.theme.get_board(tile))
        self.theme
            .get_piece(tile)
            // make bold?
            .map_or_else(|| Span::from(str.clone()), |c| str.clone().fg(c))
            .bg(self.theme.get_board(tile))
    }

    // TODO: use this everywhere
    fn set_border_colors(&self, str: String, tile: Tile) -> Span<'_> {
        todo!()
    }

    pub fn get_frame(&self, side: VertSide) -> Span {
        let (start, mid, end) = match side {
            VertSide::Top => ("╔", "═", "╗"),
            VertSide::Bottom => ("╚", "═", "╝"),
        };

        let left = self.border_width(HorizSide::Left);
        let right = self.border_width(HorizSide::Right);

        format!(
            "{}{}{}{}{}",
            " ".repeat(left - 2),
            start,
            // 2 spaces on either side of board
            mid.repeat(self.width() - (left + right - 2)),
            end,
            " ".repeat(right - 2),
        )
        .fg(self.theme.border_fg)
        .bg(self.theme.border_bg)
    }

    pub fn get_axis(&self, side: VertSide) -> Span {
        // these must be an odd number of characters in a tile
        let spaces = " ".repeat((self.tile_width() - 1) / 2);

        let mut tiles = String::new();
        for i in 0..8 {
            // 65 == 'A', ...
            tiles.push_str(
                format!(
                    "{}{}{}",
                    spaces,
                    char::from_u32(i + 65).expect("should be valid"),
                    spaces,
                )
                .as_ref(),
            );
        }

        format!(
            "{}{}{}",
            " ".repeat(self.border_width(HorizSide::Left)),
            tiles,
            " ".repeat(self.border_width(HorizSide::Left)),
        )
        .fg(self.theme.border_fg)
        .bg(self.theme.border_bg)
    }

    pub fn vert_frame_char(&self) -> Span {
        " ║ ".fg(self.theme.border_fg).bg(self.theme.border_bg)
    }

    pub fn vert_axis_char(&self, rank: usize, row: usize, side: HorizSide) -> Span {
        if (((side == HorizSide::Left) && self.axis.left)
            || ((side == HorizSide::Right) && self.axis.right))
            && true
        {
            // TODO: add more spaces so the board is squarer
            let space = " ".to_owned();
            // TODO: make test that ensures all have even, other than that one,
            // and similar invariants, like even chars for menu items,
            // and all sprites having the same number of chars
            let rank = if
            // if it's one, just print it no matter what
            (self.tile_height() == 1)
            // tile_height is odd, so we can do it the right way
            || (((self.tile_height() % 2) != 0)
                && (row == (self.tile_height().div_ceil(2))))
                //tile_height is 4, so we use our special case
                || ((self.tile_height() == 4)
                    && (((rank <= 3) && (row == 1)) || ((rank >= 4) && (row == 2))))
            {
                (rank as i8 - 8).abs().to_string()
            } else {
                " ".to_owned()
            };

            let (first, second) = if side == HorizSide::Left {
                (space, rank)
            } else {
                (rank, space)
            };
            format!("{}{}", first, second)
        } else {
            " ".to_owned()
        }
        .fg(self.theme.border_fg)
        .bg(self.theme.border_bg)
    }
}

// are two really necessary? how can I refactor this without introducing
// ambiguity?
#[derive(PartialEq, Eq)]
pub enum VertSide {
    Top,
    Bottom,
}

#[derive(PartialEq, Eq)]
pub enum HorizSide {
    Left,
    Right,
}

impl Size {
    const fn min_border_width(&self) -> usize {
        match *self {
            Self::Letters { .. } | Self::UnicodeSymbols { .. } => 1,
            Self::UnicodeArt => 3,
            _ => todo!(),
        }
    }

    pub fn get_chars(&self, piece: Option<Piece>) -> Vec<String> {
        let name = piece.map(|v| v.name);
        match self {
            Size::Letters {
                different_symbols: diff,
            } => vec![format!(" {} ", if !diff
                || piece.map(|v| v.color).unwrap_or(ChessColor::White) == ChessColor::White
            {
                // first one is used normally, latter for black pieces if `diff`
                match name {
                    None => " ",
                    Some(PieceName::Pawn) => "P",
                    Some(PieceName::Knight) => "N",
                    Some(PieceName::Bishop) => "B",
                    Some(PieceName::Rook) => "R",
                    Some(PieceName::Queen) => "Q",
                    Some(PieceName::King) => "K",
                }
            } else {
                match name {
                    None => unreachable!(),
                    Some(PieceName::Pawn) => "p",
                    Some(PieceName::Knight) => "n",
                    Some(PieceName::Bishop) => "b",
                    Some(PieceName::Rook) => "r",
                    Some(PieceName::Queen) => "q",
                    Some(PieceName::King) => "k",
                }
            }
            .to_string())],
            Size::UnicodeSymbols {
                different_symbols: diff,
            } => vec![format!(" {} ", if !diff
                || piece.map(|v| v.color).unwrap_or(ChessColor::White) == ChessColor::White
            {
                // see above arm
                match name {
                    None => " ",
                    Some(PieceName::Pawn) => "♟︎",
                    Some(PieceName::Knight) => "♞",
                    Some(PieceName::Bishop) => "♝",
                    Some(PieceName::Rook) => "♜",
                    Some(PieceName::Queen) => "♛",
                    Some(PieceName::King) => "♚",
                }
            } else {
                match name {
                    None => unreachable!(),
                    Some(PieceName::Pawn) => "♙",
                    Some(PieceName::Knight) => "♘",
                    Some(PieceName::Bishop) => "♗",
                    Some(PieceName::Rook) => "♖",
                    Some(PieceName::Queen) => "♕",
                    Some(PieceName::King) => "♔",
                }
            }
            .to_string())],
            Size::UnicodeArt => Vec::from(match name {
                // see `/chess-pieces` (in root dir) to see pieces
                None => ["     ", "     ", "     ", "     "],
                Some(PieceName::Pawn) => ["     ", "  •  ", "  │  ", "  ┴  "],
                // find one that combines first and second
                // make shorter
                // Some(PieceName::Knight) => ["  ╥  ", " /╣  ", "  ║  ", "  ╨  "],
                Some(PieceName::Knight) => ["  _  ", " ╱╣  ", "  ║  ", "  ╨  "],
                // make shorter?
                // Some(PieceName::Bishop) => ["  │  ", "  ║  ", "  │  ", "  ┴  "],
                Some(PieceName::Bishop) => ["     ", "  │  ", "  ║  ", "  ┴  "],
                // move top down one?
                // Some(PieceName::Rook) => [" ╘╬╛ ", "  ║  ", "  ║  ", "  ╨  "],
                Some(PieceName::Rook) => ["     ", " ╘╬╛ ", "  ║  ", "  ╨  "],
                // use ☼ instead of •?
                Some(PieceName::Queen) => ["• • •", " ╲╫╱ ", "  ║  ", "  ╨  "],
                Some(PieceName::King) => ["  +  ", "  ╫  ", "  ║  ", "  ╨  "],
            })
            .iter()
            .map(|str| format!("  {}  ", str))
            .collect(),
            // probably make this one with blocks
            Size::BlockArt => Vec::from(match name {
                None => [
                    "         ",
                    "         ",
                    "         ",
                    "         ",
                    "         ",
                ],
                Some(PieceName::Pawn) => [
                    "         ",
                    "    █    ",
                    "   ███   ",
                    "    █    ",
                    "   ███   ",
                ],
                Some(PieceName::Knight) => [
                    "    ██   ",
                    "  █████  ",
                    " ██ ████ ",
                    "   ████  ",
                    "  █████  ",
                ],
                Some(PieceName::Bishop) => [
                    "    █    ",
                    "   ███   ",
                    "   ███   ",
                    "    █    ",
                    "  █████  ",
                ],
                Some(PieceName::Rook) => [
                    "  █ █ █  ",
                    "  █████  ",
                    "   ███   ",
                    "   ███   ",
                    "  █████  ",
                ],
                Some(PieceName::Queen) => [
                    " █  █  █ ",
                    "  █ █ █  ",
                    "   ███   ",
                    "   ███   ",
                    "  █████  ",
                ],
                Some(PieceName::King) => [
                    "    █    ",
                    " ██ █ ██ ",
                    "█  █ █  █",
                    " █  █  █ ",
                    "  █████  ",
                ],
            })
            .iter()
            .map(|str| format!(" {} ", str))
            .collect(),
            // some large one, haven't decided the style
            // can always add more anyways, not that hard
            Size::TbdLarge => todo!(),
        }
    }
}
