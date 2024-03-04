pub const INTRO: &str = "\
Welcome to cl-chess, made by Iris Teyssier!
iris.teyssier@gmail.com";

pub const HELP: &str = "\
move:
move a piece
usage:
    `move [piece-name] [position]`
    `move [position] [position]`
    position: a letter followed by a number (example: e5)
    piece-name: the name of a piece
        (possible names: `pawn`, knight, bishop, rook, queen, king)
if intended piece is unambigous, move it
otherwise print error requiring `move [position] [position]` format

check:
see the effect of moving a piece (without actually moving it)
usage:
    `check [piece-name] [position]`
    `check [position] [position]`

hint:
get a (questionable) hint on where to move
usage:
  `hint`

undo:
    rollbacks 1 move.
usage:
    `undo`

show:
    show the board from the position of either color
usage:
    `show [color]`
    possible colors: `white`, `black`

exit:
    immediately end the game
usage:
    `exit`

 To print the board, enter without typing anything.";
