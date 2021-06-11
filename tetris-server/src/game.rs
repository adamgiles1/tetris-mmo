use crate::player::{Player, BoardOutput, AllBoardOutput, PlayerIds};
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
use rand::Rng;

lazy_static! {
    static ref inputs: Mutex<HashMap<String, Vec<Direction>>> = Mutex::new(HashMap::new());
}

pub struct Game {
    players: Vec<Player>,
    turns: i32,
    speed: i32,
    till_next_fall: i32,
    swap_boards: bool,
    till_next_swap: i32,
}

impl Game {
    pub fn new(swap_boards: bool) -> Game {
        Game {
            players: vec![],
            turns: 0,
            speed: 30,
            till_next_fall: 30,
            swap_boards,
            till_next_swap: 10,
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

        // Give player information so clients can initialize the boards
        self.send_game_init_message();
        thread::sleep(Duration::from_millis(3000));

        // start calling tick every 60th of a second to update game
        loop {
            self.tick();
            thread::sleep(Duration::from_millis(16));
        }
    }

    fn send_game_init_message(&self) {
        let mut all_player_ids= vec![];
        for player in &self.players {
            all_player_ids.push(player.username.clone());
        }

        for player in &self.players {
            let message = serde_json::to_string(&PlayerIds {
                msgType: String::from("GAME_START"),
                playerId: player.username.clone(),
                allIds: all_player_ids.clone(),
            }).unwrap();
            player.send_message(message);
        }
    }

    fn tick(&mut self) {
        for mut player in &mut self.players {
            if !player.game_ended {
                let mut all_player_inputs = inputs.lock().unwrap();
                if all_player_inputs.contains_key(&player.username) {
                    let player_inputs = all_player_inputs.remove_entry(&player.username).unwrap().1;

                    for direction in player_inputs {
                        player.attempt_move(&direction);
                    }
                }

                if self.till_next_fall <= 0 {
                    let placed = player.attempt_fall_down();
                    if !placed {
                        println!("piece placed");
                        player.set_piece(Piece::new_random());
                    }
                }
            }
        }

        if self.till_next_fall <= 0 {
            self.till_next_fall = self.speed;
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
                gameEnded: player.game_ended,
            });
        }

        for player in &self.players {
            player.send_message(serde_json::to_string(&boardsOutput).unwrap());
        }

        // every 20 seconds increase the speed
        if self.turns % (20*60) == 0 && self.speed > 2 {
            self.speed -= 4;
        }

        // every 1-20 seconds swap boards
        if self.swap_boards && self.till_next_swap <= 0 {
            self.cycle_boards();
            self.till_next_swap = rand::thread_rng().gen_range(1..21) * 60;
        }

        self.turns += 1;
        self.till_next_fall -= 1;
        self.till_next_swap -= 1;
    }

    fn cycle_boards(&mut self) {

        let mut boards = vec![];
        let mut pieces = vec![];

        for mut player in &mut self.players {
            if !player.game_ended {
                boards.push(player.get_board().clone());
                pieces.push(player.get_piece().clone());
            }
        }

        let first_item = boards.remove(0);
        boards.push(first_item);
        let first_item = pieces.remove(0);
        pieces.push(first_item);

        for mut player in &mut self.players {
            if !player.game_ended {
                player.set_board(boards.remove(0));
                player.set_piece(pieces.remove(0));
            }
        }
    }
}
