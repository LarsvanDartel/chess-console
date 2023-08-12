
/**
 * file:          types.h
 * author:        Lars van Dartel
 * date:          2023-08-04
 * version:       1.0
 */
use std::ops::Index;

pub type Color = u8;

pub const N_COLORS: usize = 2;

pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

pub const fn opposite<const COLOR: Color>() -> Color {
    COLOR ^ 1
}

pub type PieceType = u8;

pub const N_PIECE_TYPES: usize = 6;


pub const KING:   PieceType = 0;
pub const QUEEN:  PieceType = 1;
pub const ROOK:   PieceType = 2;
pub const BISHOP: PieceType = 3;
pub const KNIGHT: PieceType = 4;
pub const PAWN:   PieceType = 5;
pub const PIECE_TYPE_NONE: PieceType = 6;

pub const fn piece_type_from(p: char) -> PieceType {
    match p {
        'k' | 'K' => KING,
        'q' | 'Q' => QUEEN,
        'r' | 'R' => ROOK,
        'b' | 'B' => BISHOP,
        'n' | 'N' => KNIGHT,
        'p' | 'P' => PAWN,
        _ => PIECE_TYPE_NONE
    }
}

pub type Piece = u8;

pub const N_PIECES: usize = 15;

pub const WHITE_KING:   Piece = create_piece(WHITE, KING);
pub const WHITE_QUEEN:  Piece = create_piece(WHITE, QUEEN);
pub const WHITE_ROOK:   Piece = create_piece(WHITE, ROOK);
pub const WHITE_BISHOP: Piece = create_piece(WHITE, BISHOP);
pub const WHITE_KNIGHT: Piece = create_piece(WHITE, KNIGHT);
pub const WHITE_PAWN:   Piece = create_piece(WHITE, PAWN);

pub const BLACK_KING:   Piece = create_piece(BLACK, KING);
pub const BLACK_QUEEN:  Piece = create_piece(BLACK, QUEEN);
pub const BLACK_ROOK:   Piece = create_piece(BLACK, ROOK);
pub const BLACK_BISHOP: Piece = create_piece(BLACK, BISHOP);
pub const BLACK_KNIGHT: Piece = create_piece(BLACK, KNIGHT);
pub const BLACK_PAWN:   Piece = create_piece(BLACK, PAWN);
    
pub const PIECE_NONE:   Piece = 14;

pub const PIECE_STR: [&str; N_PIECES] = [
    "K", "Q", "R", "B", "N", "P", "", "",
    "k", "q", "r", "b", "n", "p", " "
];

// 0x2654 - 0x265F
//pub const PIECE_STR: [&str; N_PIECES] = [
//    "♔", "♕", "♖", "♗", "♘", "♙", "", "",
//    "♚", "♛", "♜", "♝", "♞", "♟", " "
//];

pub const fn create_piece(c: Color, pt: PieceType) -> Piece {
    (c << 3) + pt
}

pub const fn piece_from(p: char) -> Piece {
    let c = if p.is_ascii_uppercase() { WHITE } else { BLACK };
    create_piece(c, piece_type_from(p))
}

pub const fn color_of(p: Piece) -> Color {
    p >> 3
}

pub const fn piece_type_of(p: Piece) -> PieceType {
    p & 0b111
}

pub type Bitboard = u64;

pub fn pop_lsb(bb: &mut Bitboard) -> u8 {
    let lsb = bb.trailing_zeros() as u8;
    *bb &= *bb - 1;
    lsb
}

pub const fn shift_bb<const DIR: Dir>(bb: Bitboard) -> Bitboard {
    match DIR {
        NORTH => bb << 8,
        SOUTH => bb >> 8,
        NORTH_NORTH => bb << 16,
        SOUTH_SOUTH => bb >> 16,
        EAST => (bb & !MASK_FILE[FILE_H as usize]) << 1,
        WEST => (bb & !MASK_FILE[FILE_A as usize]) >> 1,
        NORTH_EAST => (bb & !MASK_FILE[FILE_H as usize]) << 9,
        NORTH_WEST => (bb & !MASK_FILE[FILE_A as usize]) << 7,
        SOUTH_EAST => (bb & !MASK_FILE[FILE_H as usize]) >> 7,
        SOUTH_WEST => (bb & !MASK_FILE[FILE_A as usize]) >> 9,
        _ => unreachable!()
    }
}

