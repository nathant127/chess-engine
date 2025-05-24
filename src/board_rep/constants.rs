use super::bitboard::*;
use super::board::Board;
use std::ops::{Mul, Not};

pub const FILE_A: BitBoard = 0x0101010101010101;
pub const FILE_H: BitBoard = 0x8080808080808080;
pub const NOT_FILE_A: BitBoard = !FILE_A;
pub const NOT_FILE_H: BitBoard = !FILE_H;
pub const RANK_1: BitBoard = 0x00000000000000FF;
pub const RANK_2: BitBoard = RANK_1 << (8*1);
pub const RANK_3: BitBoard = RANK_1 << (8*2);
pub const RANK_4: BitBoard = RANK_1 << (8*3);
pub const RANK_5: BitBoard = RANK_1 << (8*4);
pub const RANK_6: BitBoard = RANK_1 << (8*5);
pub const RANK_7: BitBoard = RANK_1 << (8*6);
pub const RANK_8: BitBoard = 0xFF00000000000000;
pub const DIAG_A1_H8: BitBoard = 0x8040201008040201;
pub const DIAG_H1_A8: BitBoard = 0x0102040810204080;
pub const SQUARE_LIGHT: BitBoard = 0x55AA55AA55AA55AA;
pub const SQUARE_DARK: BitBoard = 0xAA55AA55AA55AA55;

