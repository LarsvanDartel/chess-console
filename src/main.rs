#![allow(dead_code)]

mod chess;
use chess::engines::RandomEngine;
use chess::engines::eval::material::MaterialEval;
use chess::engines::pure_minimax::MiniMaxEngine;
use chess::game::Game;
use chess::position::GameResult;
use chess::types::{BLACK, WHITE};

fn main() {
    let mut n_wins = 0;
    let mut n_draws = 0;
    let mut n_losses = 0;

    for _ in 0..100 {
        let mut game = Game::new(
            Some(Box::new(MiniMaxEngine::<BLACK, WHITE, 4, MaterialEval>::new())),
            Some(Box::new(RandomEngine::<BLACK, WHITE>::new())),
        );
        match game.start(0) {
            GameResult::Checkmate(color) => {
                if color == WHITE {
                    n_wins += 1;
                } else {
                    n_losses += 1;
                }
            },
            _ => n_draws += 1
        }
    }

    println!("{:-5} wins", n_wins);
    println!("{:-5} draws", n_draws);
    println!("{:-5} losses", n_losses);
}