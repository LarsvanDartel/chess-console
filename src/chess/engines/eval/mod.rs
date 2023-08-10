pub mod positions;
pub mod moves;

pub use positions::*;
pub use moves::*;

use crate::chess::{position::Position, types::Move};

pub trait PositionEval {
    fn new() -> Self;
    fn eval(&self, p: &Position) -> i32;
}

pub trait MoveEval {
    fn new() -> Self;
    fn eval(&self, p: &Position, m: &Move) -> i32;
}