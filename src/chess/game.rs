use std::time::Duration;

use super::engines::Engine;
use super::engines::eval::{PositionEval, MoveEval};
use super::position::{Position, STARTING_FEN};
use super::position::GameResult;
use super::types::{WHITE, BLACK};
use super::types::{get_from, get_to, SQUARE_BB};

pub struct Game {
    position: Position,
}

impl Game {
    pub fn new() -> Self {
        let mut p = Position::new();
        p.load_fen(STARTING_FEN).unwrap();

        Game { position: p }
    }

    pub fn start<
        EngineA: Engine,
        const DEPTH_A: usize,
        PositionEvalA: PositionEval,
        MoveEvalA: MoveEval,

        EngineB: Engine,
        const DEPTH_B: usize,
        PositionEvalB: PositionEval,
        MoveEvalB: MoveEval,
    >(mut self, delay: u64) -> GameResult {
        let mut engine_a = EngineA::new::<WHITE, BLACK>();
        let mut engine_b = EngineB::new::<BLACK, WHITE>();
        loop {
            let m;
            if self.position.turn == WHITE {
                m = engine_a.best_move::<
                    WHITE, BLACK, DEPTH_A,
                    PositionEvalA, MoveEvalA
                >(&mut self.position);

                if m == 0 { break; }

                self.position.make_move::<WHITE>(m);
            } else {
                m = engine_b.best_move::<
                    BLACK, WHITE, DEPTH_B,
                    PositionEvalB, MoveEvalB
                >(&mut self.position);

                if m == 0 { break; }

                self.position.make_move::<BLACK>(m);
            }

            if delay > 0 {
                let mut highlighted_squares = 0;
                highlighted_squares |= SQUARE_BB[get_from(m) as usize];
                highlighted_squares |= SQUARE_BB[get_to(m) as usize];

                print!("\x1B[2J\x1B[1;1H");
                self.position.print(highlighted_squares);
                std::thread::sleep(Duration::from_millis(delay));
            }
        }

        let result = self.position.result.unwrap();

        if delay > 0 {
            print!("\x1B[2J\x1B[1;1H");
            self.position.print(0);
            match result {
                GameResult::Checkmate(color) => if color == WHITE {
                    println!("WHITE won by checkmate");
                } else {
                    println!("BLACK won by checkmate");
                }
                GameResult::Stalemate => println!("Draw by stalemate"),
                GameResult::FiftyMoveRule => println!("Draw by fifty move rule"),
                GameResult::ThreefoldRepetition => println!("Draw by threefold repetition"),
                GameResult::InsufficientMaterial => println!("Draw by insufficient material"),
            }
        }

        result
    }
}
