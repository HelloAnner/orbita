package com.anner.dfs;

/**
 * Sudoku is a puzzle where you're given a partially-filled 9 by 9 grid with
 * digits. The objective is to fill the grid with the constraint that every row,
 * column, and box (3 by 3 subgrid) must contain all of the digits from 1 to 9.
 * Implement an efficient sudoku solver
 * 
 * 2025-07-28
 * Anner
 */
public class Sudoku {

    public boolean sloveSudoku(char[][]board) {
        for(int i = 0 ; i < 9; i++) {
            for (int j = 0 ; j< 9;j++) {
                if(board[i][j] == '.' ) {
                    
                }
            }
        }
        return false;
    }

    private boolean isValid(char[][] board , int row , int col , char c) {
        return false;
    }

    public static void main(String[] args) {
        
    }
}
