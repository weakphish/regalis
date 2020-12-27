package model;

import java.util.HashMap;

public class Board {
    /* Key-values for piece values */
    private final static int EMPTY_SQUARE = 0;
    private final static int WHITE_PAWN = 1;
    private final static int BLACK_PAWN = -1;
    private final static int WHITE_KNIGHT = 2;
    private final static int BLACK_KNIGHT = -2;
    private final static int WHITE_BISHOP = 3;
    private final static int BLACK_BISHOP = -3;
    private final static int WHITE_ROOK = 4;
    private final static int BLACK_ROOK = -4;
    private final static int WHITE_QUEEN = 5;
    private final static int BLACK_QUEEN = -5;
    private final static int WHITE_KING = 6;
    private final static int BLACK_KING = -6;

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
    private final int[][] board;

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
        this.board = new int[BOARD_DIMENSIONS][BOARD_DIMENSIONS];
        for (int file = 0; file < BOARD_DIMENSIONS; file++) {
            this.board[1][file] = WHITE_PAWN;
        }
        for (int file = 0; file < BOARD_DIMENSIONS; file++) {
            this.board[6][file] = BLACK_PAWN;
        }
        // White
        this.board[0][FILE_PAIRS.get("a")] = WHITE_ROOK;
        this.board[0][FILE_PAIRS.get("h")] = WHITE_ROOK;
        this.board[0][FILE_PAIRS.get("b")] = WHITE_KNIGHT;
        this.board[0][FILE_PAIRS.get("g")] = WHITE_KNIGHT;
        this.board[0][FILE_PAIRS.get("c")] = WHITE_BISHOP;
        this.board[0][FILE_PAIRS.get("f")] = WHITE_BISHOP;
        this.board[0][FILE_PAIRS.get("d")] = WHITE_QUEEN;
        this.board[0][FILE_PAIRS.get("e")] = WHITE_KING;
        // Black
        this.board[7][FILE_PAIRS.get("a")] = BLACK_ROOK;
        this.board[7][FILE_PAIRS.get("h")] = BLACK_ROOK;
        this.board[7][FILE_PAIRS.get("b")] = BLACK_KNIGHT;
        this.board[7][FILE_PAIRS.get("g")] = BLACK_KNIGHT;
        this.board[7][FILE_PAIRS.get("c")] = BLACK_BISHOP;
        this.board[7][FILE_PAIRS.get("f")] = BLACK_BISHOP;
        this.board[7][FILE_PAIRS.get("d")] = BLACK_QUEEN;
        this.board[7][FILE_PAIRS.get("e")] = BLACK_KING;
    }

    /**
     * Get the board
     *
     * @return the board array
     */
    public int[][] getBoard() {
        return this.board;
    }
}
