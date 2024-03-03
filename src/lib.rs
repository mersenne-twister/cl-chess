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
    println!("{}", text::INTRO);

    let mut input = String::new();
    wait_for_enter(&mut input)?;
    Board::new().print(&PieceColor::White)?;

    // move:
    // move a piece
    // usage:
    // `move [piece-name] [position]`
    // `move [position] [position]`
    // if only one piece of that name can move there, move it
    // otherwise print error requiring `move [position] [position]` format
    // UNAMBIGOUS

    // check:
    // see the effect of moving a piece (without actually moving it)
    // usage:
    // `move [piece-name] [position]`
    // `move [position] [position]`

    // hint:
    // get a (questionable) hint on where to move
    // usage:
    // `hint`

    
    loop {
        input.clear();
        get_input(&mut input);
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
