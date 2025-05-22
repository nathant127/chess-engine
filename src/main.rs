use chess_engine::board_rep::bitboard;
use chess_engine::board_rep::board::*;
use chess_engine::board_rep::debug;

fn main() {
    debug::print_board(&INITIAL_BOARD);

    let board: u64 = INITIAL_BOARD.pw;
    println!();
    debug::print_bitboard(board);
    println!();
    let mut board_mut: bitboard::BitBoard = board;
    board_mut = bitboard::south_fill(board);
    debug::print_bitboard(board_mut);
    //println!();
    //debug::print_bitboard(&bitboard::rotate90cw(&INITIAL_BOARD.pw));
}
