use crate::board_rep::{bitboard::{popcount_loop, popcount_swar}, board::*};
use crate::board_rep::constants::{Colour, Piece};
use crate::game::game::Move;

#[derive(PartialEq, Eq)]
struct SimpleMoves {
    word: Vec<u8>,
    org_square: Vec<u64>,
    tgt_square: Vec<u64>,
}

const PIECE_VALUE: [u64; 6] = [1, 3, 3, 5, 9, 0];

fn white_material_value(board: &Board) -> u64 {
    let mut value = 0;
    value += popcount_swar(board.pw) * PIECE_VALUE[Piece::Pawn as usize];
    value += popcount_loop(board.nw) * PIECE_VALUE[Piece::Knight as usize];
    value += popcount_loop(board.bw) * PIECE_VALUE[Piece::Bishop as usize];
    value += popcount_loop(board.rw) * PIECE_VALUE[Piece::Rook as usize];
    value += popcount_loop(board.qw) * PIECE_VALUE[Piece::Queen as usize];
    return value;
}

fn black_material_value(board: &Board) -> u64 {
    let mut value = 0;
    value += popcount_swar(board.pb) * PIECE_VALUE[Piece::Pawn as usize];
    value += popcount_loop(board.nb) * PIECE_VALUE[Piece::Knight as usize];
    value += popcount_loop(board.bb) * PIECE_VALUE[Piece::Bishop as usize];
    value += popcount_loop(board.rb) * PIECE_VALUE[Piece::Rook as usize];
    value += popcount_loop(board.qb) * PIECE_VALUE[Piece::Queen as usize];
    return value;
}

#[inline]
fn evaluate(board : &Board) -> u64 {
    return white_material_value(board) * (popcount_swar(white(board)) - popcount_swar(black(board)));
}

fn get_all_moves(board: &Board, colour: Colour) -> SimpleMoves {

    return SimpleMoves { word: Vec::new(), org_square: Vec::new(), tgt_square: Vec::new() }
}

pub fn search_best_move(board: &Board) -> Move {
    let mut mov: Move = Move::default();

    let moves = get_all_moves(board, Colour::White);

    let mut i = 0;
    while i < moves.word.len() {

    }
    let best_move = negamax(3, *board);

    return mov; 
}

fn negamax(depth: u64, board: Board) -> u64 {
    // if depth == 0 {
    //     return evaluate(&board);
    // }

    // let mut 

    // for move in moves {
        
    // }
    return 0;
}

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