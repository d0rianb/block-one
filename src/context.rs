use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;

use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;
use speedy2d::window::MouseButton;

use crate::block::Block;
use crate::link::Link;

pub struct Context {
    blocks: Vec<Rc<RefCell<Block>>>,
    links: Vec<Link>,
    pub drag: bool,
    pub mouse_position: Vector2<f32>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            links: vec![],
            drag: false,
            mouse_position: Vector2::ZERO,
        }
    }

    pub fn on_keydown(&mut self, string: String) {
        match string.as_ref() {
            "n" | "a" => self.add_block(),
            "l" => self.add_link(),
            _ => {dbg!(string);}
        }
    }

    pub fn on_mouse_clicked(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.blocks.iter().for_each(|block| block.borrow_mut().is_focused = false); // TODO: check if shift is pressed
                let clicked_block = self.get_block_at(self.mouse_position);
                if clicked_block.is_none() { return; }
                let block = Rc::clone(clicked_block.as_ref().unwrap());
                block.borrow_mut().toggle_focus();
                drop(clicked_block);
                let last_link = self.links.last_mut();
                if let Some(link) = last_link {
                    if link.to.is_none() {
                        link.to(block.clone());
                    }
                }
            }
            _ => {}
        }
    }

    fn add_block(&mut self) {
        let block = Block::new(self.mouse_position);
        self.blocks.push(Rc::new(RefCell::new(block)));
    }

    fn add_link(&mut self) {
        let focused_block = self.get_focused_blocks();
        for block in focused_block {
            self.links.push(Link::new(block));
        }
    }

    pub fn move_block(&mut self, new_position: Vector2<f32>) {
        let delta = new_position - self.mouse_position;
        self.get_focused_blocks().iter().for_each(|block| {
            let old_pos = block.borrow_mut().pos.clone();
            block.borrow_mut().pos= old_pos.add(delta);
        });
    }

    pub fn delete_focused_block(&mut self) {
        let focused_blocks = self.get_focused_blocks();
        let mut block_remove_indices = vec![];
        let mut link_remove_indices = vec![];
        for block in focused_blocks.iter() {
            let index = self.blocks.iter().position(|context_block|Rc::ptr_eq(block, context_block));
            if let Some(i) = index { block_remove_indices.push(i); }
            for (i, link) in self.links.iter().enumerate() {
                if Rc::ptr_eq(&link.from, block) { link_remove_indices.push(i); }
                if let Some(to) = &link.to {
                    if Rc::ptr_eq(&to, block) { link_remove_indices.push(i); }
                }
            }
        }
        block_remove_indices.dedup();
        link_remove_indices.dedup();
        for i in block_remove_indices.iter().rev() { self.blocks.remove(*i); }
        for i in link_remove_indices.iter().rev() { self.links.remove(*i); }
    }

    fn get_block_at(&mut self, pos: Vector2<f32>) -> Option<Rc<RefCell<Block>>> {
        let block = self.blocks
            .iter()
            .find(|block| {
                let block = block.borrow();
                block.pos.x < pos.x && block.pos.y < pos.y && block.pos.x + block.width > pos.x && block.pos.y + block.height > pos.y
            });
        match block {
            Some(block) => Some(Rc::clone(block)),
            None => None
        }
    }

    fn get_focused_blocks(&mut self) -> Vec<Rc<RefCell<Block>>> {
        self.blocks.iter_mut()
            .filter(|block| block.borrow().is_focused)
            .map(|block| Rc::clone(block))
            .collect()
    }

    pub fn update(&mut self, _dt: f32) {}

    pub fn render(&mut self, graphics: &mut Graphics2D) {
        for block in &self.blocks {
            block.borrow().render(graphics);
        }

        for link in &self.links {
            link.render(self.mouse_position, graphics);
        }
    }
}