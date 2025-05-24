use crate::board_rep::{bitboard::{popcount_loop, popcount_swar}, board::*};
use crate::board_rep::constants::Piece;

const PIECE_VALUE: [u64; 6] = [1, 3, 3, 5, 9, 0];

fn white_material_value(board: &Board) -> u64 {
    let mut value = 0;
    value += popcount_swar(board.pw) * PIECE_VALUE[Piece::PAWN as usize];
    value += popcount_loop(board.nw) * PIECE_VALUE[Piece::KNIGHT as usize];
    value += popcount_loop(board.bw) * PIECE_VALUE[Piece::BISHOP as usize];
    value += popcount_loop(board.rw) * PIECE_VALUE[Piece::ROOK as usize];
    value += popcount_loop(board.qw) * PIECE_VALUE[Piece::QUEEN as usize];
    return value;
}

fn black_material_value(board: &Board) -> u64 {
    let mut value = 0;
    value += popcount_swar(board.pb) * PIECE_VALUE[Piece::PAWN as usize];
    value += popcount_loop(board.nb) * PIECE_VALUE[Piece::KNIGHT as usize];
    value += popcount_loop(board.bb) * PIECE_VALUE[Piece::BISHOP as usize];
    value += popcount_loop(board.rb) * PIECE_VALUE[Piece::ROOK as usize];
    value += popcount_loop(board.qb) * PIECE_VALUE[Piece::QUEEN as usize];
    return value;
}

#[inline]
fn evaluate(board : &Board) -> u64 {
    return white_material_value(board) * (popcount_swar(white(board)) - popcount_swar(black(board)));
}

// pub fn negamax(depth: u64, board: Board) -> u64 {
//     if depth == 0 {
//         return evaluate(&board);
//     }

//     for move in moves {
        
//     }

// }

// int negaMax( int depth ) {
//     if ( depth == 0 ) return evaluate();
//     int max = -oo;
//     for ( all moves)  {
//         score = -negaMax( depth - 1 );
//         if( score > max )
//             max = score;
//     }
//     return max;
// }