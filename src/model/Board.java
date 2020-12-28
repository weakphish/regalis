package model;

import java.util.HashMap;

public class Board {
    /**
     * Dimensions of the square board
     */
    public final static int BOARD_DIMENSIONS = 8;
    /**
     * Hashmap that represents the key-value pairs of the file letter-to-index
     */
    private final static HashMap<String, Integer> FILE_PAIRS = new HashMap<>();
    /**
     * A 2D integer array that represents the board state in memory
     */
    private final Piece[][] board;

    /**
     * Constructor for the board
     */
    public Board() {
        // Init the Rank hashmap
        FILE_PAIRS.put("a", 0);
        FILE_PAIRS.put("b", 1);
        FILE_PAIRS.put("c", 2);
        FILE_PAIRS.put("d", 3);
        FILE_PAIRS.put("e", 4);
        FILE_PAIRS.put("f", 5);
        FILE_PAIRS.put("g", 6);
        FILE_PAIRS.put("h", 7);

        // TODO Init the board
        // Pawns
        this.board = new Piece[BOARD_DIMENSIONS][BOARD_DIMENSIONS];
        for (int file = 0; file < BOARD_DIMENSIONS; file++) {
            this.board[1][file] = Piece.WHITE_PAWN;
        }
        for (int file = 0; file < BOARD_DIMENSIONS; file++) {
            this.board[6][file] = Piece.BLACK_PAWN;
        }
        // White
        this.board[0][FILE_PAIRS.get("a")] = Piece.WHITE_ROOK;
        this.board[0][FILE_PAIRS.get("h")] = Piece.WHITE_ROOK;
        this.board[0][FILE_PAIRS.get("b")] = Piece.WHITE_KNIGHT;
        this.board[0][FILE_PAIRS.get("g")] = Piece.WHITE_KNIGHT;
        this.board[0][FILE_PAIRS.get("c")] = Piece.WHITE_BISHOP;
        this.board[0][FILE_PAIRS.get("f")] = Piece.WHITE_BISHOP;
        this.board[0][FILE_PAIRS.get("d")] = Piece.WHITE_QUEEN;
        this.board[0][FILE_PAIRS.get("e")] = Piece.WHITE_KING;
        // BlacPiece.k
        this.board[7][FILE_PAIRS.get("a")] = Piece.BLACK_ROOK;
        this.board[7][FILE_PAIRS.get("h")] = Piece.BLACK_ROOK;
        this.board[7][FILE_PAIRS.get("b")] = Piece.BLACK_KNIGHT;
        this.board[7][FILE_PAIRS.get("g")] = Piece.BLACK_KNIGHT;
        this.board[7][FILE_PAIRS.get("c")] = Piece.BLACK_BISHOP;
        this.board[7][FILE_PAIRS.get("f")] = Piece.BLACK_BISHOP;
        this.board[7][FILE_PAIRS.get("d")] = Piece.BLACK_QUEEN;
        this.board[7][FILE_PAIRS.get("e")] = Piece.BLACK_KING;
    }

    /**
     * Get the board
     *
     * @return the board array
     */
    public Piece[][] getBoard() {
        return this.board;
    }
}
