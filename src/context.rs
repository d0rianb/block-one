pub struct Link;

use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;
use speedy2d::window::MouseButton;

use crate::block::Block;

pub struct Context {
    blocks: Vec<Block>,
    links: Vec<Link>,
    pub mouse_position: Vector2<f32>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            links: vec![],
            mouse_position: Vector2::new(0., 0.),
        }
    }

    pub fn on_keydown(&mut self, string: String) {
        match string.as_ref() {
            "n" | "a" => self.add_block(),
            _ => {}
        }
    }
    pub fn on_keypress(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.blocks.iter_mut().for_each(|block| block.is_focused = false); // TODO: check if shift is pressed
                if let Some(block) = self.get_block_at(self.mouse_position) {
                    block.toggle_focus();
                }
            },
            _ => {}
        }
    }

    fn add_block(&mut self) {
        let block = Block::new(self.mouse_position);
        self.blocks.push(block);
    }

    fn get_block_at(&mut self, pos: Vector2<f32>) -> Option<&mut Block> {
        self.blocks.iter_mut().find(|block| block.pos.x < pos.x && block.pos.y < pos.y
            && block.pos.x + block.width > pos.x && block.pos.y + block.height > pos.y)
    }

    fn get_focused_blocks(&mut self) -> Vec<&mut Block> {
        self.blocks.iter_mut().filter(|block| block.is_focused).collect()
    }

    pub fn update(&mut self, dt: f32) { }

    pub fn render(&mut self, graphics: &mut Graphics2D) {
        for block in &self.blocks {
            block.render(graphics);
        }
    }
}