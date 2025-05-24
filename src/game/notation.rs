use crate::board_rep::board::*;
use crate::board_rep::constants::{Piece, RANK_1};
use crate::board_rep::bitboard::{self, bitscan_forward, popcount_loop, shift_northx};

#[derive(Eq, PartialEq)]
enum SpecialActions {
    CASTLE,
    LONG_CASTLE,
    CHECK,
    CHECKMATE,
    CAPTURE,
}
#[derive(Eq, PartialEq)]
enum Colour {
    WHITE,
    BLACK
}

pub fn interpret_notation(mut notation: String, board: &Board, colour: Colour) -> Option<([bool; 5], usize, usize)> {
    let mut special_action: [bool; 5] = [false; 5];
    // Castle checker
    if notation == String::from("O-O") {
        special_action[SpecialActions::CASTLE as usize] = true;
        return Some((special_action, 0, 0));
    }
    // Long Castle checker
    if notation == String::from("O-O-O") {
        special_action[SpecialActions::LONG_CASTLE as usize] = true;
        return Some((special_action, 0, 0));
    }

    // Check for captures
    special_action[SpecialActions::CAPTURE as usize] = remove_match(&mut notation, 'x');

    // Check for Check
    special_action[SpecialActions::CHECK as usize] = remove_match(&mut notation, '+');

    // Check for Checkmate
    special_action[SpecialActions::CHECKMATE as usize] = remove_match(&mut notation, '#');

    // Start parsing the string
    let mut it = notation.chars();
    let mut next_char: char = match it.next() {
        Some(char) => char,
        None => return None,
    };
    
    // Get piece
    let piece: Piece = match_piece(next_char);

    // Pawn is expressed by there being no piece character so only get next char if it is not a pawn
    if piece != Piece::PAWN {
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


    // generating board info
    let my_pieces = match colour {
        Colour::BLACK => black(board),
        Colour::WHITE => white(board),
    };
    let opp_pieces = match colour {
        Colour::BLACK => white(board),
        Colour::WHITE => black(board),
    };
    let occupied = all(board);
    let empty = !occupied;

    // Find all possible pieces of the specified type that can move to the target square
    // Because Non-Pawns are reversible you can just run the normal targets functions using the target square in place of the piece square
    // (Ie. checking if a piece can attack a square is equivalent to checking if a square can attack a piece) 
    let possible_pieces = match piece {
        Piece::QUEEN => queen_targets(tgt_index, occupied) & my_pieces,
        Piece::BISHOP => bishop_targets(tgt_index, occupied) & my_pieces,
        Piece::KING => king_targets(tgt_index) & my_pieces,
        Piece::ROOK => rook_targets(tgt_index, occupied) & my_pieces,
        Piece::KNIGHT => knight_targets(tgt_index) & my_pieces,
        Piece::PAWN => {
            if colour == Colour::WHITE  {white_pawns_target_square(tgt_index, board.pw, opp_pieces, empty)}
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

    return Some((special_action, org_index, tgt_index));
}

fn remove_match(str: &mut String, char: char) -> bool {
    match str.find(char) {
        Some(index) => {str.remove(index); true},
        None => false,
    }
}

fn match_piece(char: char) -> Piece {
    match char {
        'n' => Piece::KNIGHT,
        'b' => Piece::BISHOP,
        'r' => Piece::ROOK,
        'q' => Piece::QUEEN,
        'k' => Piece::KING,
        _ => Piece::PAWN
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