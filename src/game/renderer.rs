use piston_window::*;
use sprite::*;
use uuid::Uuid;
use std::cell::RefCell;
use std::default::Default;
use std::ops::Range;
use std::rc::Rc;

use crate::board_rep::constants::{Piece, Colour};
use crate::board_rep::board::Board;
use crate::board_rep::bitboard::{rank, file, pop};

use super::resource_manager::ResourceManager;

enum ChessState {
    Playing
}
impl Default for ChessState {
    fn default() -> Self {
        ChessState::Playing
    }
}

struct BoardDefinition {
    rectangles: [[f64; 4]; 64],
    colours: [[f32; 4]; 64],
    square_size: f64,
    board_centre: [f64; 2],
    avaliable_colours: [[f32; 4]; 4],
}

impl Default for BoardDefinition {
    fn default() -> Self {
        const SQUARE_SIZE: f64 = 100.0;
        const BOARD_CENTRE: [f64; 2] = [0.0, 0.0];
        const BOARD_SIZE: f64 = SQUARE_SIZE * 8.0;

        const WHITE: [f32; 4] = [0.7,0.7,0.7,1.0];
        const BLACK: [f32; 4] = [0.3,0.3,0.3,1.0];
        const WHITE_HIGHLIGHTED: [f32; 4] = [1.0,1.0,1.0,1.0];
        const BLACK_HIGHLIGHTED: [f32; 4] = [0.0,0.0,0.0,1.0];

        let mut squares: [[f64; 4]; 64] = [[0.0; 4]; 64];
        let mut colours: [[f32; 4]; 64] = [[0.0; 4]; 64];

        let mut i = 0;
        while i < squares.len() {
            let r = rank(i) as f64;
            let f = file(i) as f64;

            let x = (f * SQUARE_SIZE) - BOARD_SIZE/2.0;
            let y = (r * SQUARE_SIZE) - BOARD_SIZE/2.0;
            squares[i ^ 56] = graphics::rectangle::square(x, y, SQUARE_SIZE);
            colours[i ^ 56] = match (i + (r as usize)) % 2 {
                0b0 => BLACK,
                0b1 => WHITE, 
                _ => panic!("Mod 2 is somehow > 1"),
            };

            i += 1;
        }

        BoardDefinition { 
            rectangles: squares, 
            colours: colours, 
            square_size: SQUARE_SIZE, 
            board_centre: BOARD_CENTRE, 
            avaliable_colours: [WHITE, WHITE_HIGHLIGHTED, BLACK, BLACK_HIGHLIGHTED],
        }
    }
}

#[derive(Clone, Copy)]
struct SpriteMD {
    id: Uuid,
    // piece: Piece,
    // colour: Colour,
}

struct ChessRenderer {
    window: RefCell<PistonWindow>,
    resource_manager: RefCell<ResourceManager>,
    state: ChessState,    
    selected_piece: Option<usize>,
    scene: Scene<G2dTexture>,
    sprite_map: [Option<SpriteMD>; 64],
    board_bg: BoardDefinition,
}


impl ChessRenderer {
    pub fn new(window: RefCell<PistonWindow>, res_man: RefCell<ResourceManager>, state: ChessState) -> ChessRenderer {
        ChessRenderer { 
            window: window, 
            resource_manager: res_man,
            state: state,
            selected_piece: None, 
            scene: Scene::new(), 
            sprite_map: [None; 64],
            board_bg: BoardDefinition::default(),
        }
    }

    pub fn init_to_state(&mut self, board: &Board, state: ChessState){
        for sprite_md in self.sprite_map {
            match sprite_md {
                Some(sprite_md) => {
                    self.scene.remove_child(sprite_md.id);
                },
                None => {},
            }            
        }
        self.sprite_map = [None; 64];
        self.state = state;
        self.selected_piece = None;
        self.create_scene(board);
    }

    pub fn render(&mut self, e: &Event) {
        use graphics::*;
        let args = e.render_args().unwrap();

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

        let (x_centre, y_centre) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let x_org = 0.0;
        let y_org = 0.0;

        self.window.borrow_mut().draw_2d(e, |c, g, _| {
            clear(BACKGROUND, g);

            let transform_to_centre = c.transform.trans(x_centre,y_centre).trans(x_org, y_org);
            
            // Draw Board
            for i in 0..self.board_bg.rectangles.len() {
                rectangle(self.board_bg.colours[i], self.board_bg.rectangles[i],  transform_to_centre, g);
            }
            // Draw sprites
            self.scene.draw(transform_to_centre, g);
        });
    }

    fn create_scene (&mut self, board: &Board) {
        self.add_bb_scene(board.pw, "white_pawn.png");
        self.add_bb_scene(board.nw, "white_knight.png");
        self.add_bb_scene(board.bw, "white_bishop.png");
        self.add_bb_scene(board.rw, "white_rook.png");
        self.add_bb_scene(board.qw, "white_queen.png");
        self.add_bb_scene(board.kw, "white_king.png");

        self.add_bb_scene(board.pb, "black_pawn.png");
        self.add_bb_scene(board.nb, "black_knight.png");
        self.add_bb_scene(board.bb, "black_bishop.png");
        self.add_bb_scene(board.rb, "black_rook.png");
        self.add_bb_scene(board.qb, "black_queen.png");
        self.add_bb_scene(board.kb, "black_king.png");
    }

    fn add_bb_scene(&mut self, mut bitboard: u64, tex_name: &str) {
        let tex = self.resource_manager.borrow_mut().get_texture(tex_name);
        
        while bitboard != 0 {
            let index = pop(&mut bitboard);
            let mut sprite = Sprite::from_texture(tex.clone());
            sprite.set_position(
                self.board_bg.rectangles[index][0] + self.board_bg.square_size/2.0, 
                self.board_bg.rectangles[index][1] + self.board_bg.square_size/2.0
            );
            let id = self.scene.add_child(sprite);
            self.sprite_map[index] = Some(SpriteMD {id: id})
        }
    }
}