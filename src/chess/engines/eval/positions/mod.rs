pub mod material;

pub use material::MaterialEval;

use super::PositionEval;
use super::PIECE_TYPE_VALUE;
use crate::chess::position::Position;
use crate::chess::types::Color;

impl PositionEval for () {
    fn eval<const US: Color, const THEM: Color>(_p: &Position) -> i32 {
        0
    }
}