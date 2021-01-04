/// Representation of a Chess Board
pub struct Board {
    board: [[i32; 8]; 8],
}

impl Board {
    /// Constructor for the board struct
    pub fn new_board() -> Board {
        // Create board
        let board = Board {
            board: [[i32; 8]; i32],
        };
        // Fill in board
        for file in board[1] {
            file = Piece.BlackPawn;
        }

        return board;
    }
}

/// Different forms a piece can be
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

/// Current game turn
pub enum Turn {
    White,
    Black,
}
