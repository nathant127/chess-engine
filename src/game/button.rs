use piston::{Event, MouseButton};
use piston::input::RenderEvent;
use piston_window::*;
use sprite::*;

use crate::board_rep::bitboard::{self, BitBoard};
use crate::board_rep::board::*;
use crate::board_rep::constants::{Piece, Colour};
use crate::game::{button, gui::*};


pub struct Button {
    text: String,
    pos: [f64; 2],
    bounds: [f64; 2],
    colour: [f32; 4],
    hover_colour: [f32; 4],

    rect: Rectangle,
}

type Rectangle = [f64;4];

impl Button {
    pub fn new(text: String, centre: [f64; 2], side_len: [f64; 2], colour: [f32; 4], hover_colour: [f32; 4]) -> Button{
        let button = Button {
            text: text,
            pos: centre,
            bounds: [side_len[0] / 2.0, side_len[1] / 2.0],
            colour: colour,
            hover_colour: hover_colour,

            rect: graphics::rectangle::centered([centre[0], centre[1], side_len[0] / 2.0, side_len[1] / 2.0]),
        };

        return button;
    }

    pub fn is_mouse_over_button(&self, cursor: [f64; 2]) -> bool {
        let button_frame = [cursor[0] - self.pos[0], cursor[1] - self.pos[1]];
        let is_inside_button = (button_frame[0].abs() < self.bounds[0]) && (button_frame[1].abs() < self.bounds[1]);
        return is_inside_button;
    }
}

impl GraphicsObject for Button {
    fn render(&mut self, e: &Event, state: &State, gui: &mut Gui) {
        gui.window.draw_2d(e, |c, g, _| {

            let transform = c.transform;

            let colour = match self.is_mouse_over_button(gui.cursor) {
                true => self.hover_colour,
                false => self.colour,
            };
            
            rectangle(colour, self.rect,  transform, g);
        });
    }
    fn on_click(&mut self, button: &MouseButton, state: &mut State, gui: &Gui) {

    }
}