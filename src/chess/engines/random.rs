use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;

use super::Engine;
use super::PRNG;
use super::eval::MoveEval;
use super::eval::PositionEval;


pub struct RandomEngine {
    rng: PRNG
}

impl Engine for RandomEngine {
    fn new<const US: Color, const THEM: Color>() -> Self {
        RandomEngine { rng: PRNG::new() }
    }

    fn best_move<
            const US: Color, const THEM: Color, const DEPTH: usize,
            PE: PositionEval, ME: MoveEval
        >(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);
        let moves = p.generate_moves::<US, THEM>();

        if moves.size == 0 { return 0; }

        let rand_index = self.rng.rand_range(0, moves.size);

        moves[rand_index]
    }
}