#[repr(u64)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING
}
impl Mul<Colour> for Piece {
    type Output = PieceColour;
    fn mul(self: Self, rhs: Colour) -> Self::Output {
        PieceColour::from_usize((self as usize) + (rhs as usize * 6))
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Colour {
    WHITE,
    BLACK
}
impl Mul<Piece> for Colour {
    type Output = PieceColour;
    fn mul(self: Self, rhs: Piece) -> Self::Output {
        PieceColour::from_usize((rhs as usize) + (self as usize * 6))
    }
}
impl Not for Colour {
    type Output = Colour;
    fn not(self: Self) -> Self::Output {
        match self {
            Colour::WHITE => Colour::BLACK,
            Colour::BLACK => Colour::WHITE,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PieceColour {
    WHITE_PAWN, WHITE_KNIGHT, WHITE_BISHOP, WHITE_ROOK, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_KNIGHT, BLACK_BISHOP, BLACK_ROOK, BLACK_QUEEN, BLACK_KING
}
impl PieceColour {
    fn from_usize(num: usize) -> PieceColour {
        match num {
            0 => PieceColour::WHITE_PAWN,
            1 => PieceColour::WHITE_KNIGHT,
            2 => PieceColour::WHITE_BISHOP,
            3 => PieceColour::WHITE_ROOK,
            4 => PieceColour::WHITE_QUEEN,
            5 => PieceColour::WHITE_KING,
            6 => PieceColour::BLACK_PAWN,
            7 => PieceColour::BLACK_KNIGHT,
            8 => PieceColour::BLACK_BISHOP,
            9 => PieceColour::BLACK_ROOK,
            10 => PieceColour::BLACK_QUEEN,
            11 => PieceColour::BLACK_KING,
            _ => {panic!("Tried to convert to piececolour but no match was found")},
        }
    }
}

pub const INITIAL_BOARD: Board = Board {
    pw: 0x000000000000FF00,
    rw: 0x0000000000000081,
    nw: 0x0000000000000042,
    bw: 0x0000000000000024,
    qw: 0x0000000000000010,
    kw: 0x0000000000000008,

    pb: 0x00FF000000000000,
    rb: 0x8100000000000000,
    nb: 0x4200000000000000,
    bb: 0x2400000000000000,
    qb: 0x0800000000000000,
    kb: 0x1000000000000000
};

pub const BITSCAN_INDEX: [usize; 64] = [
    0, 47,  1, 56, 48, 27,  2, 60,
    57, 49, 41, 37, 28, 16,  3, 61,
    54, 58, 35, 52, 50, 42, 21, 44,
    38, 32, 29, 23, 17, 11,  4, 62,
    46, 55, 26, 59, 40, 36, 15, 53,
    34, 51, 20, 43, 31, 22, 10, 45,
    25, 39, 14, 33, 19, 30,  9, 24,
    13, 18,  8, 12,  7,  6,  5, 63
];
pub const BITSCAN_DEBRUIJIN: u64 = 0x03f79d71b4cb0a89;


pub const WHITE_PAWN_ATTACKS: [u64; 64] = init_wpawn_attacks();
pub const BLACK_PAWN_ATTACKS: [u64; 64] = init_bpawn_attacks();
pub const KNIGHT_TARGETS: [u64; 64] = init_knight_targets();
pub const KING_TARGETS: [u64; 64] = init_king_targets();

#[repr(usize)]
pub enum RayDir {
    N, NE,
    E, SE,
    S, SW,
    W, NW
}
pub const RAY_TARGETS: [[u64; 64]; 8] = init_ray_targets();

const fn init_wpawn_attacks() -> [u64; 64] {
    let mut arr: [u64; 64] = [0; 64];
    let mut i: usize = 0;

    while i < arr.len() {
        let i_file = file(i); // Get file of pawn
        let left_atk = (i + 7); // Get the index NW of pawn
        let right_atk = (i + 9); // Get the index NE of pawn

        if rank(i) == 7{
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

const fn init_knight_targets() -> [u64; 64] {
    let mut arr: [u64; 64] = [0; 64];

    let mut i = 0;
    while i < arr.len() {
        let pos: u64 = 1 << i;
        let mut shifted = shift_north(pos);
        arr[i] |= shift_northeast(shifted) | shift_northwest(shifted);

        shifted = shift_east(pos);
        arr[i] |= shift_northeast(shifted) | shift_southeast(shifted);

        shifted = shift_south(pos);
        arr[i] |= shift_southeast(shifted) | shift_southwest(shifted);

        shifted = shift_west(pos);
        arr[i] |= shift_southwest(shifted) | shift_northwest(shifted);

        i += 1;
    }
    return arr;
}

const fn init_king_targets() -> [u64; 64] {
    let mut arr: [u64; 64] = [0; 64];

    let mut i = 0;
    while i < arr.len() {
        let mut pos: u64 = 1 << i;
        arr[i] = shift_east(pos) | shift_west(pos);
        pos |= arr[i];
        arr[i] |= shift_north(pos) | shift_south(pos);
        i += 1;
    }
    return arr;
}

const fn init_ray_targets() -> [[u64; 64]; 8] {
    let mut arr: [[u64; 64]; 8] = [[0; 64]; 8];

    let mut i = 0;
    while i < arr[0].len() {
        let pos: u64 = 1 << i;

        arr[RayDir::N as usize][i] = shift_north(pos);
        let mut j = 0;
        while j < 7 {
            arr[RayDir::N as usize][i] |= shift_north(arr[RayDir::N as usize][i]);
            j += 1;
        }

        arr[RayDir::NE as usize][i] = shift_northeast(pos);
        j = 0;
        while j < 7 {
            arr[RayDir::NE as usize][i] |= shift_northeast(arr[RayDir::NE as usize][i]);
            j += 1;
        }

        arr[RayDir::E as usize][i] = shift_east(pos);
        j = 0;
        while j < 7 {
            arr[RayDir::E as usize][i] |= shift_east(arr[RayDir::E as usize][i]);
            j += 1;
        }

        arr[RayDir::SE as usize][i] = shift_southeast(pos);
        j = 0;
        while j < 7 {
            arr[RayDir::SE as usize][i] |= shift_southeast(arr[RayDir::SE as usize][i]);
            j += 1;
        }

        arr[RayDir::S as usize][i] = shift_south(pos);
        j = 0;
        while j < 7 {
            arr[RayDir::S as usize][i] |= shift_south(arr[RayDir::S as usize][i]);
            j += 1;
        }

        arr[RayDir::SW as usize][i] = shift_southwest(pos);
        j = 0;
        while j < 7 {
            arr[RayDir::SW as usize][i] |= shift_southwest(arr[RayDir::SW as usize][i]);
            j += 1;
        }

        arr[RayDir::W as usize][i] = shift_west(pos);
        j = 0;
        while j < 7 {
            arr[RayDir::W as usize][i] |= shift_west(arr[RayDir::W as usize][i]);
            j += 1;
        }

        arr[RayDir::NW as usize][i] = shift_northwest(pos);
        j = 0;
        while j < 7 {
            arr[RayDir::NW as usize][i] |= shift_northwest(arr[RayDir::NW as usize][i]);
            j += 1;
        }

        i += 1;
    }
    return arr;
}

// const fn init_temp() -> [u64; 64] {
//     let mut arr: [u64; 64] = [0; 64];

//     let mut i = 0;
//     while i < arr.len() {

//         i += 1;
//     }
//     return arr;
// }
