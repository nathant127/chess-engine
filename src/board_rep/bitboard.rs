use super::constants::*;
// USES Little-Endian Rank-File Mapping (LERF)
// a1 -> bit 0
// h1 -> bit 7
// a2 -> bit 8
// h8 -> bit 63

pub type BitBoard = u64;

#[inline]
pub const fn index(rank: u64, file: u64) -> usize{
    return (8*rank + file) as usize; // LSF mapping
}
#[inline]
pub const fn file(index: usize) -> u64{
    return index as u64 & 7; // Equivalent to mod 8

}
#[inline]
pub const fn rank(index: usize) -> u64{
    return index as u64 >> 3;  // Equivalent to div/8
}

#[inline]
pub fn row(board: &BitBoard, row: u64) -> u64{
    return board >> (row*8) & RANK_1;
}
// Flipping and rotating algorithms from https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#Rotationby90degreesClockwise 
pub const fn flip_v(board: &mut BitBoard){
    const K1: u64 = 0x00FF00FF00FF00FF;
    const K2: u64 = 0x0000FFFF0000FFFF;
    *board = ((*board >>  8) & K1) | ((*board & K1) <<  8);
    *board = ((*board >> 16) & K2) | ((*board & K2) << 16);
    *board = ( *board >> 32)       | ( *board       << 32);
}
pub const fn flip_h(board: &mut BitBoard){
    const K1: u64 = 0x5555555555555555;
    const K2: u64 = 0x3333333333333333;
    const K4: u64 = 0x0f0f0f0f0f0f0f0f;
    *board = ((*board >> 1) & K1) +  2*(*board & K1);
    *board = ((*board >> 2) & K2) +  4*(*board & K2);
    *board = ((*board >> 4) & K4) + 16*(*board & K4);
}
// Flip diagonal
pub const fn flip_a1h8(board: &mut BitBoard){
    const K1: u64 = 0x5500550055005500;
    const K2: u64 = 0x3333000033330000;
    const K4: u64 = 0x0f0f0f0f00000000;
    let mut t: u64;
    t  = K4 & (*board ^ (*board << 28));
    *board ^=       t ^ (t >> 28) ;
    t  = K2 & (*board ^ (*board << 14));
    *board ^=       t ^ (t >> 14) ;
    t  = K1 & (*board ^ (*board <<  7));
    *board ^=       t ^ (t >>  7) ;
}
// Flip anti diagonal
pub const fn flip_a8h1(board: &mut BitBoard){
    const K1: u64 = 0xaa00aa00aa00aa00;
    const K2: u64 = 0xcccc0000cccc0000;
    const K4: u64 = 0xf0f0f0f00f0f0f0f;
    let mut t: u64;
    t  =       *board ^ (*board << 36) ;
    *board ^= K4 & (t ^ (*board >> 36));
    t  = K2 & (*board ^ (*board << 18));
    *board ^=       t ^ (t >> 18) ;
    t  = K1 & (*board ^ (*board <<  9));
    *board ^=       t ^ (t >>  9) ;
}

#[inline]
pub const fn rotate90(board: &mut BitBoard){
    flip_a1h8(board);
    flip_v(board);
}

// Equivalent to flip vertical -> flip horizontal
#[inline]
pub const fn rotate180(board: &mut BitBoard){
    flip_v(board);
    flip_h(board);
}
#[inline]
pub const fn rotate270(board: &mut BitBoard){
    flip_v(board);
    flip_a1h8(board);
}

/** Constant time version of popcount, should be used for densely populated bitboards
*/
pub fn popcount_swar(mut board: BitBoard) -> u64{
    const K1: u64 = 0x5555555555555555; /*  -1/3   */
    const K2: u64 = 0x3333333333333333; /*  -1/5   */
    const K4: u64 = 0x0f0f0f0f0f0f0f0f; /*  -1/17  */
    const KF: u64 = 0x0101010101010101; /*  -1/255 */
    board =  board       - ((board >> 1)  & K1);
    board = (board & K2) + ((board >> 2)  & K2);
    board = (board       +  (board >> 4)) & K4 ;
    board = (board.overflowing_mul(KF).0) >> 56;
    return board;
}
/** Fast algo for sparsely populated bitboards
 */
pub fn popcount_loop(mut board: BitBoard) -> u64 {
    let mut count: u64 = 0;
    while board != 0 {
        count += 1;
        board &= board - 1; // reset LS1B
    }
    return count;
}

/** Returns the index of the Least significant 1 bit of the bitboard
 * https://www.chessprogramming.org/BitScan 
 */
pub const fn bitscan_forward(board: BitBoard) -> usize {
    assert!(board != 0, "Bitscan forward recieved a board 0");
    return BITSCAN_INDEX[(((board ^ (board-1)).overflowing_mul(BITSCAN_DEBRUIJIN).0) >> 58) as usize];
}

/** Returns the index of the Most significant 1 bit of the bitboard
 * https://www.chessprogramming.org/BitScan 
 */
pub const fn bitscan_reverse(mut board: BitBoard) -> usize {
   assert!(board != 0, "Bitscan reverse recieved a board 0");
   board |= board >> 1; 
   board |= board >> 2;
   board |= board >> 4;
   board |= board >> 8;
   board |= board >> 16;
   board |= board >> 32;
   return BITSCAN_INDEX[((board.overflowing_mul(BITSCAN_DEBRUIJIN).0) >> 58) as usize];
}

