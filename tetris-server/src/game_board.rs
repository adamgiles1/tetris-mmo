use crate::block::Block;
use crate::piece::Piece;
use crate::coordinate::Coordinate;
use crate::player::BoardOutput;

pub struct GameBoard {
    board: [[Block; 10]; 40],
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            board: [[Block::new(); 10]; 40],
        }
    }

    pub fn get_block_color(&self, x: usize, y: usize) -> String {
        //self.board[x][y].getStatus().to_string()
        String::from("test")
    }

    pub fn place_piece(&mut self, piece: &Piece) {
        let coordinates = piece.get_coordinates();
        for coordinate in coordinates {
            self.board[coordinate.x][coordinate.y] = Block::new_with_color(piece.get_piece_type().clone());
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
        //todo move board into vector of vectors
        let board = vec![vec![]];
    }

    pub fn update() {

    }
}
