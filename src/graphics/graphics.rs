use crate::game::gui::State;
use crate::graphics::resource_manager::ResourceManager;
use piston_window::{PistonWindow, Event, MouseButton};

pub struct Gui {
    pub resource_manager: ResourceManager,
    pub window: PistonWindow,

    pub cursor: [f64; 2],
    pub window_size: [f64; 2],
}

pub trait GraphicsObject {
    fn render(&mut self, e: &Event, state: &State, gui: &mut Gui);
    fn on_click(&mut self, button: &MouseButton, state: &mut State, gui: &Gui);
    fn update(&mut self, state: &mut State, gui: &Gui);
}
