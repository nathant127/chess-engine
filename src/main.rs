use chess_engine::board_rep::bitboard::*;
use chess_engine::board_rep::board::*;
use chess_engine::board_rep::constants::*;
use chess_engine::game::notation::read_notation;
use chess_engine::game::game::*;

fn main() {
    
    let mut board: Board = INITIAL_BOARD;
    loop {
        print_board(&board);
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
