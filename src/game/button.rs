use piston::{Event, MouseButton};
use piston_window::*;

use crate::game::gui::*;


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
    pub fn new(text: &str, centre: [f64; 2], side_len: [f64; 2], colour: [f32; 4], hover_colour: [f32; 4]) -> Button{
        let button = Button {
            text: text.to_string(),
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
    fn render(&mut self, e: &Event, _state: &State, gui: &mut Gui) {
        let hovered = self.is_mouse_over_button(gui.cursor);
        let colour = if hovered { self.hover_colour } else { self.colour };

        let font_size: u32 = 24;
        let text_colour = [1.0, 1.0, 1.0, 1.0];

        let font_ptr = gui.resource_manager.get_font("Montserrat-VariableFont_wght.ttf");
        let mut font = font_ptr.borrow_mut();

        gui.window.draw_2d(e, |c, g, device| {
            rectangle(colour, self.rect, c.transform, g);

            // Measure the text so we can centre it horizontally
            let width = font.width(font_size, &self.text).unwrap_or(0.0);

            // Text is drawn with its *baseline* at the transform origin,
            // so nudge down by roughly a third of the font size to centre vertically
            let transform = c.transform.trans(
                self.pos[0] - width / 2.0,
                self.pos[1] + font_size as f64 / 3.0,
            );
            
            text::Text::new_color(text_colour, font_size)
                .draw(&self.text, &mut *font, &c.draw_state, transform, g)
                .unwrap();

            // Required: uploads the glyph textures to the GPU
            font.factory.encoder.flush(device);
        });
    }

    fn on_click(&mut self, _button: &MouseButton, _state: &mut State, _gui: &Gui) {

    }

    fn update(&mut self, _state: &mut State, _gui: &Gui) {
        
    }
}