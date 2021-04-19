use crate::coordinate::Coordinate;
use crate::game_board::GameBoard;
use crate::player::PieceOutput;
use rand::Rng;
use crate::offset::Offset;
use crate::BoardState::BoardState;

#[derive(Clone, Copy)]
pub struct Piece {
    x: i32,
    y: i32,
    piece_type: PieceType,
    offsets_index: usize,
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

        let offsets = self.piece_type.get_offsets(self.offsets_index);
        
        for offset in offsets.iter() {
            coordinates.push(Coordinate {
                x: (self.x + offset.x) as usize,
                y: (self.y + offset.y) as usize
            });
        }

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

    pub fn attempt_move(&mut self, board: &GameBoard, direction: &Direction) {
        let original_x = self.x;
        match direction {
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::FLIP_RIGHT => self.flip(direction, board),
            Direction::FLIP_LEFT => self.flip(direction, board),
            _ => {}
        }
        if !board.coordinates_are_valid(self.get_coordinates()) {
            self.x = original_x;
        }
    }

    pub fn drop(&mut self, board: &mut GameBoard) {
        while self.attempt_fall_down(board) {}
    }

    fn flip(&mut self, direction: &Direction, board: &GameBoard) {
        let og_index = self.offsets_index;
        match direction {
            Direction::FLIP_RIGHT => self.offsets_index = (self.offsets_index + 1) % 4,
            Direction::FLIP_LEFT => self.offsets_index = (self.offsets_index + 3) % 4,
            _ => std::println!("FUCK THE FLIP IS WRONG"),
        }
        if !board.coordinates_are_valid(self.get_coordinates()) {
            self.offsets_index = og_index;
        }
    }

    pub fn get_piece_output(&self) -> PieceOutput {
        PieceOutput {
            color: self.piece_type.get_color(),
            positions: self.get_coordinates(),
        }
    }
}


const fn c(x: i32, y: i32) -> Offset {
    Offset {x, y}
}

const T_OFFSET_LIST: [[Offset; 4]; 4] = [
    [c(0,0),c(-1,0),c(1,0),c(0,1)],
    [c(0,0),c(0,1),c(1,0),c(0,-1)],
    [c(0,0),c(1,0),c(-1,0),c(0,-1)],
    [c(0,0),c(-1,0),c(0,1),c(0,-1)]];

// Offsets are stored at bottom of file in a horribly stupid way
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

impl PieceType {

    pub fn get_color(&self) -> String {
        match self {
            PieceType::I => String::from("I"),
            PieceType::O => String::from("O"),
            PieceType::T => String::from("T"),
            PieceType::S => String::from("S"),
            PieceType::Z => String::from("Z"),
            PieceType::J => String::from("J"),
            PieceType::L => String::from("L"),
            PieceType::EMPTY => String::from("B")
        }
    }

    pub fn get_offsets(&self, index: usize) -> [Offset; 4] {
        // let mut offsets = [Coordinate; 4];
        // offsets[0] = Offset{x: 0, y: 0};
        // offsets[1] = Offset{x: 1, y: 1};
        // offsets[2] = Offset{x: 1, y: 0};
        // offsets[3] = Offset{x: 0, y: 1};
        // return offsets;

        return T_OFFSET_LIST[index].clone()
    }

    pub fn random() -> PieceType {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..8) {
            1 => PieceType::I,
            2 => PieceType::O,
            3 => PieceType::T,
            4 => PieceType::S,
            5 => PieceType::Z,
            6 => PieceType::J,
            7 => PieceType::L,
            _ => PieceType::I,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    FLIP_RIGHT,
    FLIP_LEFT,
    SPACE,
    NONE,
}
