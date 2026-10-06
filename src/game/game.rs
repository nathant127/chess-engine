use rand::prelude::IndexedRandom;

use piston::{Event, MouseButton};
use piston::input::RenderEvent;
use piston_window::*;
use sprite::*;

use crate::board_rep::bitboard::{self, BitBoard};
use crate::board_rep::board::*;
use crate::board_rep::constants::{Piece, Colour};
use crate::graphics::button::Button;
use crate::game::gui::*;
use crate::game::networking::{ChessClient, Message};
use uuid::Uuid;

use crate::graphics::graphics::*;

struct SceneMetaData {
    id: Uuid,
    square: usize,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Player {
    LocalPlayer,
    Bot,
    RemotePlayer
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::LocalPlayer => write!(f, "LocalPlayer"),
            Player::Bot => write!(f, "Bot"),
            Player::RemotePlayer => write!(f, "RemotePlayer"),
        }
    }
}

type Rectangle = [f64;4];
const SQUARE_SIZE: f64 = 100.0;

pub struct ChessGame {
    board: Board,
    turn: u32,
    white_player: Player,
    black_player: Player,
    in_game: bool,

    scene: Scene<G2dTexture>,
    scene_metadata: Vec<SceneMetaData>,

    highlights: [bool; 64],
    selected_piece_square: Option<usize>,
    squares: [Rectangle; 64],

    exit_button: Button,

    net_client: Option<ChessClient>
}

impl ChessGame {
    pub fn new(gui: &Gui, white_player: Player, black_player: Player) -> ChessGame {
        
        let net_client = if white_player == Player::RemotePlayer || black_player == Player::RemotePlayer {
            Some(ChessClient::new("127.0.0.1:25800".to_string()).unwrap())
        }
        else {
            None
        };

        let mut game  = ChessGame {
            board: Board::default(),
            turn: 0,
            white_player: white_player,
            black_player: black_player,
            in_game: false,

            scene: Scene::new(),
            scene_metadata: Vec::new(),

            highlights: [false; 64], 
            selected_piece_square: None,
            squares: ChessGame::init_board_squares(),

            exit_button: Button::new("X", [25.0, 25.0], [50.0, 50.0], [1.0, 0.0, 0.0, 1.0], [0.7, 0.0, 0.0, 1.0]),

            net_client: net_client,
        };

        game.create_initial_scene(gui);

        return game;
    }

    pub fn get_board(&self) -> Board {
        return self.board;
    }

    pub fn try_move(&mut self, mov: Move) -> Option<[bool; 6]> {
        if self.is_move_valid( mov) == false {
            return None;
        }
        let mut special_actions = [false; 6];
        let mut board_copy: Board = self.board;
        board_copy = board_copy & (1<<mov.tgt_square);
        if all(&board_copy) != 0 {
            special_actions[SpecialActions::Capture as usize] = true;
        }

        // TODO: Handle other special Actions

        self.board = self.board & !(1<<mov.tgt_square);

        let (piece, colour) = get_piece_at_square(mov.org_square, self.board).unwrap();

        let piece_bb: &mut BitBoard = self.board.get_bb_mut(piece, colour);
        *piece_bb &= !(1<<mov.org_square);
        *piece_bb |= 1<<mov.tgt_square;

        self.turn += 1;

        self.update_with_move(mov, special_actions);
        return Some(special_actions);
    }

