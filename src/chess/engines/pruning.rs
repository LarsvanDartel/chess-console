use crate::chess::position::GameResult::Checkmate;
use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;

use super::Engine;
use super::eval::{PositionEval, MoveEval};

pub struct PruningEngine;

impl PruningEngine {
    fn search<const US: Color, const THEM: Color, PE: PositionEval>(p: &mut Position, depth: usize, alpha: i32, beta: i32) -> i32 {
        let moves = p.generate_moves::<US, THEM>();

        if depth == 0 { return PE::eval::<US, THEM>(p); }

        if p.result.is_some() {
            return match p.result.unwrap() {
                Checkmate(_) => -i32::MAX,
                _ => 0
            };
        }

        let mut best_score = alpha;
        for i in 0..moves.size {
            p.make_move::<US>(moves[i]);
            let score = -Self::search::<THEM, US, PE>(p, depth - 1, -beta, -best_score);
            p.undo_move::<US>(moves[i]);

            if score >= beta {
                return beta;
            }

            best_score = best_score.max(score);
        }

        best_score
    }
}

impl Engine for PruningEngine {
    fn new<const US: Color, const THEM: Color>() -> Self {
        PruningEngine
    }

    fn best_move<
            const US: Color, const THEM: Color, const DEPTH: usize,
            PE: PositionEval, ME: MoveEval
        >(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);

        let moves = p.generate_moves::<US, THEM>();

        if moves.size == 0 { return 0; }

        let mut best_score = -i32::MAX;
        let mut best_move = moves[0];
        for i in 0..moves.size {
            p.make_move::<US>(moves[i]);
            let score = -Self::search::<THEM, US, PE>(p, DEPTH, -i32::MAX, -best_score);
            p.undo_move::<US>(moves[i]);

            if score > best_score {
                best_score = score;
                best_move = moves[i];
            }
        }

        best_move
    }
}