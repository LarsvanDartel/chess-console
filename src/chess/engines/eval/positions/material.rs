use crate::chess::types::{N_PIECE_TYPES, sparse_count_ones, Color};
use crate::chess::position::Position;
use crate::chess::position::GameResult::Checkmate;

use super::PositionEval;
use super::PIECE_TYPE_VALUE;

pub struct MaterialEval;

impl PositionEval for MaterialEval {
    fn eval<const US: Color, const THEM: Color>(p: &Position) -> i32 {
        if p.result.is_some() {
            return match p.result.unwrap() {
                Checkmate(_) => -i32::MAX,
                _ => 0
            };
        }

        let mut eval = 0;

        let mut bb;
        for pt in 0..N_PIECE_TYPES {
            bb = p.bb_of(US, pt as u8);
            eval += sparse_count_ones(bb) as i32 * PIECE_TYPE_VALUE[pt];
            bb = p.bb_of(THEM, pt as u8);
            eval -= sparse_count_ones(bb) as i32 * PIECE_TYPE_VALUE[pt];
        }

        eval
    }
}