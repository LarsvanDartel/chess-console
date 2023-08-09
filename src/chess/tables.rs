use super::types::*;

/**
 * file:          tables.h
 * author:        Lars van Dartel
 * date:          2023-08-04
 * version:       1.0
 * 
 * This file contains the precomputed lookup tables for piece moves
 */

pub const KING_ATTACKS: [Bitboard; N_SQUARES] = [
    0x302, 0x705, 0xe0a, 0x1c14,
    0x3828, 0x7050, 0xe0a0, 0xc040,
    0x30203, 0x70507, 0xe0a0e, 0x1c141c,
    0x382838, 0x705070, 0xe0a0e0, 0xc040c0,
    0x3020300, 0x7050700, 0xe0a0e00, 0x1c141c00,
    0x38283800, 0x70507000, 0xe0a0e000, 0xc040c000,
    0x302030000, 0x705070000, 0xe0a0e0000, 0x1c141c0000,
    0x3828380000, 0x7050700000, 0xe0a0e00000, 0xc040c00000,
    0x30203000000, 0x70507000000, 0xe0a0e000000, 0x1c141c000000,
    0x382838000000, 0x705070000000, 0xe0a0e0000000, 0xc040c0000000,
    0x3020300000000, 0x7050700000000, 0xe0a0e00000000, 0x1c141c00000000,
    0x38283800000000, 0x70507000000000, 0xe0a0e000000000, 0xc040c000000000,
    0x302030000000000, 0x705070000000000, 0xe0a0e0000000000, 0x1c141c0000000000,
    0x3828380000000000, 0x7050700000000000, 0xe0a0e00000000000, 0xc040c00000000000,
    0x203000000000000, 0x507000000000000, 0xa0e000000000000, 0x141c000000000000,
    0x2838000000000000, 0x5070000000000000, 0xa0e0000000000000, 0x40c0000000000000,
];

pub const KNIGHT_ATTACKS: [Bitboard; N_SQUARES] = [
    0x20400, 0x50800, 0xa1100, 0x142200,
    0x284400, 0x508800, 0xa01000, 0x402000,
    0x2040004, 0x5080008, 0xa110011, 0x14220022,
    0x28440044, 0x50880088, 0xa0100010, 0x40200020,
    0x204000402, 0x508000805, 0xa1100110a, 0x1422002214,
    0x2844004428, 0x5088008850, 0xa0100010a0, 0x4020002040,
    0x20400040200, 0x50800080500, 0xa1100110a00, 0x142200221400,
    0x284400442800, 0x508800885000, 0xa0100010a000, 0x402000204000,
    0x2040004020000, 0x5080008050000, 0xa1100110a0000, 0x14220022140000,
    0x28440044280000, 0x50880088500000, 0xa0100010a00000, 0x40200020400000,
    0x204000402000000, 0x508000805000000, 0xa1100110a000000, 0x1422002214000000,
    0x2844004428000000, 0x5088008850000000, 0xa0100010a0000000, 0x4020002040000000,
    0x400040200000000, 0x800080500000000, 0x1100110a00000000, 0x2200221400000000,
    0x4400442800000000, 0x8800885000000000, 0x100010a000000000, 0x2000204000000000,
    0x4020000000000, 0x8050000000000, 0x110a0000000000, 0x22140000000000,
    0x44280000000000, 0x0088500000000000, 0x0010a00000000000, 0x20400000000000
];

pub const WHITE_PAWN_ATTACKS: [Bitboard; N_SQUARES] = [
    0x200, 0x500, 0xa00, 0x1400,
    0x2800, 0x5000, 0xa000, 0x4000,
    0x20000, 0x50000, 0xa0000, 0x140000,
    0x280000, 0x500000, 0xa00000, 0x400000,
    0x2000000, 0x5000000, 0xa000000, 0x14000000,
    0x28000000, 0x50000000, 0xa0000000, 0x40000000,
    0x200000000, 0x500000000, 0xa00000000, 0x1400000000,
    0x2800000000, 0x5000000000, 0xa000000000, 0x4000000000,
    0x20000000000, 0x50000000000, 0xa0000000000, 0x140000000000,
    0x280000000000, 0x500000000000, 0xa00000000000, 0x400000000000,
    0x2000000000000, 0x5000000000000, 0xa000000000000, 0x14000000000000,
    0x28000000000000, 0x50000000000000, 0xa0000000000000, 0x40000000000000,
    0x200000000000000, 0x500000000000000, 0xa00000000000000, 0x1400000000000000,
    0x2800000000000000, 0x5000000000000000, 0xa000000000000000, 0x4000000000000000,
    0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0,
];

