use piston::{Event, MouseButton};
use piston::input::RenderEvent;
use piston_window::*;
use sprite::*;

use crate::game::button::Button;
use crate::game::gui::*;

const BACKGROUND: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

pub struct MainMenu {
    test_button: Button
}
impl MainMenu {
    pub fn new() -> MainMenu {
        const WHITE: [f32; 4] = [0.7,0.7,0.7,1.0];
        const BLACK: [f32; 4] = [0.3,0.3,0.3,1.0];
        let mm = MainMenu {
            test_button: Button::new("Test", [50.0, 50.0], [50.0, 30.0], WHITE, BLACK),
        };

        return mm;
    }
}

impl GraphicsObject for MainMenu {
    fn render(&mut self, e: &Event, state: &State, gui: &mut Gui) {
        gui.window.draw_2d(e, |c, g, _| {
            clear(BACKGROUND, g);
        });
        self.test_button.render(e, state, gui);
    }
    fn on_click(&mut self, button: &MouseButton, state: &mut State, gui: &Gui) {
        if self.test_button.is_mouse_over_button(gui.cursor) {
            state.next_scene = 1;
        }
    }
}