pub const INTRO: &str = "\
Welcome to cl-chess, made by Iris Teyssier!
iris.teyssier@gmail.com";

pub const HELP: &str = "\
moving:
    move a piece. doesn't require any command name.
    `[piece-name] [position]`
    `[position] [position]`
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

redo:
    doesn't exist! make sure when you undo you mean it!

show:
    show the board from the position of either color.
usage:
    `show [color]`
    possible colors: `white`, `black`

exit:
    immediately end the game
usage:
    `exit`

help:
    print help for using the program, or for chess rules.
usage:
    `help`
        print this message.
    `help chess`
        print all chess rules.
    `help [piece-name]`
        print rules for a specific piece
    `help check`
        print rules for check & checkmate
    `help en-passant`
        print rules for en passant
    `help castling`
        print rules for castling
    `help time`
        print information regarding timed games (\"speed chess\")
    ANYTHING ELSE I CAN PRINT? EMAIL ME AT IRIS.TEYSSIER@GMAIL.COM

To print the board, enter without typing anything.";
