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
     * Hashmap that represents the key-value pairs of the ranks letter-to-index
     */
    private final static HashMap<String, Integer> RANK_PAIRS = new HashMap<>();
    /**
     * Dimensions of the square board
     */
    private final static int BOARD_DIMENSIONS = 8;
    /**
     * A 2D integer array that represents the board state in memory
     */
    private final int[][] board;

    /**
     * Constructor for the board
     */
    public Board() {
        // Init the Rank hashmap
        RANK_PAIRS.put("a", 0);
        RANK_PAIRS.put("b", 1);
        RANK_PAIRS.put("c", 2);
        RANK_PAIRS.put("d", 3);
        RANK_PAIRS.put("e", 4);
        RANK_PAIRS.put("f", 5);
        RANK_PAIRS.put("g", 6);
        RANK_PAIRS.put("h", 6);

        // Init the board
        this.board = new int[BOARD_DIMENSIONS][BOARD_DIMENSIONS];
    }
}