pub const BLACK_PAWN_ATTACKS: [Bitboard; N_SQUARES] = [
    0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0,
    0x2, 0x5, 0xa, 0x14,
    0x28, 0x50, 0xa0, 0x40,
    0x200, 0x500, 0xa00, 0x1400,
    0x2800, 0x5000, 0xa000, 0x4000,
    0x20000, 0x50000, 0xa0000, 0x140000,
    0x280000, 0x500000, 0xa00000, 0x400000,
    0x2000000, 0x5000000, 0xa000000, 0x14000000,
    0x28000000, 0x50000000, 0xa0000000, 0x40000000,
    0x200000000, 0x500000000, 0xa00000000, 0x1400000000,
    0x2800000000, 0x5000000000, 0xa000000000, 0x4000000000,
    0x20000000000, 0x50000000000, 0xa0000000000, 0x140000000000,
    0x280000000000, 0x500000000000, 0xa00000000000, 0x400000000000,
    0x2000000000000, 0x5000000000000, 0xa000000000000, 0x14000000000000,
    0x28000000000000, 0x50000000000000, 0xa0000000000000, 0x40000000000000,
];

pub const fn sliding_attacks(sq: Square, occ: Bitboard, mask: Bitboard) -> Bitboard {
    ((mask & occ).overflowing_sub(SQUARE_BB[sq as usize].overflowing_mul(2).0).0 ^ 
    (mask & occ).reverse_bits().overflowing_sub(SQUARE_BB[sq as usize].reverse_bits().overflowing_mul(2).0).0.reverse_bits()) & mask
}

pub const fn calculate_rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    sliding_attacks(sq, occ, MASK_FILE[file_of(sq) as usize]) |
    sliding_attacks(sq, occ, MASK_RANK[rank_of(sq) as usize])
}

pub const fn calculate_bishop_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    sliding_attacks(sq, occ, MASK_DIAG[diag_of(sq) as usize]) |
    sliding_attacks(sq, occ, MASK_ANTI_DIAG[anti_diag_of(sq) as usize])
}

struct Magic {
    pub start_idx: usize,
    pub mask: Bitboard,
    pub magic: Bitboard,
    pub shift: usize,
}

