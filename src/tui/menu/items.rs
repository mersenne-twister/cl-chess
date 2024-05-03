use {
    super::{Menu, Terminal},
    crate::{args::Args, TResult},
    clap::ValueEnum,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    ratatui::{
        prelude::*,
        symbols::border,
        widgets::{
            block::{Position, Title},
            Block, Borders, Paragraph,
        },
    },
    std::{
        error::Error,
        fmt::Display,
        io::{self, stdout, Stdout},
        time::{Duration, Instant},
    },
    strum::{EnumCount, IntoEnumIterator},
    strum_macros::{EnumCount as EnumCountMacro, EnumIter, FromRepr},
};

/// in order for them to allign correctly, they have to all use either
/// an even or odd number of characters
/// which is an *insane* invariant
///
/// turns out that to be alligned, it *has* to be even
/// (including the spaces which get added)
#[derive(Default, PartialEq, Debug, EnumIter, EnumCountMacro, FromRepr)]
pub enum Item {
    #[default]
    CommenceDefault,
    Settings,
    Quit,
}

impl Item {
    pub fn handle(&self, menu: &mut Menu) {
        match self {
            Self::CommenceDefault => todo!(),
            Self::Settings => todo!(),
            Self::Quit => menu.exit = true,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{:?}", self)
                .chars()
                .map(|ch| {
                    if ch.is_ascii_uppercase() {
                        format!(" {}", ch)
                    } else {
                        ch.to_string()
                    }
                })
                .collect::<String>()
                .trim()
        )
    }
}
