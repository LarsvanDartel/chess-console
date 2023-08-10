use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;
use crate::chess::types::move_is_capture;

use super::Engine;
use super::PRNG;
use super::eval::MoveEval;
use super::eval::PositionEval;

pub struct CaptureEngine {
    rng: PRNG
}

impl Engine for CaptureEngine {
    fn new<const US: Color, const THEM: Color>() -> Self {
        CaptureEngine { rng: PRNG::new() }
    }

    fn best_move<
            const US: Color, const THEM: Color, const DEPTH: usize,
            PE: PositionEval, ME: MoveEval
        >(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);
        let moves = p.generate_moves::<US, THEM>();

        if moves.size == 0 { return 0; }

        for i in 0..moves.size {
            if move_is_capture(moves[i]) { return moves[i]; }
        }

        let rand_index = self.rng.rand_range(0, moves.size);

        moves[rand_index]
    }
}