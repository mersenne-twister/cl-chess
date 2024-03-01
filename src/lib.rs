use {args::Args, std::{error::Error, io}};

pub mod args;
pub mod text;

pub fn run(_args: Args) {
    println!("{}", text::intro);

    let mut input = String::new();

    // move:
        // move a piece
        // usage:
            // `move [piece-name] [position]`
            // `move [position] [position]`
        // if only one piece of that name can move there, move it
        // otherwise print error requiring `move [position] [position]` format
    
    // check:
        // see the effect of moving a piece (without actually moving it)
        // usage:
            // `move [piece-name] [position]`
            // `move [position] [position]`
    
    // hint:
        // get a (questionable) hint on where to move
        // usage:
            // `hint`
    


}

fn wait_for_enter(input: &mut String) {
    println!("Press enter to continue . . .");
    get_input(input);
}

fn get_input(buf: &mut String) -> Result<(), Box<dyn Error>> {
    // io::stdin().read_line(buf).unwrap_or_else(|err| {
    //     println!("Unexpected input error: {}", err);
    //     process::exit(1);
    // });
    io::stdin().read_line(buf)?
}
