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

use crate::game::game::{ChessGame, Player};
use crate::game::main_menu::MainMenu;

use crate::graphics::graphics::*;
use crate::graphics::resource_manager::ResourceManager;


pub struct State {
    pub current_scene: i64,
    pub next_scene: i64,

    pub chosen_players: [Player; 2],
}






pub struct ChessGui {
    state: State,

    gui: Gui,

    game: ChessGame,
    main_menu: MainMenu

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
            resource_manager: ResourceManager::new(texture_context),
            window: window,
            cursor: [0.0, 0.0], 
            window_size: [0.0, 0.0],
        };
        let result = gui.resource_manager.add_folder("assets");
        if let Err(e) = result {
            println!("Failed to find the assets folder, is it included with the executable? {e}");
        }

        gui.resource_manager.add_font("assets/fonts/Montserrat/Montserrat-VariableFont_wght.ttf", &mut gui.window);
        
        let game = ChessGame::new(&gui, Player::LocalPlayer, Player::LocalPlayer);

        let chess_gui = ChessGui { 
            state: State {current_scene: 0, next_scene: 0, chosen_players: [Player::LocalPlayer, Player::LocalPlayer]},
            
            gui: gui,
            
            game: game,
            main_menu: MainMenu::new(),
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

        match self.state.current_scene {
            0 => self.main_menu.render(e, &self.state, &mut self.gui),
            1 => self.game.render(e, &self.state, &mut self.gui),
            _ => {},
        }
        
    }

    fn on_click(&mut self, button: &MouseButton) {
        match self.state.current_scene {
            0 => self.main_menu.on_click(button, &mut self.state,  &self.gui),
            1 => self.game.on_click(button, &mut self.state, &self.gui),
            _ => {},
        }       

    }

    fn update(&mut self, _e: &Event) {
        match self.state.current_scene {
            0 => self.main_menu.update(&mut self.state,  &self.gui),
            1 => self.game.update(&mut self.state, &self.gui),
            _ => {},
        }    
    }

    fn handle_events(&mut self, e: &Event) {

        if self.state.current_scene != self.state.next_scene {
            if self.state.next_scene == 1 {
                self.game = ChessGame::new(&self.gui, self.state.chosen_players[0], self.state.chosen_players[1]);
            }

            self.state.current_scene = self.state.next_scene;
        }

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