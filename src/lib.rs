use {
    crate::board::Position,
    args::Args,
    board::{Board, PieceColor},
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
    // board.print(&turn)?;

    loop {
        board.print(&turn)?;
        println!("{}'s turn:", turn);
        input.clear();
        get_input(&mut input)?;

        let input = input.to_ascii_lowercase(); // this keeps the string from being dropped
        let mut input = input.as_str().split_ascii_whitespace();
        match input.next().unwrap_or("") {
            "move" => {
                let arg = input.next().unwrap_or("default");
                let Ok(mut move_position) =
                    Position::from_str(&board, input.next().unwrap_or("default"))
                else {
                    println!(
                        "Invalid arguments. Expected move position.\nEnter 'help' to see help"
                    );
                    continue;
                };
                let moved_position;

                match arg {
                    piece @ ("pawn" | "knight" | "bishop" | "rook" | "queen" | "king") => {
                        todo!();
                    }
                    "default" => {
                        println!("`move` command requires arguments. Enter `help` for details.");
                        continue;
                    }
                    position => {
                        if let Ok(piece_position) = Position::from_str(&board, position) {
                            moved_position = piece_position.clone();
                            if !board.has_piece(&piece_position) {
                                println!("Specified position must contain a piece");
                                continue;
                            }
                            // position.swap(&mut move_position);
                            board.try_move(&piece_position, &move_position);
                        } else {
                            println!(
                            "Invalid arguments. Expected piece name or position.\nEnter 'help' to see help.");
                            continue;
                        }
                    }
                }

                println!(
                    "{} moved {} {} to {}",
                    turn,
                    board.get_piece(&move_position).expect("verified to have a piece"),
                    moved_position,
                    move_position
                );
            }
            "undo" => todo!(),
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

        turn = if turn == PieceColor::White {
            PieceColor::Black
        } else {
            PieceColor::White
        };
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
