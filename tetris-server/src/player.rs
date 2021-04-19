use crate::game_board::GameBoard;
use crate::piece::{Piece, Direction, PieceType};
use ws::{Sender, Error};
use crate::game::Game;
use crate::connection::Connection;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use crate::coordinate::Coordinate;

pub struct Player {
    pub username: String,
    pub board: GameBoard,
    pub piece: Piece,
    pub sender: Sender,
    pub inputs: Vec<Direction>,
}

impl Player {

    pub fn new(sender: Sender) -> Player {
        let id = sender.connection_id().to_string();

        let player = Player {
            username: id.clone(),
            board: GameBoard::new(),
            piece: Piece::new(5, 5, PieceType::L),
            sender,
            inputs: vec![],
        };

        let message = serde_json::to_string(&PlayerId {
            msgType: String::from("PLAYERID"),
            playerId: id,
        }).unwrap();
        player.sender.send(message);

        player
    }

    pub fn get_board(&mut self) -> &mut GameBoard {
        &mut self.board
    }

    pub fn set_board(&mut self, board: GameBoard) {
        self.board = board;
    }

    pub fn get_piece(&mut self) -> &mut Piece {
        &mut self.piece
    }

    pub fn set_piece(&mut self, piece: Piece) {
        self.piece = piece;
    }

    pub fn reset_inputs(&mut self) {
        self.inputs = vec![];
    }

    pub fn has_inputs(&self) -> bool {
        !self.inputs.is_empty()
    }

    pub fn get_inputs(&self) -> &Vec<Direction> {
        &self.inputs
    }

    pub fn attempt_move(&mut self, direction: &Direction) {

        match direction {
            Direction::LEFT | Direction::RIGHT | Direction::FLIP_RIGHT | Direction::FLIP_LEFT =>
                self.piece.attempt_move(&self.board, direction),
            Direction::SPACE => self.piece.drop(&mut self.board),
            Direction::NONE => (),
        };
    }

    pub fn attempt_fall_down(&mut self) -> bool {
        self.piece.attempt_fall_down(&mut self.board)
    }

    pub fn send_message(&self, message: String) {
        self.sender.send(message);
    }

    pub fn input_from(&self, str: &str) -> Direction {
        return match str {
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            "S" => Direction::SPACE,
            "Z" => Direction::FLIP_LEFT,
            "X" => Direction::FLIP_RIGHT,
            _ => Direction::NONE,
        }
    }
}

impl ws::Handler for Player {
    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        println!("Player connected");
        Ok(())
    }

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        let value = message.as_text()?;
        let parsed: MessageType = serde_json::from_str(value).unwrap_or(MessageType { msgType: String::from("FAILURE"),});
        match &parsed.msgType[..] {
            "INPUT" => unsafe {
                let parsed: PlayerInput = serde_json::from_str(value).unwrap();
                let action = self.input_from(&parsed.action);
                println!("action is: {} id is: {}", parsed.action, self.username);
                Game::add_input(self.username.clone(), action);
            }
            _ => (),
        }
        Ok(())
    }
}

// All of the JSON models are below

#[derive(Serialize, Deserialize)]
pub struct MessageType {
    pub msgType: String
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInput {
    pub action: String
}

#[derive(Serialize, Deserialize)]
pub struct AllBoardOutput {
    pub msgType: String,
    pub boards: Vec<BoardOutput>,
}

#[derive(Serialize, Deserialize)]
pub struct BoardOutput {
    pub playerId: String,
    pub tiles: Vec<Vec<String>>,
    pub piece: PieceOutput,
}

#[derive(Serialize, Deserialize)]
pub struct PieceOutput {
    pub color: String,
    pub positions: Vec<Coordinate>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerId {
    pub msgType: String,
    pub playerId: String,
}