pub fn sparse_count_ones(bb: Bitboard) -> u8 {
    let mut b = bb;
    let mut count = 0;

    while b > 0 {
        b &= b - 1;
        count += 1;
    }

    count
}

pub fn print_bitboard(bb: Bitboard) {
    for rank in (0..8).rev() {
        print!("{}", rank + 1);
        for file in 0..8 {
            print!("{}", (bb >> (rank * 8 + file)) & 1);
        }
        println!();
    }
    println!(" abcdefgh");
}

pub type Dir = i8;

pub const N_DIRS: usize = 8;

pub const NORTH:       Dir = 8;
pub const EAST:        Dir = 1;
pub const SOUTH:       Dir = -8;
pub const WEST:        Dir = -1;

pub const NORTH_EAST:  Dir = 9;
pub const SOUTH_EAST:  Dir = -7;
pub const SOUTH_WEST:  Dir = -9;
pub const NORTH_WEST:  Dir = 7;

pub const NORTH_NORTH: Dir = 16;
pub const SOUTH_SOUTH: Dir = -16;

pub const fn relative_dir<const COLOR: Color>(dir: Dir) -> Dir {
    if COLOR == WHITE { dir } else { -dir }
}

pub const N_SQUARES: usize = 64;

pub type Square = u8;

pub const A1: Square = 0 + 0 * 8; pub const A2: Square = 0 + 1 * 8;
pub const B1: Square = 1 + 0 * 8; pub const B2: Square = 1 + 1 * 8;
pub const C1: Square = 2 + 0 * 8; pub const C2: Square = 2 + 1 * 8;
pub const D1: Square = 3 + 0 * 8; pub const D2: Square = 3 + 1 * 8;
pub const E1: Square = 4 + 0 * 8; pub const E2: Square = 4 + 1 * 8;
pub const F1: Square = 5 + 0 * 8; pub const F2: Square = 5 + 1 * 8;
pub const G1: Square = 6 + 0 * 8; pub const G2: Square = 6 + 1 * 8;
pub const H1: Square = 7 + 0 * 8; pub const H2: Square = 7 + 1 * 8;

pub const A3: Square = 0 + 2 * 8; pub const A4: Square = 0 + 3 * 8;
pub const B3: Square = 1 + 2 * 8; pub const B4: Square = 1 + 3 * 8;
pub const C3: Square = 2 + 2 * 8; pub const C4: Square = 2 + 3 * 8;
pub const D3: Square = 3 + 2 * 8; pub const D4: Square = 3 + 3 * 8;
pub const E3: Square = 4 + 2 * 8; pub const E4: Square = 4 + 3 * 8;
pub const F3: Square = 5 + 2 * 8; pub const F4: Square = 5 + 3 * 8;
pub const G3: Square = 6 + 2 * 8; pub const G4: Square = 6 + 3 * 8;
pub const H3: Square = 7 + 2 * 8; pub const H4: Square = 7 + 3 * 8;

pub const A5: Square = 0 + 4 * 8; pub const A6: Square = 0 + 5 * 8;
pub const B5: Square = 1 + 4 * 8; pub const B6: Square = 1 + 5 * 8;
pub const C5: Square = 2 + 4 * 8; pub const C6: Square = 2 + 5 * 8;
pub const D5: Square = 3 + 4 * 8; pub const D6: Square = 3 + 5 * 8;
pub const E5: Square = 4 + 4 * 8; pub const E6: Square = 4 + 5 * 8;
pub const F5: Square = 5 + 4 * 8; pub const F6: Square = 5 + 5 * 8;
pub const G5: Square = 6 + 4 * 8; pub const G6: Square = 6 + 5 * 8;
pub const H5: Square = 7 + 4 * 8; pub const H6: Square = 7 + 5 * 8;

