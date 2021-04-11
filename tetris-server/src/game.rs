use crate::player::{Player, BoardOutput, AllBoardOutput};
use crate::piece::{Piece, Direction};
use crate::piece::PieceType;
use crate::server::WebServer;
use crate::game_board::GameBoard;
use crate::connection::Connection;
use std::time::Duration;
use std::thread;

pub struct Game {
    players: Vec<Player>,
    turns: i32,
    speed: i32,
    start_at: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: vec![],
            turns: 0,
            speed: 30,
            start_at: 1,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
        if self.players.len() >= self.start_at as usize {
            self.start();
            println!("game started");
        }
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
            let player_has_inputs = player.has_inputs();
            if player_has_inputs {
                let player_inputs = player.get_inputs().clone();
                for direction in player_inputs {
                    player.attempt_move(&direction);
                }
            }

            if self.turns % self.speed == 0 {
                let placed = player.attempt_fall_down();
                if placed {
                    player.set_piece(Piece::new(0, 0, PieceType::O));
                }
            }
        }

        for player in &self.players {
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
            player.send_message(serde_json::to_string(&boardsOutput).unwrap());
        }

        println!("Tick Processed: {}", self.turns);
        self.turns += 1;
    }
}
