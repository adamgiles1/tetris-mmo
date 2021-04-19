mod server;
mod player;
mod game_board;
mod block;
mod game;
mod piece;
mod coordinate;
mod BoardState;
mod connection;
mod offset;

use crate::player::Player;

use ws::{listen, Sender};
use crate::game::Game;
use crate::connection::Connection;
use std::{io, thread};
use crate::server::WebServer;
use std::sync::Arc;

fn main() {
    println!("Starting server...");
    let player_count = 1;

    let mut connections = vec![];

    listen("127.0.0.1:6868", |out| {
        let cloned = out.clone();
        let connection = Player::new(out);
        connections.push(cloned);
        if connections.len() >= player_count {
            let cloned_connections = connections.clone();
            thread::spawn(|| {
                let mut game = Game::new();

                for connection in cloned_connections {
                    game.add_player(Player::new(connection));
                }

                game.start();
            });
            connections.clear();
        }
        connection
    }).unwrap();
}