pub const A7: Square = 0 + 6 * 8; pub const A8: Square = 0 + 7 * 8;
pub const B7: Square = 1 + 6 * 8; pub const B8: Square = 1 + 7 * 8;
pub const C7: Square = 2 + 6 * 8; pub const C8: Square = 2 + 7 * 8;
pub const D7: Square = 3 + 6 * 8; pub const D8: Square = 3 + 7 * 8;
pub const E7: Square = 4 + 6 * 8; pub const E8: Square = 4 + 7 * 8;
pub const F7: Square = 5 + 6 * 8; pub const F8: Square = 5 + 7 * 8;
pub const G7: Square = 6 + 6 * 8; pub const G8: Square = 6 + 7 * 8;
pub const H7: Square = 7 + 6 * 8; pub const H8: Square = 7 + 7 * 8;

pub const SQUARE_NONE: Square = 64;

pub const SQUARE_STR: [&str; 65] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "None",
];

pub const fn create_square(file: File, rank: Rank) -> Square {
    rank << 3 | file
}

pub fn square_from_str(sq: &str) -> Result<Square, String> {
    let mut c = sq.chars();
    let file = c.next().ok_or("Missing file")?.to_ascii_lowercase() as u8;
    if file < b'a' || file > b'h' { return Err(format!("Invalid file '{}'", file as char)); }
    let rank = c.next().ok_or("Missing rank")?.to_ascii_lowercase() as u8;
    if rank < b'1' || rank > b'8' { return Err(format!("Invalid rank '{}'", rank as char)); }

    Ok(create_square(
        file - b'a',
        rank - b'1',
    ))
}

pub const fn rank_of(sq: Square) -> Rank {
    sq >> 3
}

pub const fn file_of(sq: Square) -> File {
    sq & 0b111
}

pub const fn diag_of(sq: Square) -> u8 {
    7 + rank_of(sq) - file_of(sq)
}

pub const fn anti_diag_of(sq: Square) -> u8 {
    rank_of(sq) + file_of(sq)
}

pub type File = u8;

pub const FILE_A: File = 0;
pub const FILE_B: File = 1;
pub const FILE_C: File = 2;
pub const FILE_D: File = 3;
pub const FILE_E: File = 4;
pub const FILE_F: File = 5;
pub const FILE_G: File = 6;
pub const FILE_H: File = 7;

pub type Rank = u8;

pub const RANK_1: Rank = 0;
pub const RANK_2: Rank = 1;
pub const RANK_3: Rank = 2;
pub const RANK_4: Rank = 3;
pub const RANK_5: Rank = 4;
pub const RANK_6: Rank = 5;
pub const RANK_7: Rank = 6;
pub const RANK_8: Rank = 7;

pub const fn relative_rank<const COLOR: Color>(r: Rank) -> Rank {
    if COLOR == WHITE { r } else { 7 - r }
}

/// Precomputed file masks
pub const MASK_FILE: [Bitboard; 8] = [
    0x0101010101010101, 0x0202020202020202, 0x0404040404040404, 0x0808080808080808,
    0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080,
];

/// Precomputed rank masks
pub const MASK_RANK: [Bitboard; 8] = [
    0x00000000000000ff, 0x000000000000ff00, 0x0000000000ff0000, 0x00000000ff000000,
    0x000000ff00000000, 0x0000ff0000000000, 0x00ff000000000000, 0xff00000000000000,
];

/// Precomputed diagonal masks
pub const MASK_DIAG: [Bitboard; 15] = [
    0x0000000000000080, 0x0000000000008040, 0x0000000000804020, 0x0000000080402010,
    0x0000008040201008, 0x0000804020100804, 0x0080402010080402, 0x8040201008040201,
    0x4020100804020100, 0x2010080402010000, 0x1008040201000000, 0x0804020100000000,
    0x0402010000000000, 0x0201000000000000, 0x0100000000000000,
];

/// Precomputed anti-diagonal masks
pub const MASK_ANTI_DIAG: [Bitboard; 15] = [
    0x0000000000000001, 0x0000000000000102, 0x0000000000010204, 0x0000000001020408,
    0x0000000102040810, 0x0000010204081020, 0x0001020408102040, 0x0102040810204080,
    0x0204081020408000, 0x0408102040800000, 0x0810204080000000, 0x1020408000000000,
    0x2040800000000000, 0x4080000000000000, 0x8000000000000000,
];

