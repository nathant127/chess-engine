extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate find_folder;
extern crate sprite;
extern crate piston_window;
extern crate uuid;

use piston_window::*;
use sprite::*;

use opengl_graphics::OpenGL;

use piston::input::RenderEvent;
use piston::window::WindowSettings;

use crate::board_rep::board::{self, Board};
use crate::board_rep::constants::{Colour, Piece};
use crate::board_rep::bitboard::{self, rank, file};
use crate::game::game::{self};
use super::resource_manager::ResourceManager;
use uuid::Uuid;


struct SceneMetaData {
    id: Uuid,
    square: usize,
}

type Rectangle = [f64;4];
pub struct ChessGame {
    window: PistonWindow,
    res_man: ResourceManager,
    scene: Scene<G2dTexture>,
    scene_metadata: Vec<SceneMetaData>,
    cursor: [f64; 2],
    window_size: [f64; 2],

    board: Board,
    highlights: [bool; 64],
    selected_piece: Option<(Piece, Colour, usize)>,
    squares: [Rectangle; 64],
}

const OPENGL: OpenGL = OpenGL::V3_2;
const SQUARE_SIZE: f64 = 100.0;

impl ChessGame {
    fn make_game() -> ChessGame {
        let mut window: PistonWindow = WindowSettings::new("Chess Engine", [1000, 1000])
            .graphics_api(OPENGL)
            .exit_on_esc(true)
            .build()
            .expect("Failed to create piston window");
        window.set_lazy(true);

        let texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };

        let mut game = ChessGame { 
            window: window,
            board: Board::default(), 
            cursor: [0.0, 0.0], 
            highlights: [false; 64], 
            squares: ChessGame::init_board_squares(),
            window_size: [0.0, 0.0],
            res_man: ResourceManager::new(texture_context),
            scene: Scene::new(),
            scene_metadata: Vec::new(),
            selected_piece: None,
        };
        
        let result = game.res_man.add_folder("assets");
        if let Err(e) = result {
            println!("Failed to find the assets folder, is it included with the executable? {e}");
        }
        game.create_initial_scene();

