use piston_window::*;
use sprite::*;
use uuid::Uuid;
use std::{collections::HashMap, rc::Rc};

use crate::board_rep::constants::{Piece, Colour};
use crate::board_rep::board::Board;

enum ChessState {
    Playing
}
impl std::default::Default for ChessState {
    fn default() -> Self {
        ChessState::Playing
    }
}

struct Rectangle {
    rect: [f64; 4],
    colour
}
type Rectangle = ([f64; 4], u8);
struct ChessRenderer {
    window: Rc<PistonWindow>,
    state: ChessState,    
    selected_piece: Option<usize>,
    scene: Scene<G2dTexture>,
    scene_map: HashMap<usize, (Uuid, Piece, Colour)>,
    board_squares: [Rectangle; 64],
    highlights: [bool; 64],
}


impl ChessRenderer {
    pub fn new(window: Rc<PistonWindow>, state: ChessState) -> ChessRenderer {
        ChessRenderer { 
            window: window, 
            state: ChessState::default(), 
            selected_piece: (), 
            scene: (), 
            scene_map: (), 
            board_squares: (), 
            highlights: () }

    }
    pub fn init_to_state(board: &Board, state: ChessState){


    }
    fn test(&mut self) {
        self.scene_map.
    }
}