use std::io::{self, Write};

/// Standard size of a chess board
const BOARD_DIMENSIONS: usize = 8;

/// This structure represents the drawn chessboard to be updated after each move
pub struct Board {
    state:[[Piece; BOARD_DIMENSIONS]; BOARD_DIMENSIONS],
}

/// This struct represents a game of Chess along with whoever's turn it is
pub(crate) struct Game {
    turn: Color,
    board: Board,
}

/// This enum represents the different colors the pieces can take
#[derive(Copy, Clone)]
enum Color {
    White,
    Black,
    Empty, 
}
/// Struct that represents the current position. x,y must be less than BOARD_DIMENSIONS
#[derive(Copy, Clone)]
struct Position {
    x: i8,
    y: i8,
}
/// Struct that determines a movement in terms of a beginning and ending position
#[derive(Copy, Clone)]
struct Move {
    start: Position,
    end:   Position,
}

/// Creates a structure that represents a chess Piece
#[derive(Copy, Clone)]
struct Piece {
    board_rep:    char,
    captured:    bool,
    first_move:   bool,
    color:       Color,
    position:    Position,
    is_move_valid: fn (Piece, Move) -> bool,
}

fn is_pawn_move_valid(pawn: Piece, movement: Move) -> bool{
    if pawn.captured {
        return false;
    }
    //Can only move forward one unless it is the first time this Piece is moving
    let mut y = movement.end.y - movement.start.y;
    //Since the "white" player is at the bottom, we flip the value for this check if the piece
    //is black
    match pawn.color {
        Color::Black => {
            y = -y
        }
        Color::White => {}
        Color::Empty => {}
    }
    let x = movement.end.x - movement.start.x; //Currently not checking for attack
    //This checks if the pawn is moving one block forward or two blocks if it's the first move
    if ((y == 1) || (y == 2 && pawn.first_move)) && x == 0 {
        return true;
    } 
    if movement.end.x >= BOARD_DIMENSIONS as i8 || movement.end.y >= BOARD_DIMENSIONS as i8 {
        //This checks if the piece is trying to move off the board
        return false;
    }
    return false;
}

fn is_rook_move_valid(rook: Piece, movement: Move) -> bool {
    if rook.captured {
        return false;
    }
    if movement.end.x >= BOARD_DIMENSIONS as i8 || movement.end.y >= BOARD_DIMENSIONS as i8 {
        //This checks if the piece is trying to move off the board
        return false;
    }
    //Can move in a straight line the entire length of the board
    let x = movement.end.x - movement.start.x;
    let y = movement.end.y - movement.start.y;
    if x != 0 && y != 0 {
        // This is a diagonal move, so invalid
        return false;
    }
    // TODO Check if the Rook is jumping over a piece here
    return true;
    }

fn is_bishop_move_valid(bishop: Piece, movement: Move) -> bool {
    if bishop.captured {
        return false;
    }
    if movement.end.x >= BOARD_DIMENSIONS as i8 || movement.end.y >= BOARD_DIMENSIONS as i8 {
        //This checks if the piece is trying to move off the board
        return false;
    }
    //Can move diagonally across the entire board 
    let x = movement.end.x - movement.start.x;
    let y = movement.end.y - movement.start.y;
    // A piece is moving diagonally iff it's x movement is equal to it's y movement (ignoring
    // sign)
    if x.abs() != y.abs() {
        return false;
    }
    // TODO Check if this is jumping over a Piece here
    return true;
}

fn is_knight_move_valid(knight: Piece, movement: Move) -> bool {
    if knight.captured {
        return false;
    }
    if movement.end.x >= BOARD_DIMENSIONS as i8 || movement.end.y >= BOARD_DIMENSIONS as i8 {
        //This checks if the piece is trying to move off the board
        return false;
    }
    let x = movement.start.x - movement.end.x;
    let y = movement.start.y - movement.end.y;
    let one_norm = x.abs() + y.abs();
    let two_norm_square = x*x + y*y;
    //If the one norm is 3, then moving three spaces
    //the two norm being sqrt(5) means x^2 + y*2 = 5 has solutions
    //x= +/-1, +/-2
    //y= +/-1, +/-2
    //Which are all valid moves
    if one_norm == 3 && two_norm_square == 5 {
        return true;
    } else {
        return false;
    }
}

