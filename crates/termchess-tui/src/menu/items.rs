use {
    super::Menu,
    crate::{game::Game, Run},
    std::fmt::Display,
    strum_macros::{EnumCount as EnumCountMacro, EnumIter, FromRepr},
    termchess_common::TResult,
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
    pub fn handle(&self, menu: &mut Menu) -> TResult<()> {
        match self {
            Self::CommenceDefault => Game::new(menu.terminal.clone()).run()?,
            Self::Settings => todo!(),
            Self::Quit => menu.exit = true,
        }

        Ok(())
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
