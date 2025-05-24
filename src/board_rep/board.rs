use super::bitboard::*;
use super::constants::*;
use super::debug;
use std::ops::{BitAnd, BitOr};


pub struct Board {
    pub pw: BitBoard,
    pub nw: BitBoard,
    pub bw: BitBoard,
    pub rw: BitBoard,
    pub qw: BitBoard,
    pub kw: BitBoard,

    pub pb: BitBoard,
    pub nb: BitBoard,
    pub bb: BitBoard,
    pub rb: BitBoard,
    pub qb: BitBoard,
    pub kb: BitBoard,
}
impl Default for Board {
    fn default() -> Board {
        INITIAL_BOARD
    }
}

pub trait GetBitBoard {
    fn get_bb(& self, piece: Piece, colour: Colour) -> BitBoard;
}

impl GetBitBoard for Board {
    fn get_bb(&self, piece: Piece, colour: Colour) -> BitBoard {
        let index: usize = (piece * colour) as usize;
        match index {
            0 => self.pw,
            1 =>  self.nw,
            2 =>  self.bw,
            3 =>  self.rw,
            4 =>  self.qw,
            5 =>  self.kw,
            6 =>  self.pw,
            7 =>  self.nw,
            8 =>  self.bw,
            9 =>  self.rw,
            10 => self.qw,
            11 =>  self.kw,
            _ => {panic!("Tried to get a bitboard that doesn't exist")}
        }
    }
}

pub trait GetBitBoardMut {
    fn get_bb_mut(&mut self, piece: Piece, colour: Colour) -> &mut BitBoard;
}

impl GetBitBoardMut for Board {
    fn get_bb_mut(&mut self, piece: Piece, colour: Colour) -> &mut BitBoard {
        let index = (piece as u32) + (colour as u32 * 6);
        match index {
            0 => &mut self.pw,
            1 => &mut self.nw,
            2 => &mut self.bw,
            3 => &mut self.rw,
            4 => &mut self.qw,
            5 => &mut self.kw,
            6 => &mut self.pw,
            7 => &mut self.nw,
            8 => &mut self.bw,
            9 => &mut self.rw,
            10 => &mut self.qw,
            11 => &mut self.kw,
            _ => {panic!("Tried to get a bitboard that doesn't exist")}
        }
    }
}

impl BitAnd<BitBoard> for Board {
    type Output = Self;
    fn bitand(self, rhs: BitBoard) -> Self::Output {
        Self {
            pw: self.pw & rhs,
            bw: self.bw & rhs,
            nw: self.nw & rhs,
            rw: self.rw & rhs,
            qw: self.qw & rhs,
            kw: self.kw & rhs,

            pb: self.pb & rhs,
            bb: self.bb & rhs,
            nb: self.nb & rhs,
            rb: self.rb & rhs,
            qb: self.qb & rhs,
            kb: self.kb & rhs,
        }
    }
}

impl BitOr<BitBoard> for Board {
    type Output = Self;
    fn bitor(self, rhs: BitBoard) -> Self::Output {
        Self {
            pw: self.pw | rhs,
            bw: self.bw | rhs,
            nw: self.nw | rhs,
            rw: self.rw | rhs,
            qw: self.qw | rhs,
            kw: self.kw | rhs,

            pb: self.pb | rhs,
            bb: self.bb | rhs,
            nb: self.nb | rhs,
            rb: self.rb | rhs,
            qb: self.qb | rhs,
            kb: self.kb | rhs,
        }
    }
}

