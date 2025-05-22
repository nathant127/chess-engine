use crate::board_rep::bitboard::*;

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

#[inline]
pub fn all(state: &Board) -> BitBoard{
    return state.pw | state.rw | state.nw | state.bw | state.qw | state.kw | state.pb | state.rb | state.nb | state.bb | state.qb | state.kb;
}