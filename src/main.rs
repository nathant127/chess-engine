use chess_engine::game::gui_game::run_game;


fn main() {
    run_game();

    // let mut board: Board = INITIAL_BOARD;
    // loop {
    //     print_board(&board);
    //     let mov: Move = match read_notation(&board, Colour::White) {
    //         Some(mov) => mov,
    //         None => {
    //             println!("Notation Was Invalid"); 
    //             continue;
    //         },
    //     };
    //     execute_move(&mut board, &mov);
    // }
    

    
    //println!();
    //debug::print_bitboard(&bitboard::rotate90cw(&INITIAL_BOARD.pw));
}
