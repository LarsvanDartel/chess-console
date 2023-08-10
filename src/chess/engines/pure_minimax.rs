use crate::chess::position::GameResult::Checkmate;
use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;

use super::Engine;
use super::eval::{PositionEval, MoveEval};

pub struct MiniMaxEngine;

impl MiniMaxEngine {
    fn search<const US: Color, const THEM: Color, PE: PositionEval>(p: &mut Position, depth: usize) -> i32 {
        let moves = p.generate_moves::<US, THEM>();

        if depth == 0 { return PE::eval::<US, THEM>(p); }

        if p.result.is_some() {
            return match p.result.unwrap() {
                Checkmate(_) => -i32::MAX,
                _ => 0
            };
        }

        let mut best = i32::MIN;
        for i in 0..moves.size {
            p.make_move::<US>(moves[i]);
            let score = -Self::search::<THEM, US, PE>(p, depth - 1);
            best = best.max(score);

            p.undo_move::<US>(moves[i]);
        }

        best
    }
}

impl Engine for MiniMaxEngine {
    fn new<const US: Color, const THEM: Color>() -> Self {
        MiniMaxEngine
    }
    fn best_move<
            const US: Color, const THEM: Color, const DEPTH: usize,
            PE: PositionEval, ME: MoveEval
        >(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);

        let moves = p.generate_moves::<US, THEM>();

        if moves.size == 0 { return 0; }

        let mut best_score = i32::MIN;
        let mut best_move = moves[0];
        for i in 0..moves.size {
            p.make_move::<US>(moves[i]);

            let score = -Self::search::<THEM, US, PE>(p, DEPTH);

            if score > best_score {
                best_score = score;
                best_move = moves[i];
            }

            p.undo_move::<US>(moves[i]);
        }

        best_move
    }
}