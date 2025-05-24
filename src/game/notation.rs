use crate::board_rep::board::*;
use crate::board_rep::constants::{Piece, RANK_1, Colour};
use crate::board_rep::bitboard::{self, bitscan_forward, popcount_loop, shift_northx};
use super::game::{SpecialActions, Move};



pub fn read_notation(board: &Board, colour: Colour) -> Option<Move>{
    let mut buf: String = String::default();
    let result = std::io::stdin().read_line(&mut buf);
    match result {
        Ok(_n) => {},
        Err(err) => {
            println!("Error when reading line: {err}");
            return None;
        },
    }
    return interpret_notation(buf, board, colour);
}

pub fn interpret_notation(mut notation: String, board: &Board, colour: Colour) -> Option<Move> {
    let mut special_action: [bool; 5] = [false; 5];

    // Check for captures
    special_action[SpecialActions::Capture as usize] = remove_match(&mut notation, 'x');

    // Check for Check
    special_action[SpecialActions::Check as usize] = remove_match(&mut notation, '+');

    // Check for Checkmate
    special_action[SpecialActions::CheckMate as usize] = remove_match(&mut notation, '#');

    // Castle checker
    if notation == String::from("O-O") {
        special_action[SpecialActions::Castle as usize] = true;
        return Some(Move{special_actions: special_action, org_square: 0, tgt_square: 0, piece: Piece::King, colour: colour});
    }
    // Long Castle checker
    if notation == String::from("O-O-O") {
        special_action[SpecialActions::LongCastle as usize] = true;
        return Some(Move{special_actions: special_action, org_square: 0, tgt_square: 0, piece: Piece::King, colour: colour});
    }

    // Start parsing the string
    let mut it = notation.chars();
    let mut next_char: char = match it.next() {
        Some(char) => char,
        None => return None,
    };
    
    // Get piece
    let piece: Piece = match_piece(next_char);

    // Pawn is expressed by there being no piece character so only get next char if it is not a pawn
    if piece != Piece::Pawn {
        next_char = match it.next() {
            Some(char) => char,
            None => return None,
        };
    }
    
    let mut origin_rank: Option<u64> = None;

    // Find target file
    let tgt_file = if match_file(next_char) <= 7 {
        match_file(next_char)
    } 
    else {
        origin_rank = Some(match_rank(next_char));
        next_char = match it.next() {
            Some(char) => char,
            None => return None,
        };
        match_file(next_char)
    };
    
    if tgt_file > 7 {return None;}

    next_char = match it.next() {
        Some(char) => char,
        None => return None,
    };

    // Find target rank
    let tgt_rank = match_rank(next_char);
    if tgt_rank > 7 {return None;}

    let tgt_index = bitboard::index(tgt_rank, tgt_file);

    let opp_pieces = board.get_colour(!colour);
    let occupied = all(board);
    let empty = !occupied;

    
    // Find all possible pieces of the specified type that can move to the target square
    // Because Non-Pawns are reversible you can just run the normal targets functions using the target square in place of the piece square
    // (Ie. checking if a piece can attack a square is equivalent to checking if a square can attack a piece) 
    let possible_pieces = match piece {
        Piece::Queen => queen_targets(tgt_index, occupied) & board.get_bb(Piece::Queen, colour),
        Piece::Bishop => bishop_targets(tgt_index, occupied) & board.get_bb(Piece::Bishop, colour),
        Piece::King => king_targets(tgt_index) & board.get_bb(Piece::King, colour),
        Piece::Rook => rook_targets(tgt_index, occupied) & board.get_bb(Piece::Rook, colour),
        Piece::Knight => knight_targets(tgt_index) & board.get_bb(Piece::Knight, colour),
        Piece::Pawn => {
            if colour == Colour::White  {white_pawns_target_square(tgt_index, board.pw, opp_pieces, empty)}
            else                        {black_pawns_target_square(tgt_index, board.pb, opp_pieces, empty)}
        }
    };

    // Get the origin / from / source index
    let org_index: usize = match popcount_loop(possible_pieces){
        0 => return None,
        1 => bitscan_forward(possible_pieces),
        _ => {
            let rank = match origin_rank {
                Some(rank) => rank,
                None => return None,
            };
            
            let possible_pieces_onrank = possible_pieces & shift_northx(RANK_1, rank);
            match popcount_loop(possible_pieces_onrank) {
                0 => return None,
                1 => bitscan_forward(possible_pieces_onrank),
                _ => return None,
            }
        }
    };

    if (1<<tgt_index) & opp_pieces != 0 {
        special_action[SpecialActions::Capture as usize] = true;
    }

    return Some(Move{special_actions: special_action, org_square: org_index, tgt_square: tgt_index, piece: piece, colour: colour});
}

fn remove_match(str: &mut String, char: char) -> bool {
    match str.find(char) {
        Some(index) => {str.remove(index); true},
        None => false,
    }
}

fn match_piece(char: char) -> Piece {
    match char {
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'r' => Piece::Rook,
        'q' => Piece::Queen,
        'k' => Piece::King,
        _ => Piece::Pawn
    }
}

fn match_file(char: char) -> u64 {
    match char {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        _ => 8
    }
}

fn match_rank(char: char) -> u64 {
    match char {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => 8,
    }
}