use std::cell::RefCell;
use std::rc::Rc;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::FormattedTextBlock;
use speedy2d::Graphics2D;

use crate::font::Font;

const INITIAL_LINE_CAPACITY: usize = 2048;

pub(crate) struct Line {
    pub buffer: Vec<String>,
    pub font: Rc<RefCell<Font>>,
    pub wrap_y: u32,
    pub formatted_text_block: Rc<FormattedTextBlock>,
    previous_string: String,
}

impl Line {
    pub fn new(font: Rc<RefCell<Font>>) -> Self {
        let formatted_text_block = font.borrow().layout_text("");
        Line {
            buffer: Vec::with_capacity(INITIAL_LINE_CAPACITY),
            previous_string: String::new(),
            formatted_text_block,
            font,
            wrap_y: 0
        }
    }

    pub fn update_text_layout(&mut self) {
        let string = self.buffer.clone().join("");
        if string != self.previous_string {
            self.formatted_text_block = self.font.borrow().layout_text(&string);
            self.previous_string = string;
        }
    }

    pub fn render(&self, x: f32, y: f32, graphics: &mut Graphics2D) {
        graphics.draw_text(Vector2::new(x, y), Color::BLACK, &self.formatted_text_block);
    }

}