use graphics::color::BLACK;

use super::bitboard::*;
use super::constants::*;
use std::ops::{BitAnd, BitOr};


#[derive(Clone, Copy)]
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

// impl 
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
            6 =>  self.pb,
            7 =>  self.nb,
            8 =>  self.bb,
            9 =>  self.rb,
            10 => self.qb,
            11 =>  self.kb,
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
            6 => &mut self.pb,
            7 => &mut self.nb,
            8 => &mut self.bb,
            9 => &mut self.rb,
            10 => &mut self.qb,
            11 => &mut self.kb,
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
            Colour::White => white(self),
            Colour::Black => black(self),
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

pub fn rook_targets(square: usize, occupied: BitBoard) -> BitBoard {
    let mut targets = 0;
    targets |= get_ray_targets_pos(occupied, RayDir::N as usize, square);
    targets |= get_ray_targets_pos(occupied, RayDir::E as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::S as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::W as usize, square);
    return targets;
}
#[inline]
pub fn rook_captures(square: usize, occupied: BitBoard, opp_pieces: BitBoard) -> BitBoard {
    return rook_targets(square, occupied) & opp_pieces;
}

pub fn bishop_targets(square: usize, occupied: BitBoard) -> BitBoard {
    let mut targets = 0;
    targets |= get_ray_targets_pos(occupied, RayDir::NE as usize, square);
    targets |= get_ray_targets_pos(occupied, RayDir::NW as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::SE as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::SW as usize, square);
    return targets;
}
#[inline]
pub fn bishop_captures(square: usize, occupied: BitBoard, opp_pieces: BitBoard) -> BitBoard {
    return bishop_targets(square, occupied) & opp_pieces;
}

pub fn queen_targets(square: usize, occupied: BitBoard) -> BitBoard {
    let mut targets = 0;
    targets |= get_ray_targets_pos(occupied, RayDir::N as usize, square);
    targets |= get_ray_targets_pos(occupied, RayDir::E as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::S as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::W as usize, square);
    targets |= get_ray_targets_pos(occupied, RayDir::NE as usize, square);
    targets |= get_ray_targets_pos(occupied, RayDir::NW as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::SE as usize, square);
    targets |= get_ray_targets_neg(occupied, RayDir::SW as usize, square);
    return targets;
}
#[inline]
pub fn queen_captures(square: usize, occupied: BitBoard, opp_pieces: BitBoard) -> BitBoard {
    return queen_targets(square, occupied) & opp_pieces;
}

/** Slow function, should not be used for search */
pub fn targets(square: usize, piece: Piece, colour: Colour, occupied: BitBoard) -> BitBoard {
    match piece {
        Piece::Queen => queen_targets(square, occupied),
        Piece::Bishop => bishop_targets(square, occupied),
        Piece::King => king_targets(square),
        Piece::Rook => rook_targets(square, occupied),
        Piece::Knight => knight_targets(square),
        Piece::Pawn => {
            if colour == Colour::White  {white_pawn_targets(square, occupied)}
            else                        {black_pawn_targets(square, occupied)}
        }
    }
}

/** Returns a bitboard of all pieces that target the specified square (Friendly & Opp) */
pub fn pieces_target_square(square: usize, board: &Board) -> BitBoard {
    let mut bb = 0;
    let occupied = all(board);

    let mut rooks_and_queens = board.qw | board.qb; 
    let mut bishops_and_queens = rooks_and_queens;
    rooks_and_queens |= board.rw | board.rb;
    bishops_and_queens |= board.bw | board.bb;

    bb |= WHITE_PAWN_ATTACKS[square] & board.pb;
    bb |= BLACK_PAWN_ATTACKS[square] & board.pw;
    bb |= KNIGHT_TARGETS[square] & (board.nw | board.nb);
    bb |= KING_TARGETS[square] & (board.kw | board.kb);
    bb |= bishop_targets(square, occupied) & bishops_and_queens;
    bb |= rook_targets(square, occupied) & rooks_and_queens;

    return bb;
}

pub fn attacking_king(board: &Board, colour: Colour) -> BitBoard {
    let king_square = match colour {
        Colour::White => bitscan_forward(board.kw),
        Colour::Black => bitscan_forward(board.kb),
    };
    return pieces_target_square(king_square, board) & board.get_colour(!colour);
}

/** Returns a bitboard of all black pieces that can attack the white king */
#[inline]
pub fn white_in_check(board: &Board) -> BitBoard {
    let king_square = bitscan_forward(board.kw);
    return pieces_target_square(king_square, board) & black(board);
}
/** Returns a bitboard of all white pieces that can attack the black king */
#[inline]
pub fn black_in_check(board: &Board) -> BitBoard {
    let king_square = bitscan_forward(board.kb);
    return pieces_target_square(king_square, board) & white(board);
}
#[inline]
pub fn is_check(board: &Board) -> bool {
    return white_in_check(board) | black_in_check(board) != 0;
}

