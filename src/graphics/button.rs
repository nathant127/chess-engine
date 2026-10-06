use crate::graphics::{graphics::*, text::Text};
use piston::{Event, MouseButton};
use crate::game::gui::State;
use piston_window::*;




pub struct Button {
    text: Text,
    colour: [f32; 4],
    hover_colour: [f32; 4],

    rect: types::Rectangle,
}



impl Button {
    pub fn new(text: &str, top_left: [f64; 2], padding: [f64; 2], colour: [f32; 4], hover_colour: [f32; 4]) -> Button{

        // let text_origin = [top_left[0] + padding[0], top_left[1] + padding[1]];
        let text_origin = [top_left[0] + padding[0], top_left[1] + padding[1]];

        let text = Text::new(text, text_origin, [1.0, 1.0, 1.0, 1.0], "Montserrat-VariableFont_wght.ttf", 24);

        let text_box = text.get_text_box();

        let (width, height) = (padding[0] * 2.0 + text_box[2], padding[1] * 2.0 + text_box[3]);


        let button = Button {
            text: text,
            colour: colour,
            hover_colour: hover_colour,

            rect: [top_left[0], top_left[1], width, height],
        };

        return button;
    }

    pub fn is_mouse_over_button(&self, cursor: [f64; 2]) -> bool {
        is_inside_rect(self.rect, cursor)
    }
}

impl GraphicsObject for Button {
    fn render(&mut self, e: &Event, _state: &State, gui: &mut Gui) {
        let hovered = self.is_mouse_over_button(gui.cursor);
        let colour = if hovered { self.hover_colour } else { self.colour };


        gui.window.draw_2d(e, |c, g, _| {
            rectangle(colour, self.rect, c.transform, g);
        });

        self.text.render(e, _state, gui);
    }

    fn on_click(&mut self, _button: &MouseButton, _state: &mut State, _gui: &Gui) {

    }

    fn update(&mut self, _state: &mut State, _gui: &Gui) {
        
    }
}