pub fn north_fill(mut board: BitBoard) -> BitBoard {
   board |= board <<  8;
   board |= board << 16;
   board |= board << 32;
   return board;
}

pub fn south_fill(mut board: BitBoard) -> BitBoard {
   board |= board >>  8;
   board |= board >> 16;
   board |= board >> 32;
   return board;
}

#[inline]
pub const fn shift_north(board: BitBoard) -> BitBoard {board << 8}
#[inline]
pub const fn shift_south(board: BitBoard) -> BitBoard {board >> 8}
#[inline]
pub const fn shift_east(b : BitBoard) -> BitBoard {(b << 1) & NOT_FILE_A}
#[inline]
pub const fn shift_northeast(b: BitBoard) -> BitBoard {(b << 9) & NOT_FILE_A}
#[inline]
pub const fn shift_southeast(b: BitBoard) -> BitBoard {(b >> 7) & NOT_FILE_A}
#[inline]
pub const fn shift_west(b : BitBoard) -> BitBoard {(b >> 1) & NOT_FILE_H}
#[inline]
pub const fn shift_southwest(b: BitBoard) -> BitBoard {(b >> 9) & NOT_FILE_H}
#[inline]
pub const fn shift_northwest(b: BitBoard) -> BitBoard {(b << 7) & NOT_FILE_H}

#[inline]
pub const fn shift_northdbl(board: BitBoard) -> BitBoard {board << 16}
#[inline]
pub const fn shift_southdbl(board: BitBoard) -> BitBoard {board >> 16}

#[inline]
pub const fn shift_northx(board: BitBoard, x: u64) -> BitBoard {board << 8*x}
#[inline]
pub const fn shift_southx(board: BitBoard, x: u64) -> BitBoard {board >> 8*x}

#[inline]
pub const fn isolate(board: BitBoard) -> BitBoard {board & board.wrapping_neg()}
#[inline]
pub const fn reset(board: BitBoard) -> BitBoard {board & (board-1)}

#[inline]
pub const fn pop(board: &mut BitBoard) -> usize {
    let square = bitscan_forward(*board);
    *board = reset(*board);
    return square;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index(){
        let mut square: usize = Square::A1 as usize; 
        assert_eq!(square, 0);
        assert_eq!(rank(square), 0);
        assert_eq!(file(square), 0);
        assert_eq!(index(0,0), square);
        
        square = Square::A7 as usize;
        assert_eq!(square, 48);
        assert_eq!(rank(square), 6);
        assert_eq!(file(square), 0);
        assert_eq!(index(6,0), square);

        square = Square::E3 as usize;
        assert_eq!(square, 20);
        assert_eq!(rank(square), 2);
        assert_eq!(file(square), 4);
        assert_eq!(index(2,4), square);
    }
    #[test]
    fn test_flip(){
        let mut board: BitBoard = RANK_1;
        flip_v(&mut board);
        assert_eq!(board, RANK_8);

        board = FILE_A;
        flip_h(&mut board);
        assert_eq!(board, FILE_H);

        board = RANK_1;
        flip_a1h8(&mut board);
        assert_eq!(board, FILE_A);

        board = RANK_1;
        flip_a8h1(&mut board);
        assert_eq!(board, FILE_H);
    }

    #[test]
    fn test_rotate(){
        let board_start: BitBoard = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3);

        let mut board: BitBoard = board_start;
        rotate90(&mut board);
        assert_eq!(board, (1 << 56) | (1<<48) | (1<<40) | (1<<32)); // Equal to bits 32, 40, 48, 56
        rotate270(&mut board);
        assert_eq!(board, board_start); // Rotate back to start

        board = board_start;
        rotate180(&mut board);
        assert_eq!(board, (0xF << 60)); // Should be bits 60-63
        rotate180(&mut board);
        assert_eq!(board, board_start);

        board = board_start;
        rotate270(&mut board);
        assert_eq!(board, (1<<31)|(1<<23)|(1<<15)|(1<<7)); // Bits 31 23 15 7
        rotate90(&mut board);
        assert_eq!(board, board_start);
    }

    #[test]
    fn test_popcount(){
        let mut board: BitBoard = (1 << 0) | (3 << 32) | (1 << 27) | (1 << 63); // Count should be 5
        assert_eq!(popcount_loop(board), 5);
        assert_eq!(popcount_swar(board), 5);
        
        board = 0;
        assert_eq!(popcount_loop(board), 0);
        assert_eq!(popcount_swar(board), 0);

        board = u64::MAX;
        assert_eq!(popcount_loop(board), 64);
        assert_eq!(popcount_swar(board), 64);

        board = SQUARE_LIGHT;
        assert_eq!(popcount_loop(board), 32);
        assert_eq!(popcount_swar(board), 32);
    }

    #[test]
    fn test_bitscan(){
        let mut board: BitBoard = (0b1101 << 35) | (1 << 46);
        assert_eq!(bitscan_forward(board), 35);
        assert_eq!(bitscan_reverse(board), 46);

        board = 1;
        assert_eq!(bitscan_forward(board), 0);
        assert_eq!(bitscan_reverse(board), 0);
    }
}