const ROOK_MAGIC: [Magic; N_SQUARES] = [
    Magic { start_idx: 00000, mask: 0x000101010101017e, magic: 0x8180008240021020, shift: 52},
    Magic { start_idx: 04096, mask: 0x000202020202027c, magic: 0x0040002001403000, shift: 53},
    Magic { start_idx: 06144, mask: 0x000404040404047a, magic: 0x0900284020003101, shift: 53},
    Magic { start_idx: 08192, mask: 0x0008080808080876, magic: 0x8a00160008201040, shift: 53},
    Magic { start_idx: 10240, mask: 0x001010101010106e, magic: 0x32000200081020c4, shift: 53},
    Magic { start_idx: 12288, mask: 0x002020202020205e, magic: 0x0100080100420400, shift: 53},
    Magic { start_idx: 14336, mask: 0x004040404040403e, magic: 0x0400100801020084, shift: 53},
    Magic { start_idx: 16384, mask: 0x008080808080807e, magic: 0x2200010422820044, shift: 52},
    Magic { start_idx: 20480, mask: 0x0001010101017e00, magic: 0x0628800040028421, shift: 53},
    Magic { start_idx: 22528, mask: 0x0002020202027c00, magic: 0x0000400ad0006000, shift: 54},
    Magic { start_idx: 23552, mask: 0x0004040404047a00, magic: 0x0001001240200100, shift: 54},
    Magic { start_idx: 24576, mask: 0x0008080808087600, magic: 0x0000800800500482, shift: 54},
    Magic { start_idx: 25600, mask: 0x0010101010106e00, magic: 0x0268808004008800, shift: 54},
    Magic { start_idx: 26624, mask: 0x0020202020205e00, magic: 0x0006001008060004, shift: 54},
    Magic { start_idx: 27648, mask: 0x0040404040403e00, magic: 0x8404000803100402, shift: 54},
    Magic { start_idx: 28672, mask: 0x0080808080807e00, magic: 0x0102000406044381, shift: 53},
    Magic { start_idx: 30720, mask: 0x00010101017e0100, magic: 0x4120248004864000, shift: 53},
    Magic { start_idx: 32768, mask: 0x00020202027c0200, magic: 0x0010054000a00040, shift: 54},
    Magic { start_idx: 33792, mask: 0x00040404047a0400, magic: 0x0800410020001101, shift: 54},
    Magic { start_idx: 34816, mask: 0x0008080808760800, magic: 0x00b0008008008090, shift: 54},
    Magic { start_idx: 35840, mask: 0x00101010106e1000, magic: 0x840e828008000400, shift: 54},
    Magic { start_idx: 36864, mask: 0x00202020205e2000, magic: 0x0254808002000401, shift: 54},
    Magic { start_idx: 37888, mask: 0x00404040403e4000, magic: 0x0204808001000200, shift: 54},
    Magic { start_idx: 38912, mask: 0x00808080807e8000, magic: 0x1000420004006089, shift: 53},
    Magic { start_idx: 40960, mask: 0x000101017e010100, magic: 0xa000400080088030, shift: 53},
    Magic { start_idx: 43008, mask: 0x000202027c020200, magic: 0x4608400080600180, shift: 54},
    Magic { start_idx: 44032, mask: 0x000404047a040400, magic: 0x4302900080802000, shift: 54},
    Magic { start_idx: 45056, mask: 0x0008080876080800, magic: 0x0080080080801000, shift: 54},
    Magic { start_idx: 46080, mask: 0x001010106e101000, magic: 0x3001000500480010, shift: 54},
    Magic { start_idx: 47104, mask: 0x002020205e202000, magic: 0x0086004200500ca8, shift: 54},
    Magic { start_idx: 48128, mask: 0x004040403e404000, magic: 0x1101000100020044, shift: 54},
    Magic { start_idx: 49152, mask: 0x008080807e808000, magic: 0x0001002100068042, shift: 53},
    Magic { start_idx: 51200, mask: 0x0001017e01010100, magic: 0x0040024220800090, shift: 53},
    Magic { start_idx: 53248, mask: 0x0002027c02020200, magic: 0x3000802000804008, shift: 54},
    Magic { start_idx: 54272, mask: 0x0004047a04040400, magic: 0x092a004082003020, shift: 54},
    Magic { start_idx: 55296, mask: 0x0008087608080800, magic: 0x0080080080803001, shift: 54},
    Magic { start_idx: 56320, mask: 0x0010106e10101000, magic: 0x0280804400800801, shift: 54},
    Magic { start_idx: 57344, mask: 0x0020205e20202000, magic: 0x2000800200804400, shift: 54},
    Magic { start_idx: 58368, mask: 0x0040403e40404000, magic: 0x4008010814004210, shift: 54},
    Magic { start_idx: 59392, mask: 0x0080807e80808000, magic: 0x08860280e2000104, shift: 53},
    Magic { start_idx: 61440, mask: 0x00017e0101010100, magic: 0x00008730c0018000, shift: 53},
    Magic { start_idx: 63488, mask: 0x00027c0202020200, magic: 0x9008d00020004000, shift: 54},
    Magic { start_idx: 64512, mask: 0x00047a0404040400, magic: 0x1090402001030030, shift: 54},
    Magic { start_idx: 65536, mask: 0x0008760808080800, magic: 0x0041040810010020, shift: 54},
    Magic { start_idx: 66560, mask: 0x00106e1010101000, magic: 0x0208010010050019, shift: 54},
    Magic { start_idx: 67584, mask: 0x00205e2020202000, magic: 0x0004402010480104, shift: 54},
    Magic { start_idx: 68608, mask: 0x00403e4040404000, magic: 0x0041180950140006, shift: 54},
    Magic { start_idx: 69632, mask: 0x00807e8080808000, magic: 0x0280010880520014, shift: 53},
    Magic { start_idx: 71680, mask: 0x007e010101010100, magic: 0x8001002042108200, shift: 53},
    Magic { start_idx: 73728, mask: 0x007c020202020200, magic: 0x0208201001400240, shift: 54},
    Magic { start_idx: 74752, mask: 0x007a040404040400, magic: 0x00100400200800a0, shift: 54},
    Magic { start_idx: 75776, mask: 0x0076080808080800, magic: 0x804028114201a200, shift: 54},
    Magic { start_idx: 76800, mask: 0x006e101010101000, magic: 0x0000040080080180, shift: 54},
    Magic { start_idx: 77824, mask: 0x005e202020202000, magic: 0x0000800400520080, shift: 54},
    Magic { start_idx: 78848, mask: 0x003e404040404000, magic: 0x0408101805020c00, shift: 54},
    Magic { start_idx: 79872, mask: 0x007e808080808000, magic: 0x0200800059000480, shift: 53},
    Magic { start_idx: 81920, mask: 0x7e01010101010100, magic: 0x000c204080010011, shift: 52},
    Magic { start_idx: 86016, mask: 0x7c02020202020200, magic: 0x0000110280284001, shift: 53},
    Magic { start_idx: 88064, mask: 0x7a04040404040400, magic: 0x1008102042002882, shift: 53},
    Magic { start_idx: 90112, mask: 0x7608080808080800, magic: 0x0308042008500101, shift: 53},
    Magic { start_idx: 92160, mask: 0x6e10101010101000, magic: 0x00ca0048908c200a, shift: 53},
    Magic { start_idx: 94208, mask: 0x5e20202020202000, magic: 0xd00200900401081a, shift: 53},
    Magic { start_idx: 96256, mask: 0x3e40404040404000, magic: 0x2809000200240081, shift: 53},
    Magic { start_idx: 98304, mask: 0x7e80808080808000, magic: 0x0010210034004082, shift: 52},
];


