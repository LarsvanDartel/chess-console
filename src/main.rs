#![allow(dead_code)]

mod chess;
use chess::game::Game;
use chess::types::{BLACK, WHITE};
use chess::engines::RandomEngine;

fn main() {
    let mut game = Game::new(
        None,
        Some(Box::new(RandomEngine::<BLACK, WHITE>::new()))
    );
    game.start(0);
}