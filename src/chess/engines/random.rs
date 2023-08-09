use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;

use super::Engine;
use super::PRNG;

pub struct RandomEngine<const US: Color, const THEM: Color> {
    rng: PRNG
}

impl<const US: Color, const THEM: Color> RandomEngine<US, THEM> {
    pub fn new() -> Self {
        RandomEngine::<US, THEM> { rng: PRNG::new() }
    }
}

impl<const US: Color, const THEM: Color> Engine for RandomEngine<US, THEM> {
    fn best_move(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);

        let moves = p.generate_moves::<US, THEM>();

        let random_index = self.rng.rand_range(0, moves.size);

        moves[random_index]
    }
}