// USES Little-Endian Rank-File Mapping (LERF)
// a1 -> bit 0
// h1 -> bit 7
// a2 -> bit 8
// h8 -> bit 63

pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

pub type BitBoard = u64;

pub struct Board {
    pub pw: BitBoard,
    pub rw: BitBoard,
    pub nw: BitBoard,
    pub bw: BitBoard,
    pub qw: BitBoard,
    pub kw: BitBoard,
    pub pb: BitBoard,
    pub rb: BitBoard,
    pub nb: BitBoard,
    pub bb: BitBoard,
    pub qb: BitBoard,
    pub kb: BitBoard
}

pub const INITIAL_BOARD: Board = Board {
    pw: 0x000000000000FF00,
    rw: 0x0000000000000081,
    nw: 0x0000000000000042,
    bw: 0x0000000000000024,
    qw: 0x0000000000000010,
    kw: 0x0000000000000008,

    pb: 0x00FF000000000000,
    rb: 0x8100000000000000,
    nb: 0x4200000000000000,
    bb: 0x2400000000000000,
    qb: 0x0800000000000000,
    kb: 0x1000000000000000
};

pub const FILE_A: BitBoard = 0x0101010101010101;
pub const FILE_H: BitBoard = 0x8080808080808080;
pub const RANK_1: BitBoard = 0x00000000000000FF;
pub const RANK_8: BitBoard = 0xFF00000000000000;
pub const DIAG_A1_H8: BitBoard = 0x8040201008040201;
pub const DIAG_H1_A8: BitBoard = 0x0102040810204080;
pub const SQUARE_LIGHT: BitBoard = 0x55AA55AA55AA55AA;
pub const SQUARE_DARK: BitBoard = 0xAA55AA55AA55AA55;

#[inline]
pub fn index(rank: &u64, file: &u64) -> u64{
    return 8*rank + file; // LSF mapping
}
#[inline]
pub fn file(index: &u64) -> u64{
    return index & 7;  // Equivalent to mod 8
}
#[inline]
pub fn rank(index: &u64) -> u64{
    return index >> 3; // Equivalent to dev/8
}
#[inline]
pub fn all(state: &Board) -> BitBoard{
    return state.pw | state.rw | state.nw | state.bw | state.qw | state.kw | state.pb | state.rb | state.nb | state.bb | state.qb | state.kb;
}
#[inline]
pub fn row(board: &BitBoard, row: u64) -> u8{
    return (board >> row*8) as u8;
}