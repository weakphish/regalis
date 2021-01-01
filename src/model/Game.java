package model;

import java.util.Scanner;

/**
 * This class represents a game and the associated actions and data
 */
public class Game {
    /**
     * The current turn - even = white, odd = black
     */
    private Turn turn;
    /**
     * The game board
     */
    private final Board board;

    /**
     * Constructor - creates a new game
     */
    public Game() {
        this.board = new Board();
        this.turn = Turn.WHITE;
    }

    /**
     * Run a game on a loop until it ends via resignation or mate
     */
    public void runGame() {
        Scanner userInput = new Scanner(System.in);

        // Main loop
        while (true) {
            this.printBoard();

            String move;
            // Check whose turn it is
            if (this.turn == Turn.WHITE) {
                System.out.print("White move: ");
            } else {
                System.out.print("Black move: ");
            }
            move = userInput.nextLine();

            // TODO: Parse turn here via function and check legal moves

            // Update turn
            if (this.turn == Turn.WHITE) {
                this.turn = Turn.BLACK;
            } else {
                this.turn = Turn.WHITE;
            }
        }
    }

    /**
     * Parse a PGN notation string to convert a user inputted move into a coordinate
     */
    private void parseMove(String move) {
        Scanner parser = new Scanner(move);
    }

    /**
     * Print the board - wrapper around the Board method printBoard()
     */
    public void printBoard() {
        this.board.printBoard();
    }

    /**
     * Find legal moves for the player who's turn it currently is
     */
    private void findLegalMoves() {
        // TODO
    }
}