/// Precomputed square masks
pub const SQUARE_BB: [Bitboard; 65] = [
    0x1, 0x2, 0x4, 0x8,
    0x10, 0x20, 0x40, 0x80,
    0x100, 0x200, 0x400, 0x800,
    0x1000, 0x2000, 0x4000, 0x8000,
    0x10000, 0x20000, 0x40000, 0x80000,
    0x100000, 0x200000, 0x400000, 0x800000,
    0x1000000, 0x2000000, 0x4000000, 0x8000000,
    0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x100000000, 0x200000000, 0x400000000, 0x800000000,
    0x1000000000, 0x2000000000, 0x4000000000, 0x8000000000,
    0x10000000000, 0x20000000000, 0x40000000000, 0x80000000000,
    0x100000000000, 0x200000000000, 0x400000000000, 0x800000000000,
    0x1000000000000, 0x2000000000000, 0x4000000000000, 0x8000000000000,
    0x10000000000000, 0x20000000000000, 0x40000000000000, 0x80000000000000,
    0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000,
    0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000,
    0x0
];

pub type MoveFlags = u8;

/// Quiet move; no capture
pub const QUIET: MoveFlags = 0b0000;

/// Double pawn push; always quiet
pub const DOUBLE_PUSH: MoveFlags = 0b0001;

/// Short castles (kingside); always quiet
pub const OO: MoveFlags = 0b0010;

/// Long castles (queenside); always quiet
pub const OOO: MoveFlags = 0b0011;

/// Flag for a capture; all captures will have the 4th bit set
pub const CAPTURE: MoveFlags = 0b1000; // all captures will have this bit set

// Removed CAPTURES, it did not seem to make sense

/// Whether move was en passant
pub const EN_PASSANT: MoveFlags = 0b1010;

/// all quiet promotions
pub const PROMOTIONS: MoveFlags = 0b0111; 

/// all capture promotions
pub const PROMOTION_CAPTURES: MoveFlags = 0b1111;

// quiet promotions
/// Promotion to queen; quiet
pub const PQ_QUEEN: MoveFlags = 0b0100;
/// Promotion to rook; quiet
pub const PQ_ROOK: MoveFlags = 0b0101;
/// Promotion to bishop; quiet
pub const PQ_BISHOP: MoveFlags = 0b0110;
/// Promotion to knight; quiet
pub const PQ_KNIGHT: MoveFlags = 0b0111;

// capture promotions
/// Promotion to queen; capture
pub const PC_QUEEN: MoveFlags = 0b1100;
/// Promotion to rook; capture
pub const PC_ROOK: MoveFlags = 0b1101;
/// Promotion to bishop; capture
pub const PC_BISHOP: MoveFlags = 0b1110;
/// Promotion to knight; capture
pub const PC_KNIGHT: MoveFlags = 0b1111;

pub type Move = u16;

pub const fn create_move(from: Square, to: Square, flags: MoveFlags) -> Move {
    ((flags as u16) << 12) | ((from as u16) << 6) | to as u16
}

pub const fn get_to(m: Move) -> Square {
    (m & 0x3F) as Square
}
pub const fn get_from(m: Move) -> Square {
    ((m >> 6) & 0x3f) as Square
}
pub const fn get_flags(m: Move) -> MoveFlags {
    ((m >> 12) & 0xf) as MoveFlags
}

pub const fn move_is_capture(m: Move) -> bool {
    get_flags(m) & CAPTURE != 0
}

pub const fn move_is_promotion(m: Move) -> bool {
    get_flags(m) & 0b0100 != 0
}

pub const fn get_promotion_type(m: Move) -> PieceType {
    match get_flags(m) {
        PQ_QUEEN | PC_QUEEN => QUEEN,
        PQ_ROOK | PC_ROOK => ROOK,
        PQ_BISHOP | PC_BISHOP => BISHOP,
        PQ_KNIGHT | PC_KNIGHT => KNIGHT,
        _ => PIECE_TYPE_NONE
    }
}