impl Board {
    pub fn get_colour(self: &Self, colour: Colour) -> BitBoard {
        match colour {
            Colour::WHITE => white(self),
            Colour::BLACK => black(self),
        }
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
pub fn white_single_push_targets(white_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return shift_north(white_pawns) & empty;
}
#[inline]
pub fn white_double_push_targets(white_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return shift_north(white_single_push_targets(white_pawns, empty)) & RANK_4 & empty;
}
#[inline]
pub fn black_single_push_targets(black_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return shift_south(black_pawns) & empty;
}
#[inline]
pub fn black_double_push_targets(black_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return shift_south(black_single_push_targets(black_pawns, empty)) & RANK_5 & empty;
}
/** Returns the white pawn push targets (no captures) */
#[inline]
pub fn white_pawn_push_targets(white_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return white_single_push_targets(white_pawns, empty) | white_double_push_targets(white_pawns, empty);
}
/** Returns the black pawn push targets (no captures) */
#[inline]
pub fn black_pawn_push_targets(black_pawns: BitBoard, empty: BitBoard) -> BitBoard {
    return black_single_push_targets(black_pawns, empty) | black_double_push_targets(black_pawns, empty);
}

/** Returns all squares the white pawns are attacking */
#[inline]
pub fn white_pawn_attacks(white_pawns: BitBoard) -> BitBoard {
    return shift_northeast(white_pawns) | shift_northwest(white_pawns);
}
#[inline]
pub fn white_pawn_captures(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    return white_pawn_attacks(white_pawns) & black_pieces;
}
#[inline]
pub fn white_pawn_dbl_atks(white_pawns: BitBoard) -> BitBoard {
    return shift_northeast(white_pawns) & shift_northwest(white_pawns);
}
#[inline]
pub fn white_pawn_single_atks(white_pawns: BitBoard) -> BitBoard {
    return shift_northeast(white_pawns) ^ shift_northwest(white_pawns);
}
/** Returns all squares the black pawns are attacking */
#[inline]
pub fn black_pawn_attacks(black_pawns: BitBoard) -> BitBoard {
    return shift_southeast(black_pawns) | shift_southwest(black_pawns);
}
#[inline]
pub fn black_pawn_captures(black_pawns: BitBoard, white_pieces: BitBoard) -> BitBoard {
    return black_pawn_attacks(black_pawns) & white_pieces;
}
#[inline]
pub fn black_pawn_dbl_atks(black_pawns: BitBoard) -> BitBoard {
    return shift_southeast(black_pawns) & shift_southwest(black_pawns);
}
#[inline]
pub fn black_pawn_single_atks(black_pawns: BitBoard) -> BitBoard {
    return shift_southeast(black_pawns) ^ shift_southwest(black_pawns);
}

#[inline]
pub fn white_pawn_capture(index: usize, black_pieces: BitBoard) -> BitBoard {
    return WHITE_PAWN_ATTACKS[index] & black_pieces;
}
#[inline]
pub fn black_pawn_capture(index: usize, white_pieces: BitBoard) -> BitBoard {
    return BLACK_PAWN_ATTACKS[index] & white_pieces;
}

#[inline]
pub fn white_pawns_ableto_capture(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    return white_pawns & black_pawn_attacks(black_pieces);
}
#[inline]
pub fn black_pawns_ableto_capture(black_pawns: BitBoard, white_pieces: BitBoard) -> BitBoard {
    return black_pawns & white_pawn_attacks(white_pieces);
}

/** Returns all targets for a white pawn at the specified square including diagonal defended squares */
pub fn white_pawn_targets(square: usize, occupied: BitBoard) -> BitBoard {
    let mut bb = 0;
    let pos = 1<< square;
    let empty = !occupied;

    bb |= white_pawn_captures(pos, occupied);
    bb |= white_pawn_push_targets(pos, empty);

    return bb;
}
/** Returns all targets for a black pawn at the specified square including diagonal defended squares */
pub fn black_pawn_targets(square: usize, occupied: BitBoard) -> BitBoard {
    let mut bb = 0;
    let pos = 1<< square;
    let empty = !occupied;

    bb |= black_pawn_captures(pos, occupied);
    bb |= black_pawn_push_targets(pos, empty);

    return bb;
}

/** Returns the white pawns that can move to a specified square */
pub fn white_pawns_target_square(square: usize, white_pawns: BitBoard, black_pieces: BitBoard, empty: BitBoard) -> BitBoard {
    let square_bb = 1<<square;
    let is_capture = if square_bb & black_pieces == 0 {false} else {true};
    
    if is_capture {
        black_pawn_attacks(square_bb) & white_pawns
    }
    else {
        // Returns non zero only if there is a white pawn that can target the square
        let is_valid_pawn = square_bb & (white_pawn_push_targets(white_pawns, empty)) != 0;
        if is_valid_pawn == false 
            {0}
        else if shift_south(square_bb) & white_pawns != 0 
            {shift_south(square_bb)}
        else 
            {shift_southdbl(square_bb)}
    }
}

/** Returns the black pawns that can move to a specified square */
pub fn black_pawns_target_square(square: usize, black_pawns: BitBoard, white_pieces: BitBoard, empty: BitBoard) -> BitBoard {
    let square_bb = 1<<square;
    let is_capture = if square_bb & white_pieces == 0 {false} else {true};
    
    if is_capture {
        white_pawn_attacks(square_bb) & black_pawns
    }
    else {
        // Returns non zero only if there is a white pawn that can target the square
        let is_valid_pawn = square_bb & (black_pawn_push_targets(black_pawns, empty)) != 0;
        if is_valid_pawn == false 
            {0}
        else if shift_north(square_bb) & black_pawns != 0 
            {shift_north(square_bb)}
        else 
            {shift_northdbl(square_bb)}
    }
}

#[inline]
pub fn knight_targets(index: usize) -> BitBoard {
    return KNIGHT_TARGETS[index];
}
#[inline]
pub fn knight_captures(index: usize, opp_pieces: BitBoard) -> BitBoard {
    return knight_targets(index) & opp_pieces;
}

/**
 * TODO: Add check rules
 */
#[inline]
pub fn king_targets(index: usize) -> BitBoard {
    return KING_TARGETS[index];
}
#[inline]
pub fn king_captures(index: usize, opp_pieces: BitBoard) -> BitBoard {
    return king_targets(index) & opp_pieces;
}

/**
 * Should only be used for directions N, NE, E, NW (pos directions)
 * NOTE: Will return a bitboard that includes the location of all blockers (both white and black pieces)
 */
fn get_ray_targets_pos(occupied: BitBoard, dir: usize, index: usize) -> BitBoard {
    let mut ray = RAY_TARGETS[dir][index];
    let blockers = ray & occupied;
    if blockers != 0 {
        let bitscan = bitscan_forward(blockers);
        ray ^= RAY_TARGETS[dir][bitscan as usize];
    }
    return ray;
}

/**
 * Should only be used for directions S, SE, SW, W (neg directions)
 * NOTE: Will return a bitboard that includes the location of all blockers (both white and black pieces)
 */
fn get_ray_targets_neg(occupied: BitBoard, dir: usize, index: usize) -> BitBoard {
    let mut ray = RAY_TARGETS[dir][index];
    let blockers = ray & occupied;
    if blockers != 0 {
        let bitscan = bitscan_reverse(blockers);
        ray ^= RAY_TARGETS[dir][bitscan as usize];
    }
    return ray;
}

pub fn rook_targets(index: usize, occupied: BitBoard) -> BitBoard {
    let mut targets = 0;
    targets |= get_ray_targets_pos(occupied, RayDir::N as usize, index);
    targets |= get_ray_targets_pos(occupied, RayDir::E as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::S as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::W as usize, index);
    return targets;
}
#[inline]
pub fn rook_captures(index: usize, occupied: BitBoard, opp_pieces: BitBoard) -> BitBoard {
    return rook_targets(index, occupied) & opp_pieces;
}

pub fn bishop_targets(index: usize, occupied: BitBoard) -> BitBoard {
    let mut targets = 0;
    targets |= get_ray_targets_pos(occupied, RayDir::NE as usize, index);
    targets |= get_ray_targets_pos(occupied, RayDir::NW as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::SE as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::SW as usize, index);
    return targets;
}
#[inline]
pub fn bishop_captures(index: usize, occupied: BitBoard, opp_pieces: BitBoard) -> BitBoard {
    return bishop_targets(index, occupied) & opp_pieces;
}

pub fn queen_targets(index: usize, occupied: BitBoard) -> BitBoard {
    let mut targets = 0;
    targets |= get_ray_targets_pos(occupied, RayDir::N as usize, index);
    targets |= get_ray_targets_pos(occupied, RayDir::E as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::S as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::W as usize, index);
    targets |= get_ray_targets_pos(occupied, RayDir::NE as usize, index);
    targets |= get_ray_targets_pos(occupied, RayDir::NW as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::SE as usize, index);
    targets |= get_ray_targets_neg(occupied, RayDir::SW as usize, index);
    return targets;
}
#[inline]
pub fn queen_captures(index: usize, occupied: BitBoard, opp_pieces: BitBoard) -> BitBoard {
    return queen_targets(index, occupied) & opp_pieces;
}

/** Slow function, should not be used for search */
pub fn targets(square: usize, piece: Piece, colour: Colour, occupied: BitBoard) -> BitBoard {
    match piece {
        Piece::QUEEN => queen_targets(square, occupied),
        Piece::BISHOP => bishop_targets(square, occupied),
        Piece::KING => king_targets(square),
        Piece::ROOK => rook_targets(square, occupied),
        Piece::KNIGHT => knight_targets(square),
        Piece::PAWN => {
            if colour == Colour::WHITE  {white_pawn_targets(square, occupied)}
            else                        {black_pawn_targets(square, occupied)}
        }
    }
}

pub fn all_white_moves(board: &Board) -> [BitBoard; 64] {
    let mut moves: [BitBoard; 64] = [0; 64];
    let occupied: u64 = all(board);
    let white_pieces: u64 = white(board);
    let not_white_pieces: u64 = !white_pieces;
    let black_pieces: u64 = black(board);

    let mut index: usize = bitscan_forward(board.qw);
    moves[index] = queen_targets(index, occupied) & not_white_pieces;

    index = bitscan_forward(board.rw);
    moves[index] = rook_targets(index, occupied) & not_white_pieces;
    index = bitscan_reverse(board.rw);
    moves[index] = rook_targets(index, occupied) & not_white_pieces;

    for (i, &mut bb) in moves.iter_mut().enumerate() {
        let pos: u64 = 1<<i;


    }

    return moves;
}
