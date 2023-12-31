use crate::chess::types::Color;
use crate::chess::{position::Position, types::Move};

use super::MoveEval;
use super::PIECE_TYPE_VALUE;

pub mod simple;

pub use simple::SimpleMoveEval;

impl MoveEval for () {
    fn eval<const US: Color, const THEM: Color>(_p: &Position, _m: Move) -> i32 {
        0
    }
}