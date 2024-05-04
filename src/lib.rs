use {
    crate::board::Position,
    board::{Board, PieceColor},
    std::{
        error::Error,
        io::{self, Write},
    },
    termchess_common::Args,
    termchess_tui,
};

pub mod board;
mod cli;
pub mod parse;
pub mod text;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // tui::run_tui(args)
    termchess_tui::run(args)
}

pub fn run_cli(args: Args) -> Result<(), Box<dyn Error>> {
    // Board::new().can_move_axially(
    //     &Position::from_str("f4").unwrap(),
    //     &Position::from_str("c4").unwrap(),
    // );

    // return Ok(());

    println!("{}\n{}", text::INTRO, text::HELP);

    let mut input = String::new();
    wait_for_enter(&mut input)?;
    let mut turn = PieceColor::White;
    let mut board = Board::new();
    let mut moved_message = None;
    board.print(&turn)?;

    // TODO: add check detection
    // TODO: add checkmate detection
    // TODO: show points
    // TODO: show moves
    // TODO: add undo
    // TODO: print moves to file at end of game
    // TODO: add total time, and time per player
    // TODO: support for timed game (with real-time counter)
    // TODO: add redo error "this dosn't exist etc etc"
    // TODO: make get_pieces return [Pieces; 64] if possible

    // BIG GOALS:
    // USE PROPER NOTATION FOR MOVES
    // KEEP TRACK OF TYPES OF MOVES FOR NOTATION
    // MOVES OBVIOUSLY
    // PRINT FULL MESSAGE WHEN REPRINTING BOARD

    loop {
        print!("{}'s turn: ", turn);
        io::stdout().flush()?; // make the turn message print before the input
        input.clear();
        get_input(&mut input)?;

        let input = input.trim().to_ascii_lowercase(); // this keeps the string from being dropped
        let mut input = input.as_str().split_ascii_whitespace();
        match input.next().unwrap_or("") {
            "undo" => todo!(),
            "check" => todo!(),
            "hint" => todo!(),
            "help" => {
                println!("{}", text::HELP);
                continue;
            }
            "" => {
                // board.print(&turn)?;
                continue;
            }
            "debug" => moved_message = Some("debug".to_owned()), // debug command
            // _ => {
            //     println!("Enter 'help' to see Help.");
            //     continue;
            // }
            piece_move => {
                // let arg = input.next().unwrap_or("default");
                let arg = piece_move;
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
                                moved_message = Some(str);
                            }
                        } else {
                            println!(
                            "Invalid arguments. Expected piece name or position.\nEnter 'help' to see help.");
                            continue;
                        }
                    }
                }
                if moved_message.is_none() {
                    moved_message = Some(format!(
                        "{} moved {} {} to {}",
                        turn,
                        board
                            .get_piece(&move_position)
                            .as_ref()
                            .expect("we just moved a piece here, so it must have a piece")
                            .0,
                        moved_position,
                        move_position
                    ));
                }
            }
        }

        turn = if turn == PieceColor::White {
            PieceColor::Black
        } else {
            PieceColor::White
        };
        board.print(&turn)?;
        println!(
            "{}",
            moved_message.expect("should have been assigned by this point")
        );
        moved_message = None;
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
    buf.clear();
    io::stdin().read_line(buf)?;
    Ok(())
}
