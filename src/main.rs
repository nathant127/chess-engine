use chess_engine::board_rep::bitboard;
use chess_engine::board_rep::board::*;
use chess_engine::board_rep::constants;
use chess_engine::board_rep::debug;
use chess_engine::board_rep::constants::*;
use chess_engine::game::notation::read_notation;
use chess_engine::game::game::*;

fn main() {
    debug::print_board(&INITIAL_BOARD);

    let board: u64 = INITIAL_BOARD.pw;
    println!();
    debug::print_bitboard(board);
    println!();
    let mut board_mut: bitboard::BitBoard = board;
    board_mut = bitboard::south_fill(board);
    debug::print_bitboard(board_mut);

    debug::print_bitboard(RAY_TARGETS[RayDir::N as usize][constants::Square::A1 as usize]);
    
    let mut board: Board = INITIAL_BOARD;
    debug::print_bitboard(all(&board));
    loop {
        debug::print_bitboard(all(&board));
        let mov: Move = match read_notation(&board, Colour::WHITE) {
            Some(mov) => mov,
            None => {
                println!("Notation Was Invalid"); 
                continue;
            },
        };
        execute_move(&mut board, &mov);
    }
    

    
    //println!();
    //debug::print_bitboard(&bitboard::rotate90cw(&INITIAL_BOARD.pw));
}