    pub fn is_move_valid(&mut self, mov: Move) -> bool {
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

    fn create_initial_scene(&mut self, gui: &Gui){
        self.fill_scene_piece(self.board.pw, "white_pawn.png", gui);
        self.fill_scene_piece(self.board.nw, "white_knight.png", gui);
        self.fill_scene_piece(self.board.bw, "white_bishop.png", gui);
        self.fill_scene_piece(self.board.rw, "white_rook.png", gui);
        self.fill_scene_piece(self.board.qw, "white_queen.png", gui);
        self.fill_scene_piece(self.board.kw, "white_king.png", gui);

        self.fill_scene_piece(self.board.pb, "black_pawn.png", gui);
        self.fill_scene_piece(self.board.nb, "black_knight.png", gui);
        self.fill_scene_piece(self.board.bb, "black_bishop.png", gui);
        self.fill_scene_piece(self.board.rb, "black_rook.png", gui);
        self.fill_scene_piece(self.board.qb, "black_queen.png", gui);
        self.fill_scene_piece(self.board.kb, "black_king.png", gui);
    }

    fn fill_scene_piece(&mut self, mut bitboard: u64, tex_name: &str, gui: &Gui) {
        let tex = gui.resource_manager.get_texture(tex_name);
        while bitboard != 0 {
            let index = bitboard::pop(&mut bitboard);
            let mut sprite = Sprite::from_texture(tex.clone());
            sprite.set_position(self.squares[index][0] + SQUARE_SIZE/2.0, self.squares[index][1] + SQUARE_SIZE/2.0);
            let id = self.scene.add_child(sprite);
            self.scene_metadata.push(SceneMetaData { id: id, square: index });
        }
    }

    fn update_highlights(&mut self) {
        let square = match self.selected_piece_square {
            Some(sq) => sq,
            None => {
                self.highlights = [false; 64];
                return;
            }
        };

        let mut targets = self.square_possible_targets(square);
        while targets != 0 {
            let index = bitboard::pop(&mut targets);
            self.highlights[index] = true;
        }
    }

    fn init_board_squares() -> [Rectangle; 64] {
        let mut squares: [Rectangle; 64] = [[0.0; 4]; 64];
        const BOARD_SIZE: f64 = SQUARE_SIZE * 8.0;
        

        let mut i = 0;
        while i < squares.len() {
            let r = bitboard::rank(i) as f64;
            let f = bitboard::file(i) as f64;

            let x = (f * SQUARE_SIZE) - BOARD_SIZE/2.0;
            let y = (r * SQUARE_SIZE) - BOARD_SIZE/2.0;
            squares[i ^ 56] = graphics::rectangle::square(x, y, SQUARE_SIZE);
            i += 1;
        }


        return squares;
    }

    #[inline]
    fn is_inside_square(x: f64, y: f64, x_sq: f64, y_sq: f64, size: f64) -> bool {
        (x >= x_sq) && (x < (x_sq + size)) && (y >= y_sq) && (y < (y_sq + size))
    }

    fn resolve_square(& self, x: f64, y: f64) -> Option<usize> {
        for (i, square) in self.squares.iter().enumerate() {
            if ChessGame::is_inside_square(x, y, square[0], square[1], square[2]) {
                return Some(i);
            }
        }

        return None;
    }

    fn update_with_move(&mut self, mov: Move, special_actions: [bool; 6]) {
        
        // Remove captured piece
        if special_actions[SpecialActions::Capture as usize] == true {
            for (i,md) in self.scene_metadata.iter_mut().enumerate() {
                if md.square == mov.tgt_square {
                    self.scene.remove_child(md.id);
                    self.scene_metadata.remove(i);
                    break;
                }
            }
        }

        // TODO: Add checkmate popup
        
        for md in self.scene_metadata.iter_mut() {
            if md.square == mov.org_square {
                md.square = mov.tgt_square;
                self.scene.child_mut(md.id).unwrap().set_position(self.squares[md.square][0] + SQUARE_SIZE/2.0, self.squares[md.square][1] + SQUARE_SIZE/2.0);
            }
        }

        if let Some(nc) = &self.net_client {
            nc.send_move(mov);
        }

    }

    fn cur_player(&self) -> Player {
        if self.turn % 2 == 0 {
            self.white_player
        }
        else {
            self.black_player
        }
    }
}


impl GraphicsObject for ChessGame {
    fn render(&mut self, e: &Event, state: &State, gui: &mut Gui) {
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

        // Update what squares are highlighted based on the selected piece
        self.update_highlights();

        gui.window.draw_2d(e, |c, g, _| {
            clear(BACKGROUND, g);

            let transform = c.transform.trans(x_centre,y_centre).trans(x_board, y_board);
            let mut i = 0;
            while i < self.squares.len() {
                let r = bitboard::rank(i);
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

        self.exit_button.render(e, state, gui);
    }
    fn on_click(&mut self, _button: &MouseButton, state: &mut State, gui: &Gui) {

        if self.exit_button.is_mouse_over_button(gui.cursor) {
            state.next_scene = 0;
        }

        // If we are in multiplayer, but not connected yet, dont let player touch board
        if let Some(_) = self.net_client  {
            if self.in_game == false {
                return;
            }
        }
        
        // If the current player is a local player
        if self.cur_player() == Player::LocalPlayer {

            // Check if they clicked on board, and if so what board they clicked on
            let (x_board_frame, y_board_frame) = (gui.cursor[0] - gui.window_size[0] / 2.0, gui.cursor[1] - gui.window_size[1] / 2.0);
            let square_op: Option<usize> = self.resolve_square(x_board_frame, y_board_frame);
            
            
            if let Some(clicked_square) = square_op {
    
                if let Some(sel_square) = self.selected_piece_square  {
                    let mov: Move = Move {
                        org_square: sel_square, 
                        tgt_square: clicked_square, 
                    };

                    self.try_move(mov);
                    self.selected_piece_square = None;
                    return;
                }
                
                if self.square_can_move(clicked_square) {
                    self.selected_piece_square = Some(clicked_square);
                }
            }
            else {
                self.selected_piece_square = None;
            }
        }
        
    }

    fn update(&mut self, state: &mut State, _gui: &Gui) {

        
        // Handle multiplayer connection
        while let Some(msg) = self.net_client.as_mut().and_then(|nc| nc.poll_events()) {
            match msg {
                Message::Connected => {
                    self.net_client.as_ref().unwrap().request_match("apple");
                },
                Message::Error(e) => {
                    println!("{e}");
                },
                Message::Disconnected => {
                    self.in_game = false;
                    state.next_scene = 0;
                },
                Message::EndMatch => {
                    self.in_game = false;
                    state.next_scene = 0;
                },
                Message::StartGame(turn) => {
                    self.in_game = true;
                    if turn == 0 {
                        self.white_player = Player::LocalPlayer;
                        self.black_player = Player::RemotePlayer;
                    }
                    else {
                        self.white_player = Player::RemotePlayer;
                        self.black_player = Player::LocalPlayer;
                    }
                },
                Message::Move(mov) => {
                    if self.in_game == true && self.cur_player() == Player::RemotePlayer {
                        self.try_move(mov);
                    }
                },
                _ => {
                    println!("Can't handle that message type");
                }
            }
        }
        

        // Handle bot players
        if self.cur_player() == Player::Bot {
            // Get the pieces that correspond to the current turn
            let mut possible_pieces = match self.turn % 2 {
                0 => white(&self.board),
                _ => black(&self.board),
            };


            // Create a vector of each possible piece
            let mut squares: Vec<usize> = Vec::new();
            while possible_pieces != 0 {
                squares.push(bitboard::pop(&mut possible_pieces));
            }

            // Remove squares that can't move
            squares.retain(|&sq| self.square_can_move(sq));

            if squares.len() > 0 {
                let mut rng = rand::rng();
                let sq = squares.choose(&mut rng).unwrap();
                
                let mut targets_bb = self.square_possible_targets(*sq);
                let mut targets: Vec<usize> = Vec::new();
                while targets_bb != 0 {
                    targets.push(bitboard::pop(&mut targets_bb));
                }
                let tgt = targets.choose(&mut rng).unwrap();

                let mov = Move {
                    org_square: *sq,
                    tgt_square: *tgt,
                };
                self.try_move(mov);
            }
        }
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

#[derive(Eq, PartialEq, Clone, Copy)]
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