pub const fn get_rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    let Magic { start_idx, mask, magic, shift } = ROOK_MAGIC[sq as usize];
    let mut index = occ & mask;
    index = index.overflowing_mul(magic).0;
    index >>= shift;
    ROOK_ATTACKS[start_idx + index as usize]
}

pub const fn get_xray_rook_attacks(sq: Square, occ: Bitboard, blockers: Bitboard) -> Bitboard {
    let attacks = get_rook_attacks(sq, occ);
    let blockers = blockers & attacks;
    attacks ^ get_rook_attacks(sq, occ ^ blockers)
}

const BISHOP_MAGIC: [Magic; N_SQUARES] = [
    Magic { start_idx: 0000, mask: 0x0040201008040200, magic: 0x0010201090a20041, shift: 58},
    Magic { start_idx: 0064, mask: 0x0000402010080400, magic: 0x68100c030408e400, shift: 59},
    Magic { start_idx: 0096, mask: 0x0000004020100a00, magic: 0x1010048200408000, shift: 59},
    Magic { start_idx: 0128, mask: 0x0000000040221400, magic: 0x2208049100400c40, shift: 59},
    Magic { start_idx: 0160, mask: 0x0000000002442800, magic: 0x0801104008000001, shift: 59},
    Magic { start_idx: 0192, mask: 0x0000000204085000, magic: 0x0611032010000001, shift: 59},
    Magic { start_idx: 0224, mask: 0x0000020408102000, magic: 0x8000482410080004, shift: 59},
    Magic { start_idx: 0256, mask: 0x0002040810204000, magic: 0x0501009800980400, shift: 58},
    Magic { start_idx: 0320, mask: 0x0020100804020000, magic: 0x0000420cb1040302, shift: 59},
    Magic { start_idx: 0352, mask: 0x0040201008040000, magic: 0x0800d20a18012501, shift: 59},
    Magic { start_idx: 0384, mask: 0x00004020100a0000, magic: 0x0808100482004000, shift: 59},
    Magic { start_idx: 0416, mask: 0x0000004022140000, magic: 0x0120080843120002, shift: 59},
    Magic { start_idx: 0448, mask: 0x0000000244280000, magic: 0x0400020210004800, shift: 59},
    Magic { start_idx: 0480, mask: 0x0000020408500000, magic: 0x2420208210400401, shift: 59},
    Magic { start_idx: 0512, mask: 0x0002040810200000, magic: 0x00808e0182205202, shift: 59},
    Magic { start_idx: 0544, mask: 0x0004081020400000, magic: 0x0000006201042180, shift: 59},
    Magic { start_idx: 0576, mask: 0x0010080402000200, magic: 0x0442842802240401, shift: 59},
    Magic { start_idx: 0608, mask: 0x0020100804000400, magic: 0x0120000842040840, shift: 59},
    Magic { start_idx: 0640, mask: 0x004020100a000a00, magic: 0x0c08081000284010, shift: 57},
    Magic { start_idx: 0768, mask: 0x0000402214001400, magic: 0x0008008082004004, shift: 57},
    Magic { start_idx: 0896, mask: 0x0000024428002800, magic: 0x2034000a021a0004, shift: 57},
    Magic { start_idx: 1024, mask: 0x0002040850005000, magic: 0x0023003601190340, shift: 57},
    Magic { start_idx: 1152, mask: 0x0004081020002000, magic: 0x102082c152082000, shift: 59},
    Magic { start_idx: 1184, mask: 0x0008102040004000, magic: 0x0400864164012800, shift: 59},
    Magic { start_idx: 1216, mask: 0x0008040200020400, magic: 0x0090900440820208, shift: 59},
    Magic { start_idx: 1248, mask: 0x0010080400040800, magic: 0x0018060004100200, shift: 59},
    Magic { start_idx: 1280, mask: 0x0020100a000a1000, magic: 0xc080280004004404, shift: 57},
    Magic { start_idx: 1408, mask: 0x0040221400142200, magic: 0x0020080011010420, shift: 55},
    Magic { start_idx: 1920, mask: 0x0002442800284400, magic: 0x2921014004004040, shift: 55},
    Magic { start_idx: 2432, mask: 0x0004085000500800, magic: 0x4000820001006200, shift: 57},
    Magic { start_idx: 2560, mask: 0x0008102000201000, magic: 0x0801010034044500, shift: 59},
    Magic { start_idx: 2592, mask: 0x0010204000402000, magic: 0x84040090044a0190, shift: 59},
    Magic { start_idx: 2624, mask: 0x0004020002040800, magic: 0x6068040402102030, shift: 59},
    Magic { start_idx: 2656, mask: 0x0008040004081000, magic: 0x0016101033040180, shift: 59},
    Magic { start_idx: 2688, mask: 0x00100a000a102000, magic: 0x0000388602100402, shift: 57},
    Magic { start_idx: 2816, mask: 0x0022140014224000, magic: 0xd200200800010104, shift: 55},
    Magic { start_idx: 3328, mask: 0x0044280028440200, magic: 0x0004004090040100, shift: 55},
    Magic { start_idx: 3840, mask: 0x0008500050080400, magic: 0x000110c900020100, shift: 57},
    Magic { start_idx: 3968, mask: 0x0010200020100800, magic: 0x6001024a84320809, shift: 59},
    Magic { start_idx: 4000, mask: 0x0020400040201000, magic: 0x88010a0200012110, shift: 59},
    Magic { start_idx: 4032, mask: 0x0002000204081000, magic: 0x8022080a1800c2c0, shift: 59},
    Magic { start_idx: 4064, mask: 0x0004000408102000, magic: 0x2184120804140221, shift: 59},
    Magic { start_idx: 4096, mask: 0x000a000a10204000, magic: 0x0102001248010400, shift: 57},
    Magic { start_idx: 4224, mask: 0x0014001422400000, magic: 0x810c00314400a800, shift: 57},
    Magic { start_idx: 4352, mask: 0x0028002844020000, magic: 0x0010200410428400, shift: 57},
    Magic { start_idx: 4480, mask: 0x0050005008040200, magic: 0x0401020802000042, shift: 57},
    Magic { start_idx: 4608, mask: 0x0020002010080400, magic: 0x0064104200559200, shift: 59},
    Magic { start_idx: 4640, mask: 0x0040004020100800, magic: 0x0441410401040080, shift: 59},
    Magic { start_idx: 4672, mask: 0x0000020408102000, magic: 0x9a8c108a10100930, shift: 59},
    Magic { start_idx: 4704, mask: 0x0000040810204000, magic: 0x0240410421200020, shift: 59},
    Magic { start_idx: 4736, mask: 0x00000a1020400000, magic: 0x9080009048082040, shift: 59},
    Magic { start_idx: 4768, mask: 0x0000142240000000, magic: 0x0080821722880a40, shift: 59},
    Magic { start_idx: 4800, mask: 0x0000284402000000, magic: 0x1110004028222008, shift: 59},
    Magic { start_idx: 4832, mask: 0x0000500804020000, magic: 0x125c9020020c2000, shift: 59},
    Magic { start_idx: 4864, mask: 0x0000201008040200, magic: 0x8420041000810a10, shift: 59},
    Magic { start_idx: 4896, mask: 0x0000402010080400, magic: 0x2050049800902200, shift: 59},
    Magic { start_idx: 4928, mask: 0x0002040810204000, magic: 0x0200208410011080, shift: 58},
    Magic { start_idx: 4992, mask: 0x0004081020400000, magic: 0x0040002412021020, shift: 59},
    Magic { start_idx: 5024, mask: 0x000a102040000000, magic: 0x1000820880480800, shift: 59},
    Magic { start_idx: 5056, mask: 0x0014224000000000, magic: 0x0058042082420200, shift: 59},
    Magic { start_idx: 5088, mask: 0x0028440200000000, magic: 0x4020008010420200, shift: 59},
    Magic { start_idx: 5120, mask: 0x0050080402000000, magic: 0x2215004018020420, shift: 59},
    Magic { start_idx: 5152, mask: 0x0020100804020000, magic: 0x0208108c10040044, shift: 59},
    Magic { start_idx: 5184, mask: 0x0040201008040200, magic: 0x1028200800810190, shift: 58},
];


