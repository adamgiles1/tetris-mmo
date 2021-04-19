use crate::piece::{PieceType, Piece};

#[derive(Copy, Clone)]
pub struct Block {
    color: PieceType,
}

impl Block {
    pub fn new() -> Block {
        Block {
            color: PieceType::EMPTY
        }
    }

    pub fn new_with_color(color: PieceType) -> Block {
        Block {
            color
        }
    }

    pub fn get_color(self) -> String {
        self.color.get_color()
    }

    pub fn is_empty(&self) -> bool {
        match &self.color {
            PieceType::EMPTY => true,
            _ => false,
        }
    }

    pub fn to_string() -> String {
        String::from("L") // todo actually return character based on block type
    }
}
