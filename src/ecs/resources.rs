use ggez::Context;
use ggez::graphics::{draw_ex, DrawParam, Image, Point2};
use pcg::get_noises;
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
        Self { images: images }
    }
}

/// Queue for drawing images in game every frame
#[derive(Default)]
pub struct DrawQueueRes {
    pub images: Vec<(Image, f32, f32)>,
    pub images_keep: Vec<(Image, f32, f32)>,
}

// Might have to figure out a way to choose drawing order
// so things that should be drawn on the top get drawn on the top
impl DrawQueueRes {
    pub fn new() -> Self {
        Self {
            images: vec![],
            images_keep: vec![],
        }
    }
    pub fn draw(&mut self, ctx: &mut Context) {
        for (img, x, y) in &self.images_keep {
            draw_ex(
                ctx,
                img,
                DrawParam {
                    dest: Point2::new(*x, *y),
                    ..Default::default()
                },
            ).expect("error with drawing");
        }
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

const SQUARE_SIZE: f32 = 5.0;

pub struct NoisesRes {
    noises: Vec<(String, Image)>,
    cur_noise: usize,
}

impl NoisesRes {
    pub fn new(ctx: &mut Context) -> Self {
        let tmp = get_noises();
        let mut noises = vec![];
        for (name, vector) in tmp {
            let height = vector.len() as u16;
            let width = vector[0].len() as u16;
            let mut rgba = vec![];
            for col in vector {
                for row in col {
                    let color = (255.0 * row) as u8;
                    rgba.push(color);
                    rgba.push(color);
                    rgba.push(color);
                    rgba.push(255);
                }
            }
            let img = Image::from_rgba8(ctx, width, height, rgba.as_slice())
                .expect("error building noise image");
            noises.push((name, img));
        }
        Self {
            noises: noises,
            cur_noise: 0,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        // add code to draw name here
        draw_ex(
            ctx,
            &self.noises[self.cur_noise].1,
            DrawParam {
                scale: Point2::new(SQUARE_SIZE, SQUARE_SIZE),
                // dest: Point2::new(*x, *y),
                ..Default::default()
            },
        ).expect("error with drawing");
    }

    pub fn next(&mut self) {
        let num_noises = self.noises.len();
        self.cur_noise = (self.cur_noise + 1) % num_noises;
    }

    pub fn prev(&mut self) {
        let num_noises = self.noises.len();
        if self.cur_noise == 0 {
            self.cur_noise = num_noises - 1;
        } else {
            self.cur_noise = self.cur_noise - 1;
        }
    }

    pub fn regen(&mut self, ctx: &mut Context) {
        let tmp = get_noises();
        let mut noises = vec![];
        for (name, vector) in tmp {
            let height = vector.len() as u16;
            let width = vector[0].len() as u16;
            let mut rgba = vec![];
            for col in vector {
                for row in col {
                    let color = (255.0 * row) as u8;
                    rgba.push(color);
                    rgba.push(color);
                    rgba.push(color);
                    rgba.push(255);
                }
            }
            let img = Image::from_rgba8(ctx, width, height, rgba.as_slice())
                .expect("error building noise image");
            noises.push((name, img));
        }
        self.noises = noises;
    }
}
