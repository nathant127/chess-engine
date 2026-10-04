use piston::{Event, MouseButton};
use piston_window::*;

use crate::game::button::Button;
use crate::game::game::Player::{self, LocalPlayer};
use crate::game::gui::*;

const BACKGROUND: [f32; 4] = [0.7, 0.5, 0.5, 1.0];

pub struct MainMenu {
    play_local_button: Button,
    play_bot_button: Button,
}
impl MainMenu {
    pub fn new() -> MainMenu {
        const COLOUR: [f32; 4] = [0.3,0.7,0.3,1.0];
        const HOVER_COLOUR: [f32; 4] = [0.3,0.5,0.3,1.0];
        let mm = MainMenu {
            play_local_button: Button::new("Play Local", [80.0, 30.0], [150.0, 50.0], COLOUR, HOVER_COLOUR),
            play_bot_button: Button::new("Play Bot", [80.0, 90.0], [150.0, 50.0], COLOUR, HOVER_COLOUR),

        };

        return mm;
    }
}

impl GraphicsObject for MainMenu {
    fn render(&mut self, e: &Event, state: &State, gui: &mut Gui) {
        gui.window.draw_2d(e, |c, g, _| {
            clear(BACKGROUND, g);
        });
        self.play_local_button.render(e, state, gui);
        self.play_bot_button.render(e, state, gui);
    }
    fn on_click(&mut self, button: &MouseButton, state: &mut State, gui: &Gui) {
        if self.play_local_button.is_mouse_over_button(gui.cursor) {
            state.next_scene = 1;
            state.chosen_players = [Player::LocalPlayer, Player::LocalPlayer];
        }

        if self.play_bot_button.is_mouse_over_button(gui.cursor) {
            state.next_scene = 1;
            state.chosen_players = [Player::LocalPlayer, Player::Bot];
        }
    }
    fn update(&mut self, state: &mut State, gui: &Gui) {

    }
}