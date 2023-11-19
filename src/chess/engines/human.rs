use std::ops::ControlFlow;

use crate::chess::position::Position;
use crate::chess::types::*;

use super::Engine;
use super::eval::MoveEval;
use super::eval::PositionEval;

const DEBUG: bool = true;

pub struct HumanPlayer;

impl HumanPlayer {
    fn get_start_sq(p: &Position, moves: &mut Vec<Move>, highlighted_squares: &mut Bitboard) -> ControlFlow<(), ()> {
        *highlighted_squares = 0;
        print!("\x1B[2J\x1B[1;1H");
        p.print(*highlighted_squares);
        println!("Enter starting square:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if DEBUG && input.trim() == "fen" {
            println!("{}", p.fen());
            std::io::stdin().read_line(&mut input).unwrap();
            return ControlFlow::Continue(());
        }

        if let Ok(sq) = square_from_str(&input) {
            if moves.iter().all(|&m| get_from(m) != sq) { return ControlFlow::Continue(()); }

            moves.retain(|&m| get_from(m) == sq);

            moves.iter().for_each(|&m| {
                *highlighted_squares |= SQUARE_BB[get_to(m) as usize];
            });

            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    }
    
    fn get_end_sq(p: &Position, moves: &mut Vec<Move>, highlighted_squares: &mut Bitboard) -> ControlFlow<bool, bool> {
        print!("\x1B[2J\x1B[1;1H");
        p.print(*highlighted_squares);
        println!("Enter ending square:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if DEBUG && input.trim() == "quit" {
            return ControlFlow::Continue(true);
        }

        if let Ok(sq) = square_from_str(&input) {
            if moves.iter().all(|&m| get_to(m) != sq) { return ControlFlow::Continue(false); }

            moves.retain(|&m| get_to(m) == sq);

            if moves.len() > 1 {
                moves.iter().for_each(|&m| assert!(move_is_promotion(m)));
                return ControlFlow::Break(true);
            }

            return ControlFlow::Break(false);
        }

        ControlFlow::Continue(false)
    }

    fn get_promotion_type(p: &Position, moves: &mut Vec<Move>, highlighted_squares: &mut Bitboard) -> ControlFlow<(), bool> {
        print!("\x1B[2J\x1B[1;1H");
        p.print(*highlighted_squares);
        println!("Enter promotion type: (q/r/n/b)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if DEBUG && input.trim() == "quit" {
            return ControlFlow::Continue(true);
        }

        if let Some(pt) = input.chars().next() {
            if pt != 'q' && pt != 'r' && pt != 'n' && pt != 'b' { return ControlFlow::Continue(false); }

            let promotion_type = piece_type_from(pt);

            let flags = match promotion_type {
                QUEEN => PQ_QUEEN,
                ROOK => PQ_ROOK,
                BISHOP => PQ_BISHOP,
                KNIGHT => PQ_KNIGHT,
                _ => unreachable!()
            };

            moves.retain(|&m| get_flags(m) & !CAPTURE == flags);

            assert!(moves.len() == 1);

            return ControlFlow::Break(());
        }

        ControlFlow::Continue(false)
    }
}

impl Engine for HumanPlayer {
    fn new<const US: Color, const THEM: Color>() -> Self {
        HumanPlayer
    }

    fn best_move<
            const US: Color, const THEM: Color, const DEPTH: usize,
            PE: PositionEval, ME: MoveEval
        >(&mut self, p: &mut Position) -> Move {
        assert!(p.turn == US);
        let mut moves;
        let mut highlighted_squares;

        'main_loop: loop {
            moves = p.generate_moves::<US, THEM>().to_vec();
            highlighted_squares = 0;

            loop {
                match Self::get_start_sq(p, &mut moves, &mut highlighted_squares) {
                    ControlFlow::Continue(()) => continue,
                    ControlFlow::Break(()) => break,
                }
            }

            loop {
                match Self::get_end_sq(p, &mut moves, &mut highlighted_squares) {
                    ControlFlow::Continue(true) => continue 'main_loop,
                    ControlFlow::Continue(false) => continue,
                    ControlFlow::Break(true) => break,
                    ControlFlow::Break(false) => break 'main_loop,
                }
            }
            
            loop {
                match Self::get_promotion_type(p, &mut moves, &mut highlighted_squares) {
                    ControlFlow::Continue(true) => continue 'main_loop,
                    ControlFlow::Continue(false) => continue,
                    ControlFlow::Break(()) => break,
                }
            }

            break;
        }
        moves[0]
    }
}