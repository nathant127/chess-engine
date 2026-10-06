use piston::{Event, MouseButton};
use piston_window::*;

use crate::graphics::graphics::GraphicsObject;
use crate::graphics::button::Button;
use crate::game::game::Player;
use crate::game::gui::{GAME_STATE};
use crate::graphics::graphics::*;

const BACKGROUND: [f32; 4] = [0.7, 0.5, 0.5, 1.0];

pub struct MainMenu {
    play_local_button: Button,
    play_bot_button: Button,
    play_multi_button: Button,
}
impl MainMenu {
    pub fn new() -> MainMenu {
        const COLOUR: [f32; 4] = [0.3,0.7,0.3,1.0];
        const HOVER_COLOUR: [f32; 4] = [0.3,0.5,0.3,1.0];
        let mm = MainMenu {
            play_local_button: Button::new("Play Local", [10.0, 10.0], [10.0, 10.0], COLOUR, HOVER_COLOUR, MainMenu::play_local_cb),
            play_bot_button: Button::new("Play Bot", [10.0, 100.0], [10.0, 10.0], COLOUR, HOVER_COLOUR, MainMenu::play_bot_cb),
            play_multi_button: Button::new("Play Multiplayer", [10.0, 190.0], [10.0, 10.0], COLOUR, HOVER_COLOUR, MainMenu::play_multi_cb),

        };

        return mm;
    }

    fn play_local_cb() {
        GAME_STATE.with_borrow_mut(|s| {
            s.next_scene = 1;
            s.chosen_players = [Player::LocalPlayer, Player::LocalPlayer];
        });
    }

    fn play_bot_cb() {
        GAME_STATE.with_borrow_mut(|s| {
            s.next_scene = 1;
            s.chosen_players = [Player::LocalPlayer, Player::Bot];
        });
    }

    fn play_multi_cb() {
        GAME_STATE.with_borrow_mut(|s| {
            s.next_scene = 1;
            s.chosen_players = [Player::LocalPlayer, Player::RemotePlayer];
        });
    }
}

impl GraphicsObject for MainMenu {
    fn render(&mut self, e: &Event, gui: &mut Gui) {
        gui.window.draw_2d(e, |_c, g, _| {
            clear(BACKGROUND, g);
        });
        self.play_local_button.render(e, gui);
        self.play_bot_button.render(e, gui);
        self.play_multi_button.render(e,  gui);
    }
    fn on_click(&mut self, button: &MouseButton, gui: &Gui) {
        self.play_local_button.on_click(button, gui);
        self.play_bot_button.on_click(button, gui);
        self.play_multi_button.on_click(button, gui);

    }
    fn update(&mut self, _gui: &Gui) {

    }
}