#![allow(dead_code)]

mod chess;
use chess::*;

fn main() {
    let mut game = Game::new();
    game.start();
}