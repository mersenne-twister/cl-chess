use {
    crate::board::Position,
    args::Args,
    board::{Board, PieceColor},
    std::{
        error::Error,
        io::{self, Write},
        thread,
        time::Duration,
    },
};

pub mod args;
pub mod board;
pub mod parse;
pub mod text;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    Board::new().can_move_diagonally(&Position::from_str("a1").unwrap(), &Position::from_str("d4").unwrap());

    return Ok(());


    

    println!("{}\n{}", text::INTRO, text::HELP);

    let mut input = String::new();
    wait_for_enter(&mut input)?;
    let mut turn = PieceColor::White;
    let mut board = Board::new();
    let mut moved_message = String::new();
    board.print(&turn)?;

    // TODO: add check detection
    // TODO: add checkmate detection
    // TODO: show points
    // TODO: show moves
    // TODO: add undo
    // TODO: print moves to file at end of game
    // TODO: add total time, and time per player
    // TODO: support for timed game (with real-time counter)

    loop {
        print!("{}'s turn: ", turn);
        io::stdout().flush()?; // make the turn message print before the input
        input.clear();
        get_input(&mut input)?;

        let input = input.trim().to_ascii_lowercase(); // this keeps the string from being dropped
        let mut input = input.as_str().split_ascii_whitespace();
        match input.next().unwrap_or("") {
            "move" => {
                let arg = input.next().unwrap_or("default");
                let Ok(move_position) = Position::from_str(input.next().unwrap_or("default"))
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
                        if let Ok(piece_position) = Position::from_str(position) {
                            moved_position = piece_position.clone();

                            let result = board.try_move(&piece_position, &move_position);
                            if let Err(err) = result {
                                println!("{}", err);
                                continue;
                            } else if let Ok(Some(str)) = result {
                                moved_message = str;
                            }
                        } else {
                            println!(
                            "Invalid arguments. Expected piece name or position.\nEnter 'help' to see help.");
                            continue;
                        }
                    }
                }
                if moved_message.len() == 0 {
                    moved_message = format!(
                        "{} moved {} {} to {}",
                        turn,
                        board
                            .get_piece(&move_position)
                            .as_ref()
                            .expect("we just moved a piece here, so it must have a piece")
                            .0,
                        moved_position,
                        move_position
                    );
                }
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
        board.print(&turn)?;
        println!("{}", moved_message);
        moved_message.clear();
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
