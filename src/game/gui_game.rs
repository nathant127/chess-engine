extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate find_folder;

use piston_window::*;

use opengl_graphics::{GlGraphics, OpenGL};

use glutin_window::GlutinWindow as Window;

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{WindowSettings};
use piston::input::*;


use crate::board_rep::board::{self, Board};
use crate::board_rep::constants::{Piece, Colour, Square};
use crate::board_rep::bitboard::{self, rank, file};

type Rectangle = [f64;4];
pub struct ChessGame {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    events: Events,
    board: Board,
    cursor: [f64; 2],
    highlights: [bool; 64],
    squares_initialized: bool,
    squares: [Rectangle; 64],
    window_size: [f64; 2],
    piece_textures: [Texture; 12],
}

const OPENGL: OpenGL = OpenGL::V2_1;

impl ChessGame {
    fn make_game() -> ChessGame {
        let window: PistonWindow = WindowSettings::new("Chess Engine", [1000, 1000])
                .graphics_api(OPENGL)
                .exit_on_esc(true)
                .build()
                .unwrap();
        window.set_lazy(true);
        ChessGame { 
            gl: GlGraphics::new(OPENGL), 
            window: window,
            events: Events::new(EventSettings::new()),
            board: Board::default(), 
            cursor: [0.0, 0.0], 
            highlights: [false; 64], 
            squares_initialized: false, 
            squares: [[0.0; 4]; 64], 
            window_size: [0.0, 0.0],
        }

        Texture::from_path()
    }
    
    fn render(&mut self, args: &RenderArgs) {
        self.window_size = args.window_size;
        self.render_board(args);
    }

    fn render_board(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        const WHITE: [f32; 4] = [0.7,0.7,0.7,1.0];
        const BLACK: [f32; 4] = [0.3,0.3,0.3,1.0];
        const WHITE_HIGHLIGHTED: [f32; 4] = [1.0,1.0,1.0,1.0];
        const BLACK_HIGHLIGHTED: [f32; 4] = [0.0,0.0,0.0,1.0];

        if self.squares_initialized == false {
            self.squares = ChessGame::init_board_squares();
        }

        let (x_centre, y_centre) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let x_board = 0.0;
        let y_board = 0.0;

        self.gl.draw(args.viewport(), |c, gl | {
            clear(BACKGROUND, gl);

            let transform = c.transform.trans(x_centre,y_centre).trans(x_board, y_board);
            let mut i = 0;
            while i < self.squares.len() {
                let r = rank(i);
                let render_option = ((i + (r as usize)) % 2) | (self.highlights[i] as usize) << 1;
                let colour = match render_option {
                    0b00 => WHITE,
                    0b01 => BLACK,
                    0b10 => WHITE_HIGHLIGHTED,
                    0b11 => BLACK_HIGHLIGHTED,
                    _ => {
                        i+=1; 
                        continue;
                    },
                };
                rectangle(colour, self.squares[i],  transform, gl);
                image()
                i+=1;
            }
        });

    }

    fn on_click(&mut self, button: &MouseButton) {
        println!("Pressed Button {:?} at pos {:}, {:}", button, self.cursor[0], self.cursor[1]);
        let (x_board_frame, y_board_frame) = (self.cursor[0] - self.window_size[0] / 2.0, self.cursor[1] - self.window_size[1] / 2.0);
        let square = self.resolve_square(x_board_frame, y_board_frame);
        match square {
            Some(sq) => println!("Pressed Square {:}", sq),
            None => {},
        }
        
        self.update_highlights(square);

    }

    fn update_highlights(&mut self, square_op: Option<usize>) {
        self.highlights = [false; 64];
        let square = match square_op {
            Some(sq) => sq,
            None => {
                return;
            }
        };

        let piece_colour = match board::get_piece_at_square(square, self.board) {
            Some(p) => p,
            None => {
                return;
            }
        };

        let mut targets = board::legal_targets(square, piece_colour.0, piece_colour.1, &self.board);
        while targets != 0 {
            let index = bitboard::pop(&mut targets);
            self.highlights[index] = true;
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
        const BOARD_SIZE: f64 = 1000.0;
        const SQUARE_SIZE: f64 = BOARD_SIZE / 8.0;

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
        if let Some(args) = e.render_args() {
            self.render(&args);
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
    while let Some(e) = game.events.next(&mut game.window){
        game.update(&e);
    }
    
    
}