use chess_engine::{bitboard::INITIAL_BOARD, debug};

fn main() {
    debug::print_board(&INITIAL_BOARD);
    println!();
    let combined: u64 = INITIAL_BOARD.pw | INITIAL_BOARD.pb;
    debug::print_bitboard(&(combined / 8));
}
