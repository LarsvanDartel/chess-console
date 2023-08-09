use std::time::Duration;

use super::engines::Engine;
use super::position::{Position, STARTING_FEN};
use super::position::GameResult;
use super::types::{WHITE, BLACK};
use super::types::square_from_str;
use super::types::{piece_type_from, QUEEN, ROOK, BISHOP, KNIGHT};
use super::types::{Move, get_from, get_to, get_flags, SQUARE_BB};
use super::types::{move_is_promotion, PQ_QUEEN, PQ_ROOK, PQ_BISHOP, PQ_KNIGHT, CAPTURE};

pub struct Game {
    position: Position,
    player_a: Option<Box<dyn Engine>>,
    player_b: Option<Box<dyn Engine>>,
}

impl Game {
    pub fn new(player_a: Option<Box<dyn Engine>>, player_b: Option<Box<dyn Engine>>) -> Self {
        let mut p = Position::new();
        p.load_fen(STARTING_FEN).unwrap();

        Game { position: p, player_a, player_b }
    }

    pub fn start(&mut self, delay: u64) -> GameResult {
        let mut input;

        let mut moves;
        let mut highlighted_squares = 0;
        let mut prev_move = 0;
        let mut is_bot_turn = false;
        'game_loop: loop {
            if self.position.turn == WHITE {
                moves = self.position.generate_moves::<WHITE, BLACK>().to_vec();
            } else {
                moves = self.position.generate_moves::<BLACK, WHITE>().to_vec();
            }

            if self.position.result.is_some() {
                break;
            }

            if self.position.turn == WHITE && self.player_a.is_some() {
                let m = self.player_a.as_mut().unwrap().best_move(&mut self.position);

                self.position.make_move::<WHITE>(m);

                prev_move = m;
                is_bot_turn = true;
            } else if self.position.turn == BLACK && self.player_b.is_some() {
                let m = self.player_b.as_mut().unwrap().best_move(&mut self.position);

                self.position.make_move::<BLACK>(m);

                prev_move = m;
                is_bot_turn = true;
            }

            if is_bot_turn && delay > 0 {
                is_bot_turn = false;
                highlighted_squares = 0;
                highlighted_squares |= SQUARE_BB[get_from(prev_move) as usize];
                highlighted_squares |= SQUARE_BB[get_to(prev_move) as usize];


                print!("\x1B[2J\x1B[1;1H");
                self.position.print(highlighted_squares);

                std::thread::sleep(Duration::from_millis(delay));

                continue;
            }

            if is_bot_turn {
                is_bot_turn = false;
                continue;
            }


            loop {
                print!("\x1B[2J\x1B[1;1H");
                self.position.print(highlighted_squares);
                println!("Enter starting square:");

                input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "fen" {
                    println!("{}", self.position.fen());
                    std::io::stdin().read_line(&mut input).unwrap();
                    continue;
                }

                if let Ok(sq) = square_from_str(&input) {
                    let filtered_moves = moves.iter()
                        .map(|&m| m)
                        .filter(|&m| get_from(m) == sq)
                        .collect::<Vec<Move>>();
                    if filtered_moves.len() == 0 { continue; }

                    moves = filtered_moves;

                    moves.iter().for_each(|&m| {
                        highlighted_squares |= SQUARE_BB[get_to(m) as usize];
                    });
                    break;
                }
            }

            loop {
                print!("\x1B[2J\x1B[1;1H");
                self.position.print(highlighted_squares);
                println!("Enter ending square:");

                input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "undo" {
                    if self.position.turn == WHITE {
                        self.position.undo_move::<BLACK>(prev_move);
                    } else {
                        self.position.undo_move::<WHITE>(prev_move);
                    }
                }
                if input.trim() == "quit" { continue 'game_loop; }

                if let Ok(sq) = square_from_str(&input) {
                    let filtered_moves = moves.iter()
                        .map(|&m| m)
                        .filter(|&m| get_to(m) == sq)
                        .collect::<Vec<Move>>();
                    if filtered_moves.len() == 0 { continue; }

                    moves = filtered_moves;

                    break;
                }
            }

            if move_is_promotion(moves[0]) {
                loop {
                    print!("\x1B[2J\x1B[1;1H");
                    self.position.print(highlighted_squares);
                    println!("Enter promotion type: (q/r/n/b)");

                    input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    if input.trim() == "quit" { continue 'game_loop; }

                    let promotion_type = piece_type_from(input.chars().next().unwrap());
                    let flags = match promotion_type {
                        QUEEN => PQ_QUEEN,
                        ROOK => PQ_ROOK,
                        BISHOP => PQ_BISHOP,
                        KNIGHT => PQ_KNIGHT,
                        _ => continue
                    };

                    moves = moves.iter()
                        .map(|&m| m)
                        .filter(|&m| get_flags(m) == flags || get_flags(m) == flags & CAPTURE)
                        .collect::<Vec<Move>>();
                }
            }

            assert!(moves.len() == 1);

            prev_move = moves[0];

            highlighted_squares = 0;
            highlighted_squares |= SQUARE_BB[get_from(prev_move) as usize];
            highlighted_squares |= SQUARE_BB[get_to(prev_move) as usize];
            if self.position.turn == WHITE {
                self.position.make_move::<WHITE>(moves[0]);
            } else {
                self.position.make_move::<BLACK>(moves[0]);
            }
        }

        print!("\x1B[2J\x1B[1;1H");
        self.position.print(0);
        let result = self.position.result.as_ref().unwrap();
        match *result {
            GameResult::Checkmate(color) => if color == WHITE {
                println!("WHITE won by checkmate");
            } else {
                println!("BLACK won by checkmate");
            }
            GameResult::Stalemate => println!("Draw by stalemate"),
            GameResult::FiftyMoveRule => println!("Draw by fifty move rule"),
            GameResult::ThreefoldRepetition => println!("Draw by threefold repetition"),
            GameResult::InsufficientMaterial => println!("Draw by insufficient material"),
            
        }

        *result
    }
}
