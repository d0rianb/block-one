use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::render_helper::{draw_rounded_rectangle_with_border};

#[derive(Copy, Clone)]
pub struct Block {
    pub pos: Vector2<f32>,
    pub width: f32,
    pub height: f32,
    pub is_focused: bool,
}

impl Default for Block {
    fn default() -> Self {
        Self::new(Vector2::new(400., 200.))
    }
}

impl Block {
    pub fn new(pos: Vector2<f32>) -> Self {
        Self {
            pos,
            width: 150.,
            height: 80.,
            is_focused: false
        }
    }

    pub fn toggle_focus(&mut self) {
        self.is_focused = !self.is_focused;
    }

    pub fn render(&self, graphics: &mut Graphics2D) {
        let border_color = if self.is_focused { Color::BLACK } else { Color::from_rgb(100., 100., 100.) };
        draw_rounded_rectangle_with_border(self.pos.x, self.pos.y, self.width, self.height, 5., 0.5, Color::LIGHT_GRAY, border_color, graphics);
    }
}