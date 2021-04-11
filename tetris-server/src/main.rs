mod server;
mod player;
mod game_board;
mod block;
mod game;
mod piece;
mod coordinate;
mod BoardState;
mod connection;

use crate::player::Player;

use ws::{listen, Sender};
use crate::game::Game;
use crate::connection::Connection;
use std::{io, thread};
use crate::server::WebServer;
use std::sync::Arc;

fn main() {
    println!("Starting server...");

    //let mut game = Game::new();
    let mut connections = vec![];

    listen("127.0.0.1:3012", |out| {
        let cloned = out.clone();
        let connection = Player::new(out);
        connections.push(cloned);
        if connections.len() >= 1 {
            let cloned_connections = connections.clone();
            thread::spawn(|| {
                let mut game = Game::new();

                for connection in cloned_connections {
                    game.add_player(Player::new(connection));
                }

                game.start();
            });
        }
        connection
    }).unwrap();
}
