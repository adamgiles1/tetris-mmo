use crate::block::Block;
use crate::piece::{Piece, PieceType};
use crate::coordinate::Coordinate;
use crate::player::BoardOutput;
use std::collections::HashSet;
use std::ops::Range;

pub struct GameBoard {
    board: [[Block; 40]; 10],
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            board: [[Block::new(); 40]; 10],
        }
    }

    pub fn get_block_color(&self, x: usize, y: usize) -> String {
        self.board[x][y].get_color()
    }

    pub fn place_piece(&mut self, piece: &Piece) {
        let coordinates = piece.get_coordinates();
        for coordinate in coordinates {
            self.board[coordinate.x][coordinate.y] = Block::new_with_color(piece.get_piece_type().clone());
        }

        self.check_lines_for_clear(0..22);
    }

    fn check_lines_for_clear(&mut self, rows_to_check: Range<usize>) {
        let mut clear_row;
        for y in rows_to_check {
            clear_row = true;
            while clear_row {
                for x in 0..10 {
                    if self.board[x][y].is_empty() {
                        clear_row = false;
                    }
                }

                if clear_row {
                    self.clear_line(y);
                }
            }
        }
    }

    fn clear_line(&mut self, line: usize) {
        println!("clearing line");
        // Shift all higher lines down by one
        for y in line..39 {
            for x in 0..10 {
                self.board[x][y] = self.board[x][y+1];
            }
        }

        for x in 0..10 {
            self.board[x][39] = Block::new();
        }
    }

    pub fn coordinates_are_valid(&self, coordinates: Vec<Coordinate>) -> bool {
        for coordinate in coordinates {
            let x = coordinate.x;
            let y = coordinate.y;
            if x >= self.board.len() || y >= self.board[0].len() {
                return false;
            }
            if !self.board[x][y].is_empty() {
                return false;
            }
        }

        true
    }

    pub fn get_board_output(&self) -> Vec<Vec<String>> {
        let mut board: Vec<Vec<String>> = vec![vec![String::from('#'); 40]; 10];

        for (x, row) in self.board.iter().enumerate() {
            for (y, block) in row.iter().enumerate() {
                board[x][y] = self.get_block_color(x, y);
            }
        }

        board
    }

    pub fn update() {

    }
}
