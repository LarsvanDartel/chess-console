#![allow(dead_code)]

mod chess;

use std::time::Instant;

use chess::engines::RandomEngine;
use chess::engines::OrderingEngine;
use chess::engines::eval::MaterialEval;
use chess::engines::eval::SimpleMoveEval;
use chess::game::Game;
use chess::position::GameResult;
use chess::types::WHITE;

fn main() {
    const N_GAMES: usize = 100;

    let mut n_wins = 0;
    let mut n_draws = 0;
    let mut n_losses = 0;

    let now = Instant::now();
    for _ in 0..N_GAMES {
        let game = Game::new();
        match game.start::<
            OrderingEngine,
            2,
            MaterialEval,
            SimpleMoveEval,

            RandomEngine,
            0,
            (),
            ()
        >(0) {
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
    let elapsed = now.elapsed();

    println!("{} game(s) played in {}ms", N_GAMES, elapsed.as_millis());
    println!("{:-5} wins", n_wins);
    println!("{:-5} draws", n_draws);
    println!("{:-5} losses", n_losses);
}