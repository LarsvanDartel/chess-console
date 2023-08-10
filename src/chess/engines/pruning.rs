use crate::chess::position::GameResult::Checkmate;
use crate::chess::position::Position;
use crate::chess::types::Move;
use crate::chess::types::Color;

use super::Engine;
use super::eval::Eval;

pub struct PruningEngine<const US: Color, const THEM: Color, const DEPTH: usize, E: Eval> {
    eval: E
}

impl<const US: Color, const THEM: Color, const DEPTH: usize, E: Eval> PruningEngine<US, THEM, DEPTH, E> {
    pub fn new() -> Self {
        PruningEngine::<US, THEM, DEPTH, E> { eval: E::new() }
    }

    fn search<const U: Color, const T: Color>(&self, p: &mut Position, depth: usize, alpha: i32, beta: i32) -> i32 {
        let moves = p.generate_moves::<U, T>();
        
        if depth == 0 { return self.eval.eval(p) }
        
        if p.result.is_some() {
            return match p.result.unwrap() {
                Checkmate(_) => -i32::MAX,
                _ => 0
            };
        }
        
        let mut best_score = alpha;
        for i in 0..moves.size {
            p.make_move::<U>(moves[i]);
            let score = -self.search::<T, U>(p, depth - 1, -beta, -best_score);
            p.undo_move::<U>(moves[i]);

            if score >= beta {
                return beta;
            }

            best_score = best_score.max(score);
        }

        best_score
    }
}

impl<const US: Color, const THEM: Color, const DEPTH: usize, E: Eval> Engine for PruningEngine<US, THEM, DEPTH, E> {
    fn best_move(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);

        let moves = p.generate_moves::<US, THEM>();

        let mut best_score = -i32::MAX;
        let mut best_move = moves[0];
        for i in 0..moves.size {
            p.make_move::<US>(moves[i]);
            let score = -self.search::<THEM, US>(p, DEPTH, -i32::MAX, -best_score);
            p.undo_move::<US>(moves[i]);
    
            if score > best_score {
                best_score = score;
                best_move = moves[i];
            }
        }

        best_move
    }
}