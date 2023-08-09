use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;
use crate::chess::types::move_is_capture;

use super::Engine;
use super::PRNG;

pub struct CaptureEngine<const US: Color, const THEM: Color> {
    rng: PRNG
}

impl<const US: Color, const THEM: Color> CaptureEngine<US, THEM> {
    pub fn new() -> Self {
        CaptureEngine::<US, THEM> { rng: PRNG::new() }
    }
}

impl<const US: Color, const THEM: Color> Engine for CaptureEngine<US, THEM> {
    fn best_move(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);

        let moves = p.generate_moves::<US, THEM>();

        for i in 0..moves.size {
            if move_is_capture(moves[i]) {
                return moves[i];
            }
        }

        let random_index = self.rng.rand_range(0, moves.size);

        moves[random_index]
    }
}