pub const fn get_bishop_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    let Magic { start_idx, mask, magic, shift } = BISHOP_MAGIC[sq as usize];
    let mut index = occ & mask;
    index = index.overflowing_mul(magic).0;
    index >>= shift;
    BISHOP_ATTACKS[start_idx + index as usize]
}

pub const fn get_xray_bishop_attacks(sq: Square, occ: Bitboard, blockers: Bitboard) -> Bitboard {
    let attacks = get_bishop_attacks(sq, occ);
    let blockers = blockers & attacks;
    attacks ^ get_bishop_attacks(sq, occ ^ blockers)
}

pub const SQUARES_BETWEEN: [[Bitboard; N_SQUARES]; N_SQUARES] = {
    let mut squares_between = [[0; N_SQUARES]; N_SQUARES];

    let mut sq1 = A1;

    let mut sqs;
    while sq1 <= H8 {
        let mut sq2 = A1;
        while sq2 <= H8 {
            if sq1 == sq2 { sq2 += 1; continue; }
            sqs = SQUARE_BB[sq1 as usize] | SQUARE_BB[sq2 as usize];
            if file_of(sq1) == file_of(sq2) || rank_of(sq1) == rank_of(sq2) {
                squares_between[sq1 as usize][sq2 as usize] =
                    calculate_rook_attacks(sq1, sqs) & calculate_rook_attacks(sq2, sqs);
            } else if diag_of(sq1) == diag_of(sq2) || anti_diag_of(sq1) == anti_diag_of(sq2) {
                squares_between[sq1 as usize][sq2 as usize] =
                    calculate_bishop_attacks(sq1, sqs) & calculate_bishop_attacks(sq2, sqs);
            }
            
            sq2 += 1;
        }
        sq1 += 1;
    }

    squares_between
};

