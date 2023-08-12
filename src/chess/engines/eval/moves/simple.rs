use crate::chess::position::Position;
use crate::chess::tables::all_pawn_attacks;
use crate::chess::types::*;

use super::MoveEval;
use super::PIECE_TYPE_VALUE;
pub struct SimpleMoveEval;

impl MoveEval for SimpleMoveEval {
    fn eval<const US: Color, const THEM: Color>(p: &Position, m: Move) -> i32 {
        let mut move_score = 0;

        if move_is_capture(m) {
            let moving_piece = p.board[get_from(m) as usize];

            let mut captured_sq = get_to(m);
            if get_flags(m) == EN_PASSANT {
                captured_sq = (captured_sq as Dir + relative_dir::<US>(SOUTH)) as Square;
            }

            let captured_piece = p.board[captured_sq as usize];

            move_score += 10 * PIECE_TYPE_VALUE[piece_type_of(moving_piece) as usize] -
                PIECE_TYPE_VALUE[piece_type_of(captured_piece) as usize];
        }

        if move_is_promotion(m) {
            let promotion_piece = get_promotion_type(m);
            move_score += PIECE_TYPE_VALUE[piece_type_of(promotion_piece) as usize];
        }

        if SQUARE_BB[get_to(m) as usize] & all_pawn_attacks::<THEM>(p.bb_of(THEM, PAWN)) != 0 {
            let moving_piece = p.board[get_from(m) as usize];
            move_score -= PIECE_TYPE_VALUE[piece_type_of(moving_piece) as usize];
        }

        move_score
    }
}