use crate::board_rep::bitboard::*;
use crate::board_rep::board::{Board, all};

pub fn print_board(state: &Board) {
    print_bitboard(&all(state));
}

pub fn print_bitboard(bitboard: &BitBoard){
    println!("{:08b}", row(bitboard,7));
    println!("{:08b}", row(bitboard,6));
    println!("{:08b}", row(bitboard,5));
    println!("{:08b}", row(bitboard,4));
    println!("{:08b}", row(bitboard,3));
    println!("{:08b}", row(bitboard,2));
    println!("{:08b}", row(bitboard,1));
    println!("{:08b}", row(bitboard,0));
}