pub const LINE_BETWEEN: [[Bitboard; N_SQUARES]; N_SQUARES] = {
    let mut line_between = [[0; N_SQUARES]; N_SQUARES];

    let mut sq1 = A1;

    while sq1 <= H8 {
        let mut sq2 = A1;
        while sq2 <= H8 {
            if sq1 == sq2 { sq2 += 1; continue; }
            if file_of(sq1) == file_of(sq2) || rank_of(sq1) == rank_of(sq2) {
                line_between[sq1 as usize][sq2 as usize] =
                    calculate_rook_attacks(sq1, 0) & calculate_rook_attacks(sq2, 0) |
                    SQUARE_BB[sq1 as usize] | SQUARE_BB[sq2 as usize];
            } else if diag_of(sq1) == diag_of(sq2) || anti_diag_of(sq1) == anti_diag_of(sq2) {
                line_between[sq1 as usize][sq2 as usize] =
                    calculate_bishop_attacks(sq1, 0) & calculate_bishop_attacks(sq2, 0) |
                    SQUARE_BB[sq1 as usize] | SQUARE_BB[sq2 as usize];}
            
            sq2 += 1;
        }
        sq1 += 1;
    }

    line_between
};

pub const PAWN_ATTACKS: [[Bitboard; N_SQUARES]; N_COLORS] = [ WHITE_PAWN_ATTACKS, BLACK_PAWN_ATTACKS ];
pub const PSEUDO_LEGAL_ATTACKS: [[Bitboard; N_SQUARES]; N_PIECE_TYPES] = {
    let mut pseudo_legal_attacks = [
        KING_ATTACKS,
        [0; N_SQUARES],
        [0; N_SQUARES],
        [0; N_SQUARES],
        KNIGHT_ATTACKS,
        [0; N_SQUARES],
    ];

    let mut sq = A1;

    while sq <= H8 {
        pseudo_legal_attacks[ROOK as usize][sq as usize] = calculate_rook_attacks(sq, 0);
        pseudo_legal_attacks[BISHOP as usize][sq as usize] = calculate_bishop_attacks(sq, 0);
        pseudo_legal_attacks[QUEEN as usize][sq as usize] = 
            pseudo_legal_attacks[ROOK as usize][sq as usize] | pseudo_legal_attacks[BISHOP as usize][sq as usize];

        sq += 1;
    }

    pseudo_legal_attacks
};

