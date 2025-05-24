use crate::board_rep::bitboard::BitBoard;
use crate::board_rep::{board::*, debug};
use crate::board_rep::constants::{Piece, Colour};
#[derive(Eq, PartialEq)]
pub enum SpecialActions {
    CASTLE,
    LONG_CASTLE,
    CHECK,
    CHECKMATE,
    CAPTURE,
}

#[derive(Eq, PartialEq)]
pub struct Move {
    pub special_actions: [bool; 5],
    pub org_square: usize,
    pub tgt_square: usize,
    pub piece: Piece,
    pub colour: Colour,
}


pub fn execute_move(board: &mut Board, mov: &Move) {
    println!("Org Square: {:}", mov.org_square);
    println!("Tgt Square: {:}", mov.tgt_square);
    if is_move_valid(board, mov) == false {
        println!("Move is invalid");
        return;
    }

    let piece_bb: &mut BitBoard = board.get_bb_mut(mov.piece, mov.colour);
    *piece_bb &= !(1<<mov.org_square);
    *piece_bb |= 1<<mov.tgt_square;

}

pub fn is_move_valid(board: &Board, mov: &Move) -> bool {
    let piece_bb: BitBoard = board.get_bb(mov.piece, mov.colour);

    let piece_exists = piece_bb & (1 << mov.org_square) != 0;
    if !piece_exists
        {return false;}

    let occupied = all(board);
    let piece_targets = targets(mov.org_square, mov.piece, mov.colour, occupied);

    let my_pieces = board.get_colour(mov.colour);
    let valid_moves = piece_targets & !my_pieces;

    let can_piece_reach_target = valid_moves & (1<<mov.tgt_square) != 0;
    if !can_piece_reach_target
        {return false;}

    return true;
}