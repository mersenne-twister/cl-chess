use {
    args::Args,
    board::{Board, PieceColor},
    colored::Colorize,
    std::{error::Error, io},
};

pub mod args;
pub mod board;
pub mod parse;
pub mod text;

pub fn run(_args: Args) -> Result<(), Box<dyn Error>> {
    println!("{}\n{}", text::INTRO, text::HELP);

    let mut input = String::new();
    wait_for_enter(&mut input)?;
    let mut turn = PieceColor::White;
    let mut board = Board::new();
    board.print(&turn)?;

    loop {
        println!("{}'s turn:", turn);
        input.clear();
        get_input(&mut input)?;

        let input = input.to_ascii_lowercase(); // this keeps the string from being dropped
        let mut input = input.as_str().split_ascii_whitespace();
        match input.next().unwrap_or("") {
            "move" => match input.next().unwrap_or("default") {
                piece @ ("pawn" | "knight" | "bishop" | "rook" | "queen" | "king") => {
                    todo!();
                }
                "default" => {
                    println!("`move` command requires arguments. Enter `help` for details.");
                    continue;
                }
                position => {
                    todo!();

                    println!(
                        "Invalid argument. Expected piece name or position.\nEnter 'help' to see Help.");
                }
            },
            "check" => todo!(),
            "hint" => todo!(),
            "help" => {
                println!("{}", text::HELP);
                continue;
            }
            "" => {
                board.print(&turn)?;
                continue;
            }
            "fuck" => (), // debug command
            _ => {
                println!("Enter 'help' to see Help.");
                continue;
            }
        }

        turn = (if turn == PieceColor::White {
            PieceColor::Black
        } else {
            PieceColor::White
        });
        board.print(&turn);
    }

    Ok(())
}

fn wait_for_enter(input: &mut String) -> Result<(), Box<dyn Error>> {
    println!("Press enter to continue . . .");
    get_input(input)?;
    Ok(())
}

fn get_input(buf: &mut String) -> Result<(), Box<dyn Error>> {
    // io::stdin().read_line(buf).unwrap_or_else(|err| {
    //     println!("Unexpected input error: {}", err);
    //     process::exit(1);
    // });
    io::stdin().read_line(buf)?;
    Ok(())
}
