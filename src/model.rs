/// This structure represents a chess board and assosciated state.
pub struct Board {
    state: [[Piece; 8]; 8],
}

impl Board {
    pub fn new_board() -> Board {
        let board: Board;

        // Init pieces
        for rank in board.state {
            for file in rank {
                file[1] = Piece::WhitePawn;
            }
        }

        return board;
    }
}

/// This enum represents the different types a game piece can be.
enum Piece {
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
