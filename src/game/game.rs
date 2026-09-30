use crate::board_rep::bitboard::{BitBoard};
use crate::board_rep::board::*;
use crate::board_rep::constants::{Piece, Colour};

pub struct ChessGame {
    board: Board,
    turn: u32,
}

impl ChessGame {
    pub fn new() -> ChessGame {
        let game  = ChessGame {
            board: Board::default(),
            turn: 0,
        };

        return game;
    }

    pub fn get_board(&self) -> Board {
        return self.board;
    }

    pub fn try_move(&mut self, mov: &Move) -> Option<[bool; 6]> {
        if self.is_move_valid( mov) == false {
            return None;
        }
        let mut special_actions = [false; 6];
        let mut board_copy: Board = self.board;
        board_copy = board_copy & (1<<mov.tgt_square);
        if all(&board_copy) != 0 {
            special_actions[SpecialActions::Capture as usize] = true;
        }

        self.board = self.board & !(1<<mov.tgt_square);

        let (piece, colour) = get_piece_at_square(mov.org_square, self.board).unwrap();

        let piece_bb: &mut BitBoard = self.board.get_bb_mut(piece, colour);
        *piece_bb &= !(1<<mov.org_square);
        *piece_bb |= 1<<mov.tgt_square;

        self.turn += 1;
        return Some(special_actions);
    }

    pub fn is_move_valid(&mut self, mov: &Move) -> bool {
        let (piece, colour) = match get_piece_at_square(mov.org_square, self.board) {
            Some(pc) => pc,
            None => return false
        };

        // If piece to move does not match the turn
        if (self.turn % 2 == 0 && colour == Colour::Black) || (self.turn % 2 == 1 && colour == Colour::White) {
            return false;
        } 

        let possible_targets = legal_targets(mov.org_square, piece, colour, &self.board);
        let tgt_bb = 1 << mov.tgt_square;

        return match possible_targets & tgt_bb {
            0 => false,
            _ => true
        }
    }

    /// Returns true if there is a piece on the given square that can move
    pub fn square_can_move(&mut self, square: usize) -> bool {
        return self.square_possible_targets(square) != 0;
    }

    /// Returns a bitboard of every possible square the piece on the given square could target
    /// Returns empty bitboard if it is not the correct turn, or if selected square does not contain a piece
    pub fn square_possible_targets(&mut self, square: usize) -> BitBoard {
        let (piece, colour) = match get_piece_at_square(square, self.board) {
            Some(pc) => pc,
            None => return 0
        };

        if (self.turn % 2 == 0 && colour == Colour::Black) || (self.turn % 2 == 1 && colour == Colour::White) {
            return 0;
        }

        return legal_targets(square, piece, colour, &self.board);
    }
}

#[derive(Eq, PartialEq)]
pub enum SpecialActions {
    Castle,
    LongCastle,
    Check,
    CheckMate,
    Capture,
    Promotion,
}

#[derive(Eq, PartialEq)]
pub struct Move {
    pub org_square: usize,
    pub tgt_square: usize,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            org_square: 0,
            tgt_square: 0,
        }
    }
}

pub fn print_board(board: &Board) {
    let mut char_arrs: [[char; 64]; 6] = [['0'; 64]; 6];
    //print!("{}[2J", 27 as char);

    char_arrs[0] = fmt_bitboard_char(board.pw | board.pb, 'p');
    char_arrs[1] = fmt_bitboard_char(board.nw | board.nb, 'n');
    char_arrs[2] = fmt_bitboard_char(board.bw | board.bb, 'b');
    char_arrs[3] = fmt_bitboard_char(board.rw | board.rb, 'r');
    char_arrs[4] = fmt_bitboard_char(board.qw | board.qb, 'q');
    char_arrs[5] = fmt_bitboard_char(board.kw | board.kb, 'k');

    let mut i: usize = 0;
    let mut output = ['0'; 64];
    while i < char_arrs[0].len() {
        for char_arr in char_arrs {
            if char_arr[i] != '0' {
                output[i] = char_arr[i]; 
                break;
            }
        }
        i += 1;
    }
    
    i = 0;
    while i < 8 {
        let mut j: usize = 0;
        while j < 8 {
            print!("{:}", output[((7-i)*8)+j]);
            j += 1;
        }
        println!();
        i += 1;
    }
    
}

pub fn fmt_bitboard_char(board: BitBoard, disp_char: char) -> [char; 64]{
    let mut arr: [char; 64] = ['0'; 64];

    let mut i: usize = 0;
    while i < arr.len() {
        if (1<<i) & board != 0 {
            arr[i] = disp_char;
        }
        i += 1;
    }
    return arr;
}


