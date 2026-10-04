extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate find_folder;
extern crate sprite;
extern crate piston_window;
extern crate uuid;

use piston_window::*;

use opengl_graphics::OpenGL;

use piston::input::RenderEvent;
use piston::window::WindowSettings;

use crate::game::game::ChessGame;
use super::resource_manager::ResourceManager;



pub struct State {
    
}

pub trait GameScene {
    fn render(&mut self, e: &Event, state: &State, gui: &mut Gui);
    fn on_click(&mut self, button: &MouseButton, state: &mut State, gui: &Gui);
}


pub struct Gui {
    pub resource_manager: ResourceManager,
    pub window: PistonWindow,

    pub cursor: [f64; 2],
    pub window_size: [f64; 2],
}



pub struct ChessGui {
    state: State,

    gui: Gui,

    game: ChessGame,

}

const OPENGL: OpenGL = OpenGL::V3_2;


impl ChessGui {
    pub fn new() -> ChessGui {
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

        let mut gui = Gui {
            resource_manager: ResourceManager::new(texture_context),
            window: window,
            cursor: [0.0, 0.0], 
            window_size: [0.0, 0.0],
        };
        let result = gui.resource_manager.add_folder("assets");
        if let Err(e) = result {
            println!("Failed to find the assets folder, is it included with the executable? {e}");
        }
        
        let game = ChessGame::new(&gui);

        let chess_gui = ChessGui { 
            state: State {},
            
            gui: gui,
            
            game: game,
        };

        return chess_gui;
    }

    pub fn run(&mut self) {
        while let Some(e) = self.gui.window.next() {
            self.update(&e);
        }   
    }
    
    fn render(&mut self, e: &Event) {
        let args = e.render_args().unwrap();
        self.gui.window_size = args.window_size;
        self.game.render(e, &self.state, &mut self.gui);
    }

    fn on_click(&mut self, button: &MouseButton) {
        self.game.on_click(button, &mut self.state, &mut self.gui);        

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