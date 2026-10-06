use std::{cell::RefCell, rc::Rc};

use crate::graphics::graphics::*;
use piston::{Event, MouseButton};
use crate::game::gui::State;
use piston_window::*;

use crate::graphics::resource_manager::RESOURCE_MANAGER;




pub struct Text {
    text: String,
    origin: [f64; 2],
    colour: types::Color,
    font_size: u32,
    text_box: types::Rectangle,

    font_ptr: Rc<RefCell<Glyphs>>,

}



impl Text {
    pub fn new(text: &str, top_left: [f64; 2], colour: [f32; 4], font: &str, font_size: u32) -> Text {
        let font_ptr = RESOURCE_MANAGER.with_borrow(|r| 
            r.get_font(font)
        );

        let (width, top_extent, bottom_extent) = {
            let mut font = font_ptr.borrow_mut();
    
            let width = font.width(font_size, text).unwrap_or(0.0);
            let (top_extent, bottom_extent) = Text::text_extents(&mut *font, font_size, text);

            (width, top_extent, bottom_extent)
        };
        
        let height = top_extent + bottom_extent;
        let origin = [top_left[0], top_left[1] + top_extent];

        

        let button = Text {
            text: text.to_string(),
            origin: origin,
            colour: colour,
            font_size: font_size,
            text_box: [top_left[0], top_left[1], width, height],

            font_ptr: font_ptr,
        };

        return button;
    }

    pub fn get_text_box(&self) -> types::Rectangle {
        self.text_box
    }

    fn text_extents<C: CharacterCache>(cache: &mut C, size: u32, text: &str) -> (f64, f64) {
        let mut above = 0.0f64;
        let mut below = 0.0f64;
        for ch in text.chars() {
            if let Ok(c) = cache.character(size, ch) {
                let top = c.top();
                above = above.max(top);
                below = below.max(c.atlas_size[1] - top);
            }
        }
        (above, below)
    }
}

impl GraphicsObject for Text {
    fn render(&mut self, e: &Event, _state: &State, gui: &mut Gui) {

        
        let mut font = self.font_ptr.borrow_mut();
        gui.window.draw_2d(e, |c, g, device| {        
            let transform = c.transform.trans(
                self.origin[0],
                self.origin[1],
            );
            text::Text::new_color(self.colour, self.font_size)
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