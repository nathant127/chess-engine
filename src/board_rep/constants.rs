use crate::board_rep::bitboard::*;

pub const WHITE_PAWN_ATTACKS: [u64; 64] = init_wpawn_attacks();
pub const BLACK_PAWN_ATTACKS: [u64; 64] = init_bpawn_attacks();

const fn init_wpawn_attacks() -> [u64; 64] {
    let mut arr: [u64; 64] = [0; 64];
    let mut i: usize = 0;

    while i < arr.len() {
        let i_file = file(i as u64); // Get file of pawn
        let left_atk = (i + 7) as u64; // Get the index NW of pawn
        let right_atk = (i + 9) as u64; // Get the index NE of pawn

        if rank(i as u64) == 7{
            break;
        }
        // If the file of the attack is a distance of one away add it
        if i_file.wrapping_sub(file(left_atk)) == 1{ 
            arr[i] |= 1 << left_atk;
        }
        if (file(right_atk)).wrapping_sub(i_file) == 1{
            arr[i] |= 1 << right_atk;
        }
        i += 1;
    }
    return arr;
}

const fn init_bpawn_attacks() -> [u64; 64] {
    let mut w_arr: [u64; 64] = init_wpawn_attacks();
    let mut b_arr: [u64; 64] = [0; 64];

    let mut i = 0;
    // Black pawn attack array is just the white pawn array rotated 180 and reversed
    while i < w_arr.len() {
        rotate180(&mut w_arr[i]);
        b_arr[63-i] = w_arr[i];
        i += 1;
    }
    return b_arr;
}