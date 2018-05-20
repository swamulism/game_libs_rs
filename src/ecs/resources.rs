use ggez::Context;
use ggez::graphics::{draw_ex, DrawParam, Image, Point2};
use std::collections::HashMap;

/// Used to store what input the player has put in
#[derive(Default)]
pub struct InputRes {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl InputRes {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

/// Used to store all images used in game
/// Uses flyweight pattern (hopefully)
#[derive(Default)]
pub struct SpritesRes {
    pub images: HashMap<String, Image>,
}

impl SpritesRes {
    pub fn new(images: HashMap<String, Image>) -> Self {
        Self { images }
    }
}

/// Queue for drawing images in game every frame
#[derive(Default)]
pub struct DrawQueueRes {
    pub images: Vec<(Image, f32, f32)>,
}

// Might have to figure out a way to choose drawing order
// so things that should be drawn on the top get drawn on the top
impl DrawQueueRes {
    pub fn new() -> Self {
        Self { images: vec![] }
    }
    pub fn draw(&mut self, ctx: &mut Context) {
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
        self.images.clear();
    }
}
