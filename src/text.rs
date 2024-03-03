pub const INTRO: &str = "\
Welcome to cl-chess, made by Iris Teyssier!
iris.teyssier@gmail.com";

pub const HELP: &str = "\
move:
move a piece
usage:
    `move [piece-name] [position]`
    `move [position] [position]`
if only one piece of that name can move there, move it
otherwise print error requiring `move [position] [position]` format
UNAMBIGOUS

check:
see the effect of moving a piece (without actually moving it)
usage:
    `move [piece-name] [position]`
    `move [position] [position]`

hint:
get a (questionable) hint on where to move
usage:
  `hint`";