pub fn make<const FLAGS: MoveFlags>(from: Square, to: Bitboard, list: &mut MoveList) {
    let mut bb = to;
    while bb > 0 {
        let to: Square = pop_lsb(&mut bb);

        match FLAGS {
            PROMOTIONS => {
                list.push(create_move(from, to, PQ_QUEEN));
                list.push(create_move(from, to, PQ_ROOK));
                list.push(create_move(from, to, PQ_BISHOP));
                list.push(create_move(from, to, PQ_KNIGHT));
            },
            PROMOTION_CAPTURES => {
                list.push(create_move(from, to, PC_QUEEN));
                list.push(create_move(from, to, PC_ROOK));
                list.push(create_move(from, to, PC_BISHOP));
                list.push(create_move(from, to, PC_KNIGHT));
            }
            _ => {
                list.push(create_move(from, to, FLAGS));
            }
        }
    }
}

pub fn move_from_str(m: String) -> Result<Move, String> {
    if m.len() != 4 { return Err("incorrect length".to_string()); }
    let start_sq = &m[0..2];
    let end_sq = &m[2..4];
    Ok(create_move(
        square_from_str(start_sq)?,
        square_from_str(end_sq)?,
    0
    ))
}

const MOVE_FLAG_STR: [&str; 16] = [
    "", "", "O-O", "O-O-O", "=Q", "=R", "=B", "=N", "(capture)", "", "e.p.", "",
    "=Q (capture)", "=R (capture)", "=B (capture)", "=N (capture)"
];

pub fn print_move(m: Move) {
    print!("{}-{} {}", SQUARE_STR[get_from(m) as usize], SQUARE_STR[get_to(m) as usize], MOVE_FLAG_STR[get_flags(m) as usize])
}

const MAX_MOVES: usize = 218;
pub struct MoveList {
    moves: [Move; MAX_MOVES],
    pub size: usize
}

impl MoveList {
    pub fn new() -> Self {
        MoveList { moves: [0; MAX_MOVES], size: 0 }
    }
    
    pub fn push(&mut self, m: Move) {
        self.moves[self.size] = m;
        self.size += 1;
    }

    pub fn to_vec(self) -> Vec<Move> {
        self.moves[0..self.size].to_vec()
    }
}

impl Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size);
        &self.moves[index]
    }
}

/// Mask for white king (E1) and white kingside rook (H1)
pub const WHITE_OO_MASK: Bitboard = 0x90;

/// Mask for white king (E1) and white queenside rook (A1)
pub const WHITE_OOO_MASK: Bitboard = 0x11;

/// Mask for squares between the white king and white kingside rook (F1,G1)
pub const WHITE_OO_BLOCKERS_MASK: Bitboard = 0x60;

/// Mask for squares between the white king and white queenside rook (B1,C1,D1)
pub const WHITE_OOO_BLOCKERS_MASK: Bitboard = 0xe;

/// Mask for black king (E1) and black kingside rook (H1)
pub const BLACK_OO_MASK: Bitboard = 0x9000000000000000;

/// Mask for black king (E1) and black queenside rook (A1)
pub const BLACK_OOO_MASK: Bitboard = 0x1100000000000000;

/// Mask for squares between the black king and black kingside rook (F1,G1)
pub const BLACK_OO_BLOCKERS_MASK: Bitboard = 0x6000000000000000;

/// Mask for squares between the black king and black queenside rook (B1,C1,D1)
pub const BLACK_OOO_BLOCKERS_MASK: Bitboard = 0xE00000000000000;

/// Mask for all kings and rooks
pub const ALL_CASTLING_MASK: Bitboard = 0x9100000000000091;

pub const fn oo_mask<const COLOR: Color>() -> Bitboard {
    if COLOR == WHITE { WHITE_OO_MASK } else { BLACK_OO_MASK }
}

pub const fn ooo_mask<const COLOR: Color>() -> Bitboard {
    if COLOR == WHITE { WHITE_OOO_MASK } else { BLACK_OOO_MASK }
}

pub const fn oo_blockers_mask<const COLOR: Color>() -> Bitboard {
    if COLOR == WHITE { WHITE_OO_BLOCKERS_MASK } else { BLACK_OO_BLOCKERS_MASK }
}

pub const fn ooo_blockers_mask<const COLOR: Color>() -> Bitboard {
    if COLOR == WHITE { WHITE_OOO_BLOCKERS_MASK } else { BLACK_OOO_BLOCKERS_MASK }
}

pub const fn ooo_ignore_danger<const COLOR: Color>() -> Bitboard {
    if COLOR == WHITE { 0x2 } else { 0x200000000000000 }
}