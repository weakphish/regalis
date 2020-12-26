package view;

import model.Board;

public class Regalis {
    public static void main(String[] args) {
        // Debugging...
        Board board = new Board();
        int[][] boardArray = board.getBoard();
        for (int rank = 0; rank < Board.BOARD_DIMENSIONS; rank++) {
            for (int file = 0; file < Board.BOARD_DIMENSIONS; file++) {
                System.out.format("%4d", boardArray[rank][file]);
            }
            System.out.println();
        }
    }
}
