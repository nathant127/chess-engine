extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate find_folder;
extern crate sprite;
extern crate piston_window;
extern crate uuid;

use std::cell::RefCell;

use piston_window::*;

use opengl_graphics::OpenGL;

use piston::input::RenderEvent;
use piston::window::WindowSettings;

use crate::game::game::{ChessGame, Player};
use crate::game::main_menu::MainMenu;

use crate::graphics::graphics::*;
use crate::graphics::resource_manager::RESOURCE_MANAGER;

#[derive(Default)]
pub struct State {
    pub current_scene: i64,
    pub next_scene: i64,

    pub chosen_players: [Player; 2],
}

thread_local! {
    pub static GAME_STATE: RefCell<State> = RefCell::new(State::default());
}

pub struct ChessGui {
    gui: Gui,

    scenes: Vec<Box<dyn GraphicsObject>>,
}

const OPENGL: OpenGL = OpenGL::V3_2;


impl ChessGui {
    pub fn new() -> ChessGui {
        let mut window: PistonWindow = WindowSettings::new("Chess Engine", [1000, 1000])
            .graphics_api(OPENGL)
            .exit_on_esc(true)
            .build()
            .expect("Failed to create piston window");
        window.set_lazy(false);

        let texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };

        let mut gui = Gui {
            window: window,
            cursor: [0.0, 0.0], 
            window_size: [0.0, 0.0],
        };

        RESOURCE_MANAGER.with_borrow_mut(|r|
            {
                r.init(texture_context);
                if let Err(e) = r.add_folder("assets") {
                    println!("Failed to find the assets folder, is it included with the executable? {e}");
                }
                r.add_font("assets/fonts/Montserrat/Montserrat-VariableFont_wght.ttf", &mut gui.window);
            }
        );
        
        let scenes: Vec<Box<dyn GraphicsObject>> = vec![
            Box::new(MainMenu::new()),
            Box::new(ChessGame::new(Player::LocalPlayer, Player::LocalPlayer)),
        ];

        let chess_gui = ChessGui { 
            gui: gui,

            scenes: scenes,
        };

        return chess_gui;
    }

    pub fn run(&mut self) {
        while let Some(e) = self.gui.window.next() {
            self.handle_events(&e);
        }   
    }
    
    fn render(&mut self, e: &Event) {
        let args = e.render_args().unwrap();
        self.gui.window_size = args.window_size;

        let cur_scene = GAME_STATE.with_borrow(|s| s.current_scene);
        self.scenes[cur_scene as usize].as_mut().render(e, &mut self.gui);        
    }

    fn on_click(&mut self, button: &MouseButton) {
        let cur_scene = GAME_STATE.with_borrow(|s| s.current_scene);
        self.scenes[cur_scene as usize].as_mut().on_click(button, &self.gui);           

    }

    fn update(&mut self, _e: &Event) {
        let cur_scene = GAME_STATE.with_borrow(|s| s.current_scene);
        self.scenes[cur_scene as usize].as_mut().update(&mut self.gui);
    }

    fn handle_events(&mut self, e: &Event) {
        GAME_STATE.with_borrow_mut(|state| {
            if state.current_scene != state.next_scene {
                if state.next_scene == 1 {
                    self.scenes[1] = Box::new(ChessGame::new(state.chosen_players[0], state.chosen_players[1]));
                }

                state.current_scene = state.next_scene;
            }
        });
        

        if let Some(_args) = e.update_args() {
            self.update(&e);
        }

        if let Some(_args) = e.render_args() {
            self.render(&e);
        }

        // Update mouse cursor pos every frame
        e.mouse_cursor(|pos| {
            self.gui.cursor = pos;
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