        return game;
    }

    fn create_initial_scene(&mut self){
        self.fill_scene_piece(self.board.pw, "white_pawn.png");
        self.fill_scene_piece(self.board.nw, "white_knight.png");
        self.fill_scene_piece(self.board.bw, "white_bishop.png");
        self.fill_scene_piece(self.board.rw, "white_rook.png");
        self.fill_scene_piece(self.board.qw, "white_queen.png");
        self.fill_scene_piece(self.board.kw, "white_king.png");

        self.fill_scene_piece(self.board.pb, "black_pawn.png");
        self.fill_scene_piece(self.board.nb, "black_knight.png");
        self.fill_scene_piece(self.board.bb, "black_bishop.png");
        self.fill_scene_piece(self.board.rb, "black_rook.png");
        self.fill_scene_piece(self.board.qb, "black_queen.png");
        self.fill_scene_piece(self.board.kb, "black_king.png");
    }

    fn fill_scene_piece(&mut self, mut bitboard: u64, tex_name: &str) {
        let tex = self.res_man.get_texture(tex_name);
        while bitboard != 0 {
            let index = bitboard::pop(&mut bitboard);
            let mut sprite = Sprite::from_texture(tex.clone());
            sprite.set_position(self.squares[index][0] + SQUARE_SIZE/2.0, self.squares[index][1] + SQUARE_SIZE/2.0);
            let id = self.scene.add_child(sprite);
            self.scene_metadata.push(SceneMetaData { id: id, square: index });
        }
    }
    
    fn render(&mut self, e: &Event) {
        let args = e.render_args().unwrap();
        self.window_size = args.window_size;
        self.render_board(e);
    }

    fn render_board(&mut self, e: &Event) {
        use graphics::*;
        let args = e.render_args().unwrap();

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        const WHITE: [f32; 4] = [0.7,0.7,0.7,1.0];
        const BLACK: [f32; 4] = [0.3,0.3,0.3,1.0];
        const WHITE_HIGHLIGHTED: [f32; 4] = [1.0,1.0,1.0,1.0];
        const BLACK_HIGHLIGHTED: [f32; 4] = [0.0,0.0,0.0,1.0];

        let (x_centre, y_centre) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let x_board = 0.0;
        let y_board = 0.0;

        self.window.draw_2d(e, |c, g, _| {
            clear(BACKGROUND, g);

            let transform = c.transform.trans(x_centre,y_centre).trans(x_board, y_board);
            let mut i = 0;
            while i < self.squares.len() {
                let r = rank(i);
                let render_option = ((i + (r as usize)) % 2) | (self.highlights[i] as usize) << 1;
                let colour = match render_option {
                    0b00 => BLACK,
                    0b01 => WHITE,
                    0b10 => BLACK_HIGHLIGHTED,
                    0b11 => WHITE_HIGHLIGHTED,
                    _ => {
                        i+=1; 
                        continue;
                    },
                };
                rectangle(colour, self.squares[i],  transform, g);
                i+=1;
            }

            self.scene.draw(transform, g);
        });

    }

    fn on_click(&mut self, button: &MouseButton) {
        println!("Pressed Button {:?} at pos {:}, {:}", button, self.cursor[0], self.cursor[1]);
        let (x_board_frame, y_board_frame) = (self.cursor[0] - self.window_size[0] / 2.0, self.cursor[1] - self.window_size[1] / 2.0);
        let square_op: Option<usize> = self.resolve_square(x_board_frame, y_board_frame);
        
        self.highlights = [false; 64];
        let clicked_square = match square_op {
            Some(sq) => sq,
            None => {
                self.selected_piece = None;
                return;
            }
        };

        
        if let Some(sel_piece) = self.selected_piece  {
            let targets = board::legal_targets(sel_piece.2, sel_piece.0, sel_piece.1, &self.board);
            if (1<<clicked_square) & targets != 0 {
                let mov: game::Move = game::Move {
                    org_square: sel_piece.2, 
                    tgt_square: clicked_square, 
                    piece: sel_piece.0, 
                    colour: sel_piece.1
                };
                let special_actions = match game::execute_move(&mut self.board, &mov) {
                    Some(x) => x,
                    None => {return;},
                };
                self.update_sprites(&mov, special_actions);
                self.selected_piece = None;
                return;
            }
        }

        let piece_colour = match board::get_piece_at_square(clicked_square, self.board) {
            Some(p) => p,
            None => {
                self.selected_piece = None;
                return;
            }
        };
        self.selected_piece = Some((piece_colour.0, piece_colour.1, clicked_square));

        self.update_highlights(clicked_square, piece_colour);

    }

    fn update_highlights(&mut self, square: usize, piece_colour: (Piece, Colour)) {
        let mut targets = board::legal_targets(square, piece_colour.0, piece_colour.1, &self.board);
        while targets != 0 {
            let index = bitboard::pop(&mut targets);
            self.highlights[index] = true;
        }
    }

    fn update_sprites(&mut self, mov: &game::Move, special_actions: [bool; 6]) {
        // Remove captured piece
        if special_actions[game::SpecialActions::Capture as usize] == true {
            for (i,md) in self.scene_metadata.iter_mut().enumerate() {
                if md.square == mov.tgt_square {
                    self.scene.remove_child(md.id);
                    self.scene_metadata.remove(i);
                    break;
                }
            }
        }

        for md in self.scene_metadata.iter_mut() {
            if md.square == mov.org_square {
                md.square = mov.tgt_square;
                self.scene.child_mut(md.id).unwrap().set_position(self.squares[md.square][0] + SQUARE_SIZE/2.0, self.squares[md.square][1] + SQUARE_SIZE/2.0);
            }
        }

    }

    fn resolve_square(& self, x: f64, y: f64) -> Option<usize> {
        for (i, square) in self.squares.iter().enumerate() {
            if ChessGame::is_inside_square(x, y, square[0], square[1], square[2]) {
                return Some(i);
            }
        }

        return None;
    }

    #[inline]
    fn is_inside_square(x: f64, y: f64, x_sq: f64, y_sq: f64, size: f64) -> bool {
        (x >= x_sq) && (x < (x_sq + size)) && (y >= y_sq) && (y < (y_sq + size))
    }

    fn init_board_squares() -> [Rectangle; 64] {
        let mut squares: [Rectangle; 64] = [[0.0; 4]; 64];
        const BOARD_SIZE: f64 = SQUARE_SIZE * 8.0;
        

        let mut i = 0;
        while i < squares.len() {
            let r = rank(i) as f64;
            let f = file(i) as f64;

            let x = (f * SQUARE_SIZE) - BOARD_SIZE/2.0;
            let y = (r * SQUARE_SIZE) - BOARD_SIZE/2.0;
            squares[i ^ 56] = graphics::rectangle::square(x, y, SQUARE_SIZE);
            i += 1;
        }


        return squares;
    }

    fn update(&mut self, e: &Event) {
        if let Some(_args) = e.render_args() {
            self.render(&e);
        }
        // if let Some(args) = e.update_args() {
        //     self.update(&args);
        // }

        // Update mouse cursor pos every frame
        e.mouse_cursor(|pos| {
            self.cursor = pos;
        });
        // If click check on click
        if let Some(Button::Mouse(args)) = e.press_args() {
        self.on_click(&args);
        }
        // if let Some(args) = e.button_args() {
        //     println!("Scancode {:?}", args);
        // }
    }
}

pub fn run_game() {
    // Change this to OpenGL::V2_1 if not working.
    
    

    // Create a new game and run it.
    let mut game = ChessGame::make_game();

    // Run game until there are no more events
    while let Some(e) = game.window.next() {
        game.update(&e);
    }   
}