pub const fn attacks<const PT: PieceType>(sq: Square, occ: Bitboard) -> Bitboard {
    assert!(PT != PAWN, "use pawn_attacks instead");
    match PT {
        ROOK => get_rook_attacks(sq, occ),
        BISHOP => get_bishop_attacks(sq, occ),
        QUEEN => attacks::<ROOK>(sq, occ) | attacks::<BISHOP>(sq, occ),
        _ => PSEUDO_LEGAL_ATTACKS[PT as usize][sq as usize]
    }
}

pub const fn get_attacks(pt: PieceType, sq: Square, occ: Bitboard) -> Bitboard {
    match pt {
        ROOK => attacks::<ROOK>(sq, occ),
        BISHOP => attacks::<BISHOP>(sq, occ),
        QUEEN => attacks::<QUEEN>(sq, occ),
        _ => PSEUDO_LEGAL_ATTACKS[pt as usize][sq as usize]
    }
}

pub const fn all_pawn_attacks<const COLOR: Color>(pawns: Bitboard) -> Bitboard {
    if COLOR == WHITE {
        shift_bb::<NORTH_WEST>(pawns) | shift_bb::<NORTH_EAST>(pawns)
    } else {
        shift_bb::<SOUTH_WEST>(pawns) | shift_bb::<SOUTH_EAST>(pawns)
    }
}

pub const fn pawn_attacks<const COLOR: Color>(sq: Square) -> u64 {
    PAWN_ATTACKS[COLOR as usize][sq as usize]
}