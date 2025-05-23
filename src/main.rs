use chess_engine::board_rep::bitboard;
use chess_engine::board_rep::board::*;
use chess_engine::board_rep::constants;
use chess_engine::board_rep::debug;
use chess_engine::board_rep::constants::*;

fn main() {
    debug::print_board(&INITIAL_BOARD);

    let board: u64 = INITIAL_BOARD.pw;
    println!();
    debug::print_bitboard(board);
    println!();
    let mut board_mut: bitboard::BitBoard = board;
    board_mut = bitboard::south_fill(board);
    debug::print_bitboard(board_mut);

    debug::print_bitboard(KNIGHT_TARGETS[constants::Square::E4 as usize]);
    //println!();
    //debug::print_bitboard(&bitboard::rotate90cw(&INITIAL_BOARD.pw));
}
