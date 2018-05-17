use std::collections::HashMap;
use ggez::graphics::Image;

#[derive(Default)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

// #[derive(Default)]
pub struct Sprites {
    pub images: HashMap<String, Image>,
}

impl Sprites {
    pub fn new(images: HashMap<String, Image>) -> Self {
        Self {
            images
        }
    }
}