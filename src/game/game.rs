use crate::board_rep::bitboard::{BitBoard, row, flip_h, flip_v};
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

pub fn print_board(board: &Board) {
    let mut char_arrs: [[char; 64]; 6] = [['0'; 64]; 6];
    //print!("{}[2J", 27 as char);

    char_arrs[0] = fmt_bitboard_char(board.pw | board.pb, 'p');
    char_arrs[1] = fmt_bitboard_char(board.nw | board.nb, 'n');
    char_arrs[2] = fmt_bitboard_char(board.bw | board.bb, 'b');
    char_arrs[3] = fmt_bitboard_char(board.rw | board.rb, 'r');
    char_arrs[4] = fmt_bitboard_char(board.qw | board.qb, 'q');
    char_arrs[5] = fmt_bitboard_char(board.kw | board.kb, 'k');

    let mut i: usize = 0;
    let mut output = ['0'; 64];
    while i < char_arrs[0].len() {
        for char_arr in char_arrs {
            if char_arr[i] != '0' {
                output[i] = char_arr[i]; 
                break;
            }
        }
        i += 1;
    }
    
    i = 0;
    while i < 8 {
        let mut j: usize = 0;
        while j < 8 {
            print!("{:}", output[((7-i)*8)+j]);
            j += 1;
        }
        println!();
        i += 1;
    }
    
}

pub fn fmt_bitboard_char(board: BitBoard, disp_char: char) -> [char; 64]{
    let mut arr: [char; 64] = ['0'; 64];

    let mut i: usize = 0;
    while i < arr.len() {
        if (1<<i) & board != 0 {
            arr[i] = disp_char;
        }
        i += 1;
    }
    return arr;
}


pub fn execute_move(board: &mut Board, mov: &Move) {
    if is_move_valid(board, mov) == false {
        println!("Move is invalid");
        return;
    }

    if mov.special_actions[SpecialActions::CAPTURE as usize] == true {
        *board = *board & !(1<<mov.tgt_square);
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