/** Returns all moves that the white king can make legally */
pub fn legal_white_king_targets(square: usize, board: &Board) -> BitBoard {
    let friendly_pieces = white(board);
    let opp_pieces = !friendly_pieces;
    let mut psudeo_king_moves = KING_TARGETS[square] & opp_pieces;
    let king_attackers = white_in_check(board);

    if king_attackers == 0 {return psudeo_king_moves;}

    let mut legal_king_moves = 0;
    while psudeo_king_moves != 0 {
        let square = pop(&mut psudeo_king_moves);
        if pieces_target_square(square, board) & opp_pieces == 0 {legal_king_moves |= 1<<square};
    }

    return legal_king_moves;
}
/** Returns all moves that the white king can make legally */
pub fn legal_black_king_targets(square: usize, board: &Board) -> BitBoard {
    let friendly_pieces = black(board);
    let opp_pieces = !friendly_pieces;
    let mut psudeo_king_moves = KING_TARGETS[square] & opp_pieces;
    let king_attackers = black_in_check(board);

    if king_attackers == 0 {return psudeo_king_moves;}

    let mut legal_king_moves = 0;
    while psudeo_king_moves != 0 {
        let square = pop(&mut psudeo_king_moves);
        if pieces_target_square(square, board) & opp_pieces == 0 {legal_king_moves |= 1<<square};
    }

    return legal_king_moves;
}

#[inline]
pub fn in_between(square_from: usize, square_to: usize) -> u64 {
    return IN_BETWEEN_RAY[square_from][square_to];
}
#[inline]
pub fn may_move(square_from: usize, square_to: usize, occupied: BitBoard) -> bool {
    return (in_between(square_from, square_to) & occupied) == 0;
}

pub fn legal_targets(square: usize, piece: Piece, colour: Colour, board: &Board) -> BitBoard {
    let occupied = all(board);
    let mut psudeo_targets = targets(square, piece, colour, occupied);
    psudeo_targets &= !board.get_colour(colour);

    if !is_check(board) {return psudeo_targets;}
    
    if piece == Piece::King {
        return match colour {
            Colour::White => legal_white_king_targets(square, board),
            Colour::Black => legal_black_king_targets(square, board),
        };
    }

    let attacking_pieces = match colour {
        Colour::White => white_in_check(board),
        Colour::Black => black_in_check(board),
    };
    let king_square = match colour {
        Colour::White => bitscan_forward(board.kw),
        Colour::Black => bitscan_forward(board.kb),
    };
    let sliding_pieces = attacking_pieces & (board.rw | board.rb | board.bw | board.bb | board.qw | board.qb);
    let capturable_attacking_pieces = attacking_pieces & psudeo_targets;

    let mut legal_targets = 0;    

    let not_friendly_pieces = !board.get_colour(colour);
    legal_targets |= match piece {
        Piece::Knight => knight_targets(king_square) & knight_targets(square) & not_friendly_pieces,
        Piece::Bishop => bishop_targets(king_square, occupied) & bishop_targets(square, occupied) & not_friendly_pieces,
        Piece::Queen => queen_targets(king_square, occupied) & queen_targets(square, occupied) & not_friendly_pieces,
        Piece::Rook => rook_targets(king_square, occupied) & rook_targets(square, occupied) & not_friendly_pieces,
        Piece::Pawn => {
            match colour {
                Colour::White => BLACK_PAWN_ATTACKS[king_square] & WHITE_PAWN_ATTACKS[square] & not_friendly_pieces,
                Colour::Black => WHITE_PAWN_ATTACKS[king_square] & BLACK_PAWN_ATTACKS[square] & not_friendly_pieces,
            }
        },
        Piece::King => panic!("Impossible has happened"),
    };

    match popcount_loop(attacking_pieces) {
        1 => {
            // If there is only one attacking piece and we can capture it add captuing it to targets
            if (reset(attacking_pieces) == 0) && (capturable_attacking_pieces != 0) {
                legal_targets |= capturable_attacking_pieces;
            }
            
            // If the 1 attacking piece is a sliding piece, return all moves that can block that sliding piece from targeting the king
            if sliding_pieces != 0 {
                legal_targets |= in_between(king_square, bitscan_forward(sliding_pieces)) & psudeo_targets;
            }
            return legal_targets;
        }
        _ => {
            return 0;
        }
    };
}

pub fn is_checkmate(board: &Board) -> bool {
    return false;
}

// pub fn white_in_checkmate(board: &Board) -> bool {
//     let mut checked_king_square = 0;
//     let king_square = bitscan_forward(board.kw);
//     if pieces_target_square(king_square, board) != 0 {checked_king_square = king_square}

//     let king_square = bitscan_forward(board.kb);
//     if pieces_target_square(king_square, board) != 0 {checked_king_square = king_square}

//     if checked_king_square != 0 {
//         let king_moves = KING_TARGETS[checked_king_square] & !;
//     }



//     return false;
// }



pub fn get_piece_at_square(square: usize, mut board: Board) -> Option<(Piece, Colour)> {
    let square_bb: u64 = 1<<square;
    board = board & square_bb;

    let colour = match white(&board) {
        0 => Colour::Black,
        _ => Colour::White,
    };
    let piece = if board.pw | board.pb != 0 {Piece::Pawn}
    else if board.nw | board.nb != 0 {Piece::Knight}
    else if board.bw | board.bb != 0 {Piece::Bishop}
    else if board.rw | board.rb != 0 {Piece::Rook}
    else if board.qw | board.qb != 0 {Piece::Queen}
    else if board.kw | board.kb != 0 {Piece::King}
    else {return None};

    
    return Some((piece, colour));
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
