package model;

import java.util.HashMap;

/**
 * This class represents a chess board with an enum type for pieces occupying squares. The board itself is modeled as
 * a 2D array of Pieces.
 */
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
     * Hashmap used to pair Pieces with a character for terminal representation
     */
    private final static HashMap<Piece, Character> PIECE_CHARS = new HashMap<>();
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

        // Init piece characters
        PIECE_CHARS.put(null, '0');
        PIECE_CHARS.put(Piece.WHITE_PAWN, 'p');
        PIECE_CHARS.put(Piece.WHITE_KNIGHT, 'n');
        PIECE_CHARS.put(Piece.WHITE_BISHOP, 'b');
        PIECE_CHARS.put(Piece.WHITE_ROOK, 'r');
        PIECE_CHARS.put(Piece.WHITE_QUEEN, 'q');
        PIECE_CHARS.put(Piece.WHITE_KING, 'k');
        PIECE_CHARS.put(Piece.BLACK_PAWN, 'P');
        PIECE_CHARS.put(Piece.BLACK_KNIGHT, 'N');
        PIECE_CHARS.put(Piece.BLACK_BISHOP, 'B');
        PIECE_CHARS.put(Piece.BLACK_ROOK, 'R');
        PIECE_CHARS.put(Piece.BLACK_QUEEN, 'Q');
        PIECE_CHARS.put(Piece.BLACK_KING, 'K');

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

    /**
     * Print the board to the console
     * // TODO print null chars, find way to represent black and white squares
     */
    public void printBoard() {
        for (int rank = 0; rank < BOARD_DIMENSIONS; rank++) {
            for (int file = 0; file < BOARD_DIMENSIONS; file++) {
                System.out.printf("%4c", Board.PIECE_CHARS.get(this.getBoard()[rank][file]));
            }
            System.out.println();
        }
    }
}
