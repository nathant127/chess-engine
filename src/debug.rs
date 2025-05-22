use crate::bitboard::{Board, BitBoard};
use crate::bitboard;

pub fn print_board(state: &Board) {
    print_bitboard(&bitboard::all(state));
}

pub fn print_bitboard(bitboard: &BitBoard){
    println!("{:08b}", bitboard::row(bitboard,7));
    println!("{:08b}", bitboard::row(bitboard,6));
    println!("{:08b}", bitboard::row(bitboard,5));
    println!("{:08b}", bitboard::row(bitboard,4));
    println!("{:08b}", bitboard::row(bitboard,3));
    println!("{:08b}", bitboard::row(bitboard,2));
    println!("{:08b}", bitboard::row(bitboard,1));
    println!("{:08b}", bitboard::row(bitboard,0));
}