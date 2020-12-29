package model;

/**
 * This class represents a game and the associated actions and data
 */
public class Game {
    /**
     * The current turn - even = white, odd = black
     */
    private final int turn;
    /**
     * The game board
     */
    private final Board board;

    /**
     * Constructor - creates a new game
     */
    public Game() {
        this.board = new Board();
        this.turn = 0;
    }

    /**
     * Print the board - wrapper around the Board method printBoard()
     */
    public void printBoard() {
        this.board.printBoard();
    }
}