fn is_queen_move_valid(queen: Piece, movement: Move) -> bool {
    if queen.captured {
        return false;
    }
    if movement.end.x >= BOARD_DIMENSIONS as i8 || movement.end.y >= BOARD_DIMENSIONS as i8 {
        //This checks if the piece is trying to move off the board
        return false;
    }
    let x = movement.end.x - movement.start.x;
    let y = movement.end.y - movement.start.y;
    // A Queen can move as a rook or as a bishop
    // We section them off this way so that we can more easily implement checking for piece
    // jumping
    if x != 0 && y == 0 {
        // TODO check for piece jumping along x axis
        return true;
    } else if x == 0 && y != 0 {
        // TODO check for piece jumping along y axis
        return true;
    }
    // Now we check as if the Queen is a bishop
    // Check along y = x with origin at movement.start
    if x == y {
        // Check for jumping a piece
        return true;
    } else if x == -y {
        // Check for jumping a piece
        return true;
    }
    //Now we are not moving like a bishop nor a rook so we fail
    return false;
}

fn is_king_move_valid(king: Piece, movement: Move) -> bool {
    
    if king.captured {
        return false;
    }

    if movement.end.x >= BOARD_DIMENSIONS as i8 || movement.end.y >= BOARD_DIMENSIONS as i8 {
        //This checks if the piece is trying to move off the board
        return false;
    }

    let x = movement.end.x - movement.start.x;
    let y = movement.end.y - movement.start.y;
    // TODO: Check for a castling move
    if x > 1 || y > 1 {
        return false;
    }
    // TODO: Implement checking for "check" maybe as a separate function?
    return true;
}

fn empty_piece_move(_empty: Piece, _empty_move: Move) -> bool {
    return false;
}

impl Game {
    /// Create a new game
    pub fn new() -> Game {
        let mut new_game = Game {
            turn: Color::White,
            board: Board {
                // Initializes the state of the board as "empty" pieces to be updated during the
                // next step
                state: [[ Piece {
                    board_rep: '_', captured: true, first_move: false,
                    color: Color::Empty, position: Position {x: -1, y: -1}, 
                    is_move_valid: empty_piece_move
                }; BOARD_DIMENSIONS]; BOARD_DIMENSIONS],
            },
        };
        // Initialize board state
        // White pawns
        // NOTE: White on top, Black on bottom
        for file in 0..BOARD_DIMENSIONS {
            // creates a position the pawn is going to be (not needed for drawing but needed for
            // move creation later. Be careful to not desync these. A smarter implementation is
            // also possible
            let temp_pos = Position {x: 1, y: file as i8};
            let temp_pawn = Piece { board_rep: 'P', captured: false, first_move: true, color: Color::White, position: temp_pos, is_move_valid: is_pawn_move_valid };
            new_game.board.state[1][file] = temp_pawn;
        }
        // Here, we are not going to create the Piece and position on separate lines and will
        // follow the creation flow on new_game.state initialization
        new_game.board.state[0][0] = Piece {
            board_rep: 'R', captured: false, first_move: true,
                    color: Color::White, position: Position {x: 0, y: 0},
            is_move_valid: is_rook_move_valid
        };
        new_game.board.state[0][1] = Piece {
            board_rep: 'N',
            captured: false,
            first_move: true,
            color: Color::White,
            position: Position { x: 0, y: 1 },
            is_move_valid: is_knight_move_valid,
        };
        new_game.board.state[0][2] = Piece {
            board_rep: 'B',
            captured: false,
            first_move: true,
            color: Color::White,
            position: Position { x: 0, y: 2 },
            is_move_valid: is_rook_move_valid,
        };
        new_game.board.state[0][3] = Piece {
            board_rep: 'Q',
            captured: false,
            first_move: true,
            color: Color::White,
            position: Position { x: 0, y: 3 },
            is_move_valid: is_queen_move_valid,
        };
        new_game.board.state[0][4] = Piece {
            board_rep: 'K',
            captured: false,
            first_move: true,
            color: Color::White,
            position: Position { x: 0, y: 4 },
            is_move_valid: is_king_move_valid,
        };
        new_game.board.state[0][5] = Piece {
            board_rep: 'B',
            captured: false,
            first_move: true,
            color: Color::White,
            position: Position { x: 0, y: 5 },
            is_move_valid: is_bishop_move_valid,
        };
        new_game.board.state[0][6] = Piece {
            board_rep: 'N',
            captured: false,
            first_move: true,
            color: Color::White,
            position: Position { x: 0, y: 6 },
            is_move_valid: is_knight_move_valid,
        };
        new_game.board.state[0][7] = Piece {
            board_rep: 'R',
            captured: false,
            first_move: true,
            color: Color::White,
            position: Position { x: 0, y: 7 },
            is_move_valid: is_rook_move_valid,
        };

        for file in 0..BOARD_DIMENSIONS {
            // creates a position the pawn is going to be (not needed for drawing but needed for
            // move creation later. Be careful to not desync these. A smarter implementation is
            // also possible
            let temp_pos = Position { x: 6, y: file as i8 };
            let temp_pawn = Piece { board_rep: 'p',
                captured: false,
                first_move: true,
                color: Color::Black,
                position: temp_pos,
                is_move_valid: is_pawn_move_valid
            };
            new_game.board.state[6][file] = temp_pawn;
        }
        // Here, we are not going to create the Piece and position on separate lines and will
        // follow the creation flow on new_game.state initialization
        new_game.board.state[7][0] = Piece {
            board_rep: 'r', captured: false, first_move: true,
                    color: Color::Black, position: Position {x: 0, y: 0},
            is_move_valid: is_rook_move_valid
        };
        new_game.board.state[7][1] = Piece {
            board_rep: 'n',
            captured: false,
            first_move: true,
            color: Color::Black,
            position: Position { x: 0, y: 1 },
            is_move_valid: is_knight_move_valid,
        };
        new_game.board.state[7][2] = Piece {
            board_rep: 'b',
            captured: false,
            first_move: true,
            color: Color::Black,
            position: Position { x: 0, y: 2 },
            is_move_valid: is_rook_move_valid,
        };
        new_game.board.state[7][3] = Piece {
            board_rep: 'q',
            captured: false,
            first_move: true,
            color: Color::Black,
            position: Position { x: 0, y: 3 },
            is_move_valid: is_queen_move_valid,
        };
        new_game.board.state[7][4] = Piece {
            board_rep: 'k',
            captured: false,
            first_move: true,
            color: Color::Black,
            position: Position { x: 0, y: 4 },
            is_move_valid: is_king_move_valid,
        };
        new_game.board.state[7][5] = Piece {
            board_rep: 'b',
            captured: false,
            first_move: true,
            color: Color::Black,
            position: Position { x: 0, y: 5 },
            is_move_valid: is_bishop_move_valid,
        };
        new_game.board.state[7][6] = Piece {
            board_rep: 'n',
            captured: false,
            first_move: true,
            color: Color::Black,
            position: Position { x: 0, y: 6 },
            is_move_valid: is_knight_move_valid,
        };
        new_game.board.state[7][7] = Piece {
            board_rep: 'r',
            captured: false,
            first_move: true,
            color: Color::Black,
            position: Position { x: 0, y: 7 },
            is_move_valid: is_rook_move_valid,
        };

        return new_game;
    }

