use crate::game::gui::State;
use graphics::types;
use piston_window::{PistonWindow, Event, MouseButton};

pub struct Gui {
    pub window: PistonWindow,

    pub cursor: [f64; 2],
    pub window_size: [f64; 2],
}

pub trait GraphicsObject {
    fn render(&mut self, e: &Event, state: &State, gui: &mut Gui);
    fn on_click(&mut self, button: &MouseButton, state: &mut State, gui: &Gui);
    fn update(&mut self, state: &mut State, gui: &Gui);
}


pub fn is_inside_rect(rect: types::Rectangle, pos: [f64; 2]) -> bool {
    (pos[0] > rect[0]) && (pos[0] < rect[0] + rect[2]) && (pos[1] > rect[1]) && (pos[1] < rect[1] + rect[3])
}