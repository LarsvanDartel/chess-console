pub mod material;

use crate::chess::position::Position;

pub trait Eval {
    fn new() -> Self;
    fn eval(&self, p: &Position) -> i32;
}