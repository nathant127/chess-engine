use super::bitboard::*;

pub fn print_bitboard(mut bitboard: BitBoard){
    flip_h(&mut bitboard);
    println!("{:#x}", bitboard);
    println!("{:08b}", row(&bitboard,7));
    println!("{:08b}", row(&bitboard,6));
    println!("{:08b}", row(&bitboard,5));
    println!("{:08b}", row(&bitboard,4));
    println!("{:08b}", row(&bitboard,3));
    println!("{:08b}", row(&bitboard,2));
    println!("{:08b}", row(&bitboard,1));
    println!("{:08b}", row(&bitboard,0));
}