pub mod positions;
pub mod moves;

pub use positions::*;
pub use moves::*;

use crate::chess::{position::Position, types::{Move, N_PIECE_TYPES, Color}};

const PIECE_TYPE_VALUE: [i32; N_PIECE_TYPES] = [
    0, // KING
    900, // QUEEN
    500, // ROOK
    300, // BISHOP
    300, // KNIGHT
    100, // PAWN
];

pub trait PositionEval {
    fn eval<const US: Color, const THEM: Color>(p: &Position) -> i32;
}

pub trait MoveEval {
    fn eval<const US: Color, const THEM: Color>(p: &Position, m: Move) -> i32;
}