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
        FILE_PAIRS.put("h", 6);

        // TODO Init the board
        this.board = new int[BOARD_DIMENSIONS][BOARD_DIMENSIONS];
        for (int i = 0; i < BOARD_DIMENSIONS; i++) {
            this.board[6][i] = WHITE_PAWN;
        }
        for (int i = 0; i < BOARD_DIMENSIONS; i++) {
            this.board[1][i] = BLACK_PAWN;
        }
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
