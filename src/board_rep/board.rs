use crate::board_rep::bitboard::*;
use crate::board_rep::constants::*;

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
    return shift_north(white_pawns) & empty;
}
#[inline]
pub fn white_double_push(white_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return shift_north(white_single_push(white_pawns, empty)) & RANK_4 & empty;
}
#[inline]
pub fn black_single_push(black_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return shift_south(black_pawns) & empty;
}
#[inline]
pub fn black_double_push(black_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return shift_south(black_single_push(black_pawns, empty)) & RANK_5 & empty;
}



#[inline]
pub fn white_pawn_attack(index: usize, black_pieces: BitBoard) -> BitBoard {
    return WHITE_PAWN_ATTACKS[index] & black_pieces;
}
#[inline]
pub fn black_pawn_attack(index: usize, white_pieces: BitBoard) -> BitBoard {
    return BLACK_PAWN_ATTACKS[index] & white_pieces;
}