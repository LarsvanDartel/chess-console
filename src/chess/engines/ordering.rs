use crate::chess::position::GameResult::Checkmate;
use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;

use super::Engine;
use super::eval::{PositionEval, MoveEval};

pub struct OrderingEngine;

impl OrderingEngine {
    fn search<const US: Color, const THEM: Color, PE: PositionEval, ME: MoveEval>(p: &mut Position, depth: usize, alpha: i32, beta: i32) -> i32 {
        if depth == 0 { return PE::eval::<US, THEM>(p); }
        
        let mut moves = p.generate_moves::<US, THEM>().to_vec();
        moves.sort_unstable_by_key(|&m| -ME::eval::<US, THEM>(p, m));

        if p.result.is_some() {
            return match p.result.unwrap() {
                Checkmate(_) => -i32::MAX,
                _ => 0
            };
        }
        
        let mut best_score = alpha;
        for m in moves {
            p.make_move::<US>(m);
            let score = -Self::search::<THEM, US, PE, ME>(p, depth - 1, -beta, -best_score);
            p.undo_move::<US>(m);

            if score >= beta {
                return beta;
            }

            best_score = best_score.max(score);
        }

        best_score
    }
}

impl Engine for OrderingEngine {
    fn new<const US: Color, const THEM: Color>() -> Self {
        OrderingEngine
    }

    fn best_move<
            const US: Color, const THEM: Color, const DEPTH: usize,
            PE: PositionEval, ME: MoveEval
        >(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);

        let mut moves = p.generate_moves::<US, THEM>().to_vec();
        moves.sort_unstable_by_key(|&m| ME::eval::<US, THEM>(p, m));

        if moves.len() == 0 { return 0; }

        let mut best_score = -i32::MAX;
        let mut best_move = moves[0];
        for m in moves {
            p.make_move::<US>(m);
            let score = -Self::search::<THEM, US, PE, ME>(p, DEPTH, -i32::MAX, -best_score);
            p.undo_move::<US>(m);
    
            if score > best_score {
                best_score = score;
                best_move = m;
            }
        }

        best_move
    }
}