use chess_engine::{bitboard::INITIAL_BOARD, debug};
use chess_engine::bitboard;

fn main() {
    debug::print_board(&INITIAL_BOARD);

    let board: u64 = INITIAL_BOARD.pw;
    println!();
    debug::print_bitboard(&board);
    println!();
    let mut board_mut = board;
    bitboard::flip_anti_diag(&mut board_mut);
    debug::print_bitboard(&board_mut);
    //println!();
    //debug::print_bitboard(&bitboard::rotate90cw(&INITIAL_BOARD.pw));
}
