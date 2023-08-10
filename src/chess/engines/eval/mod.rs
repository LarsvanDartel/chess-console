pub mod material;

pub use material::MaterialEval;

use crate::chess::position::Position;

pub trait Eval {
    fn new() -> Self;
    fn eval(&self, p: &Position) -> i32;
}