use crate::chess::types::{N_PIECE_TYPES, sparse_count_ones};
use crate::chess::position::Position;
use crate::chess::position::GameResult::Checkmate;

use super::PositionEval;

const PIECE_TYPE_VALUE: [i32; N_PIECE_TYPES] = [
    0, // KING
    900, // QUEEN
    500, // ROOK
    300, // BISHOP
    300, // KNIGHT
    100, // PAWN
];

pub struct MaterialEval;

impl PositionEval for MaterialEval {
    fn new() -> Self {
        MaterialEval {}
    }

    fn eval(&self, p: &Position) -> i32 {
        if p.result.is_some() {
            return match p.result.unwrap() {
                Checkmate(_) => -i32::MAX,
                _ => 0
            };
        }

        let mut eval = 0;

        let mut bb;
        for pt in 0..N_PIECE_TYPES {
            bb = p.bb_of(p.turn, pt as u8);
            eval += sparse_count_ones(bb) as i32 * PIECE_TYPE_VALUE[pt];
            bb = p.bb_of(p.turn ^ 1, pt as u8);
            eval -= sparse_count_ones(bb) as i32 * PIECE_TYPE_VALUE[pt];
        }

        eval
    }
}