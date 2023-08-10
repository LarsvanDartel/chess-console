pub mod eval;

pub mod random;
pub mod capture;
pub mod pure_minimax;
pub mod pruning;
pub mod ordering;

use self::eval::{PositionEval, MoveEval};

pub use random::RandomEngine;
pub use capture::CaptureEngine;
pub use pure_minimax::MiniMaxEngine;
pub use pruning::PruningEngine;
pub use ordering::OrderingEngine;

use std::time::{SystemTime, UNIX_EPOCH};

use super::types::{Move, Color};
use super::position::Position;

pub trait Engine {
    fn new<const US: Color, const THEM: Color>() -> Self;
    fn best_move<
        const US: Color, const THEM: Color, const DEPTH: usize,
        PE: PositionEval, ME: MoveEval
    >(&mut self, p: &mut Position) -> Move;
}

pub struct PRNG {
    seed: u64,
}

impl PRNG {
    pub fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() & u64::MAX as u128;

        PRNG { seed: seed as u64 }
    }

    pub const fn new_seeded(seed: u64) -> Self {
        PRNG { seed }
    }

    fn rand_u64(&mut self) -> u64 {
        let s = &mut self.seed;
        *s ^= *s >> 12;
        *s ^= *s << 25;
        *s ^= *s >> 27;
        *s * 2685821657736338717u64
    }

    pub fn rand<T: From<u64>>(&mut self) -> T {
        T::from(self.rand_u64())
    }

    pub fn rand_range(&mut self, a: usize, b: usize) -> usize {
        let rand = self.rand_u64();
        
        let factor = (rand as f64) / (u64::MAX as f64 + 1.0);

        a + ((b - a) as f64 * factor) as usize
    }
}