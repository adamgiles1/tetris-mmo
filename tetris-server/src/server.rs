use crate::player::Player;
use crate::game_board::GameBoard;
use crate::piece::{Piece, PieceType};
use serde_json;
use crate::game::Game;
use ws::listen;
use std::thread;

pub struct WebServer {
    pub(crate) game: Game,
}

impl WebServer {
    // pub fn start_server(&mut self) {
    //
    // }
}

