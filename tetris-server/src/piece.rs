use crate::coordinate::Coordinate;
use crate::game_board::GameBoard;
use crate::player::PieceOutput;

#[derive(Clone, Copy)]
pub struct Piece {
    x: i32,
    y: i32,
    piece_type: PieceType,
    offsets_index: i32,
}

impl Piece {

    pub fn new(x: i32, y: i32, piece_type: PieceType) -> Piece {
        Piece {
            x,
            y,
            piece_type,
            offsets_index: 0,
        }
    }

    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        let mut coordinates = vec![];
        // todo Add all coordinates per offsets supplied via piece type
        coordinates.push(Coordinate{
            x: self.x as usize,
            y: self.y as usize
        });

        coordinates
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn attempt_fall_down(&mut self, board: &mut GameBoard) -> bool {
        let original_y = self.y;
        self.y -= 1;
        if !board.coordinates_are_valid(self.get_coordinates()) {
            self.y = original_y;
            board.place_piece(self);
            return false;
        }
        true
    }

    pub fn attempt_move(&mut self, board: &GameBoard, direction: &Direction) -> bool {
        let original_x = self.x;
        match direction {
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::UP => self.flip(direction),
            Direction::DOWN => self.flip(direction),
        }
        if !board.coordinates_are_valid(self.get_coordinates()) {
            self.x = original_x;
            return false;
        }
        true
    }

    fn flip(&mut self, direction: &Direction) {
        match direction {
            Direction::UP=> self.offsets_index = (self.offsets_index + 1) % 4,
            Direction::DOWN => self.offsets_index = (self.offsets_index - 1 + 4) % 4,
            _ => std::println!("FUCK THE FLIP IS WRONG"),
        }
    }

    pub fn get_piece_output(&self) -> PieceOutput {
        //todo move piece coordinates into PieceOutput object
        PieceOutput {
            color: "".to_string(),
            positions: vec![]
        }
    }
}

#[derive(Clone, Copy)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    EMPTY,
}

#[derive(Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
