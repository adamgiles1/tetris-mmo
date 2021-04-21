use crate::player::{Player, BoardOutput, AllBoardOutput};
use crate::piece::{Piece, Direction};
use crate::piece::PieceType;
use crate::server::WebServer;
use crate::game_board::GameBoard;
use crate::connection::Connection;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref inputs: Mutex<HashMap<String, Vec<Direction>>> = Mutex::new(HashMap::new());
}

pub struct Game {
    players: Vec<Player>,
    turns: i32,
    speed: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: vec![],
            turns: 0,
            speed: 30,
        }
    }

    pub unsafe fn add_input(username: String, direction: Direction) {
        let mut locked_inputs = inputs.lock().unwrap();
        if locked_inputs.contains_key(&username) {
            let mut existingInputs = locked_inputs.remove_entry(&username).unwrap();
            existingInputs.1.push(direction);
            locked_inputs.insert(username, existingInputs.1);
        } else {
            locked_inputs.insert(username, vec![direction]);
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn start(&mut self) {
        println!("starting game");
        //start calling tick every 60th of a second to update game
        loop {

            self.tick();
            thread::sleep(Duration::from_millis(16));
        }
    }

    fn tick(&mut self) {
        for mut player in &mut self.players {
            let mut all_player_inputs = inputs.lock().unwrap();
            if all_player_inputs.contains_key(&player.username) {
                let player_inputs = all_player_inputs.remove_entry(&player.username).unwrap().1;

                for direction in player_inputs {
                    player.attempt_move(&direction);
                }
            }


            if self.turns % self.speed == 0 {
                let placed = player.attempt_fall_down();
                if !placed {
                    println!("piece placed");
                    player.set_piece(Piece::new_random());
                }
            }
        }

        let mut boardsOutput = AllBoardOutput {
            msgType: String::from("BOARD"),
            boards: vec![]
        };
        for player in &self.players {
            boardsOutput.boards.push(BoardOutput {
                playerId: String::from(&player.username),
                tiles: player.board.get_board_output(),
                piece: player.piece.get_piece_output(),
            });
        }

        for player in &self.players {
            player.send_message(serde_json::to_string(&boardsOutput).unwrap());
        }

        self.turns += 1;
    }
}
