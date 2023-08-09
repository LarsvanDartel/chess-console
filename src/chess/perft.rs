use std::time::Instant;

use super::types::{Color, WHITE, BLACK};
use super::types::print_move;
use super::position::Position;
use super::position::STARTING_FEN;

pub fn perft<const US: Color, const THEM: Color>(p: &mut Position, depth: usize) -> u64 {
    let mut nodes = 0;

    let moves = p.generate_moves::<US, THEM>();

    if depth == 1 { return moves.size as u64 }

    for i in 0..moves.size {
        p.make_move::<US>(moves[i]);
        nodes += perft::<THEM, US>(p, depth - 1);
        p.undo_move::<US>(moves[i]);
    }

    nodes
}

pub fn perftdiv<const US: Color, const THEM: Color>(p: &mut Position, depth: usize) {
    let mut nodes = 0;
    let mut pf;

    let moves = p.generate_moves::<US, THEM>();

    for i in 0..moves.size {
        print_move(moves[i]);

        p.make_move::<US>(moves[i]);
        pf = perft::<THEM, US>(p, depth - 1);

        println!(": {} moves", pf);
        nodes += pf;

        p.undo_move::<US>(moves[i]);
    }

    println!("\n Total: {} moves", nodes);
}

pub fn perft_test() {
    let mut p = Position::new();
    p.load_fen(STARTING_FEN).unwrap();

    println!("{}", p);

    let begin = Instant::now();
    let n = perft::<WHITE, BLACK>(&mut p, 6);
    let elapsed = begin.elapsed();

    println!("Nodes: {}", n);
    println!("NPS:   {}", n as u128 * 1000000 / elapsed.as_micros());
    println!("Time:  {}us", elapsed.as_micros());
}