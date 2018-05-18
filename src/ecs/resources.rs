use ggez::Context;

use ggez::graphics::{draw_ex, DrawParam, Image, Point2};
use std::collections::HashMap;

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

#[derive(Default)]
pub struct Sprites {
    pub images: HashMap<String, Image>,
}

impl Sprites {
    pub fn new(images: HashMap<String, Image>) -> Self {
        Self { images }
    }
}

#[derive(Default)]
pub struct ToBeDrawn {
    pub images: Vec<(Image, f32, f32)>,
}

impl ToBeDrawn {
    pub fn new() -> Self {
        Self { images: vec![] }
    }
    pub fn draw(&self, ctx: &mut Context) {
        for (img, x, y) in &self.images {
            draw_ex(
                ctx,
                img,
                DrawParam {
                    dest: Point2::new(*x, *y),
                    ..Default::default()
                },
            ).expect("error with drawing");
        }
    }
    pub fn clear(&mut self) {
        self.images = vec![];
    }
}
