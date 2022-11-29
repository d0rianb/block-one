use std::cell::{Ref, RefCell};
use std::convert::AsRef;
use std::ops::{Deref, Mul};
use std::ptr;
use std::rc::Rc;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;

use crate::block::Block;
use crate::render_helper::draw_bezier_curve;

pub struct Link {
    pub from: Rc<RefCell<Block>>,
    pub to: Option<Rc<RefCell<Block>>>,
}

impl Link {
    pub fn new(block: Rc<RefCell<Block>>) -> Self {
        Self {
            from: block,
            to: None
        }
    }

    pub fn to(&mut self, to: Rc<RefCell<Block>>) {
        if Rc::ptr_eq(&to, &self.from) { return; }
        self.to = Some(to);
    }

    pub fn render(&self, mouse_pos: Vector2<f32>, graphics: &mut Graphics2D) {
        let from_block = self.from.borrow();
        let virtual_mouse_block = Block::new(mouse_pos); // Virtual block representing the cursor
        let virtual_mouse_ref= Rc::new(RefCell::new(virtual_mouse_block));
        let mut to_block = self.to.as_ref().unwrap_or(&virtual_mouse_ref).borrow();
        let dist = from_block.pos.x - to_block.pos.x;
        let offset = dist.abs() / 2.;
        let start = if dist > 0. { Ref::clone(&to_block) } else { Ref::clone(&from_block) };
        graphics.draw_circle(start.pos, 5., Color::GREEN); // DEBUG
        let end = if start.pos == from_block.pos { Ref::clone(&to_block) } else { Ref::clone(&from_block) };
        draw_bezier_curve(
            start.pos + Vector2::new(start.width, start.height / 2.),
            start.pos + Vector2::new(offset + start.width, start.height / 2.).mul(0.8), // control 1
            end.pos + Vector2::new(-offset, end.height / 2.).mul(0.8), // control 2
            end.pos + Vector2::new(0., end.height / 2.),
            graphics
        );
    }
}