    /// Main game loop
    pub fn run_game(&mut self) {
        let game_over: bool = false;
        while !game_over {
            self.print_board();
            println!();

            // Get input for the current user
            // |-> somehow call engine to make the move
            let mut user_input = String::new();
            match self.turn {
                Color::White => print!("White move: "),
                Color::Black => print!("Black move: "),
                Color::Empty => print!("Should never happen, file a bug report for \"Empty turn\"")
            };

            // Flush the input here because, for reasons i'm not entirely sure of,
            // not flushing = printing and taking user input in the wrong order
            io::stdout()
                .flush()
                .expect("Could not read input.");
            io::stdin()
                .read_line(&mut user_input)
                .expect("Couldn't read input.");

            // Parse user input and translate it to a movement
            // |-> Helper function to check legality of parsed move
            self.parse_move(&user_input);

            // Make user move and update board state and turn, check for check / mate

            match self.turn {
                Color::White => self.turn = Color::Black,
                Color::Black => self.turn = Color::White,
                Color::Empty => self.turn = Color::Empty,
            }
        }
    }
    
    /// Parse a PGN move aka: Algebraic notation
    fn parse_move(&self, _user_move_string: &String) {
      // TODO
  }


    /// Find the legal moves for a given piece
    fn find_legal_moves(&self, piece: Piece) {
        // TODO
    }

    /// Print the game board to the console
    pub fn print_board(&self) {
        for rank in 0..BOARD_DIMENSIONS {
            for file in 0..BOARD_DIMENSIONS {
                print!("{}", self.board.state[rank][file].board_rep)
            }
            println!();
        }
    }
}

