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
impl Default for Board {
    fn default() -> Board {
        INITIAL_BOARD
    }
}

pub fn make_board(pw: BitBoard, rw: BitBoard, nw: BitBoard, bw: BitBoard, qw: BitBoard, kw: BitBoard,pb: BitBoard, rb: BitBoard, nb: BitBoard, bb: BitBoard, qb: BitBoard, kb: BitBoard) -> Board{
    let board: Board = Board {
        pw: pw,
        rw: rw,
        nw: nw,
        bw: bw,
        qw: qw,
        kw: kw,
        pb: pb,
        rb: rb,
        nb: nb,
        bb: bb,
        qb: qb,
        kb: kb
    };
    return board;
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
#[inline]
pub fn white(state: &Board) -> BitBoard {
    return state.pw | state.rw | state.nw | state.bw | state.qw | state.kw;
}
#[inline]
pub fn black(state: &Board) -> BitBoard {
    return state.pb | state.rb | state.nb | state.bb | state.qb | state.kb;
}

#[inline]
pub fn white_single_push(white_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return north_one(white_pawns) & empty;
}
#[inline]
pub fn white_double_push(white_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return north_one(white_single_push(white_pawns, empty)) & RANK_4 & empty;
}
#[inline]
pub fn black_single_push(black_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return south_one(black_pawns) & empty;
}
#[inline]
pub fn black_double_push(black_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return south_one(black_single_push(black_pawns, empty)) & RANK_5 & empty;
}



#[inline]
pub fn pawn_attack(state: &Board) -> BitBoard {
    return 0;
}