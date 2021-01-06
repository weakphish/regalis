use std::io::{self, Write};

const BOARD_DIMENSIONS: usize = 8;

/// This structure represents a chess board and associated state.
pub struct Board {
    state: [[Piece; 8]; 8],
}

/// This struct represents a game of Chess with associated state.
pub(crate) struct Game {
    turn: Turn,
    board: Board,
}

impl Game {
    /// Create a new game
    pub fn new() -> Game {
        let mut new_game = Game {
            turn: Turn::WhiteTurn,
            board: Board {
                state: [[Piece::EmptySquare; BOARD_DIMENSIONS]; BOARD_DIMENSIONS],
            },
        };
        // Initialize board state
        // White pawns
        for file in 0..BOARD_DIMENSIONS {
            new_game.board.state[1][file] = Piece::WhitePawn;
        }
        // White back rank
        new_game.board.state[0][0] = Piece::WhiteRook;
        new_game.board.state[0][1] = Piece::WhiteKnight;
        new_game.board.state[0][2] = Piece::WhiteBishop;
        new_game.board.state[0][3] = Piece::WhiteQueen;
        new_game.board.state[0][4] = Piece::WhiteKing;
        new_game.board.state[0][5] = Piece::WhiteBishop;
        new_game.board.state[0][6] = Piece::WhiteKnight;
        new_game.board.state[0][7] = Piece::WhiteRook;

        // Black pawns
        for file in 0..BOARD_DIMENSIONS {
            new_game.board.state[6][file] = Piece::BlackPawn;
        }
        // Black back rank
        new_game.board.state[7][0] = Piece::BlackRook;

        new_game.board.state[7][1] = Piece::BlackKnight;
        new_game.board.state[7][2] = Piece::BlackBishop;
        new_game.board.state[7][3] = Piece::BlackQueen;
        new_game.board.state[7][4] = Piece::BlackKing;
        new_game.board.state[7][5] = Piece::BlackBishop;
        new_game.board.state[7][6] = Piece::BlackKnight;
        new_game.board.state[7][7] = Piece::BlackRook;

        return new_game;
    }

    /// Main game loop
    pub fn run_game(&mut self) {
        let game_over: bool = false;
        while !game_over {
            self.print_board();
            println!();

            // Get input for the current user
            // -> somehow call engine to make the move
            let mut user_input = String::new();
            match self.turn {
                Turn::WhiteTurn => print!("White move: "),
                Turn::BlackTurn => print!("Black move: "),
            };

            io::stdout().flush().expect("Could not read input.");
            io::stdin()
                .read_line(&mut user_input)
                .expect("Couldn't read input.");

            // Parse user input and translate it to a movement
            // -> Helper function to check legality of parsed move

            // Make user move and update board state and turn, check for check / mate

            match self.turn {
                Turn::WhiteTurn => self.turn = Turn::BlackTurn,
                Turn::BlackTurn => self.turn = Turn::WhiteTurn,
            }
        }
    }

    /// Print the game board to the console
    pub fn print_board(&self) {
        for rank in 0..BOARD_DIMENSIONS {
            for file in 0..BOARD_DIMENSIONS {
                match self.board.state[rank][file] {
                    Piece::EmptySquare => print!("x"),
                    Piece::WhitePawn   => print!("p"),
                    Piece::WhiteKnight => print!("n"),
                    Piece::WhiteBishop => print!("b"),
                    Piece::WhiteRook   => print!("r"),
                    Piece::WhiteQueen  => print!("q"),
                    Piece::WhiteKing   => print!("k"),
                    Piece::BlackPawn   => print!("P"),
                    Piece::BlackKnight => print!("N"),
                    Piece::BlackBishop => print!("B"),
                    Piece::BlackRook   => print!("R"),
                    Piece::BlackQueen  => print!("Q"),
                    Piece::BlackKing   => print!("K"),
                }
            }
            println!();
        }
    }
}

/// This enum represents the different types a game piece can be.
#[derive(Copy, Clone)]
enum Piece {
    EmptySquare,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

/// This enum represents the different turns that can exist in a game of chess
enum Turn {
    WhiteTurn,
    BlackTurn,
}
