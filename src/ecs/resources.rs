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
/// also used to draw things to screen
#[derive(Default)]
pub struct SpritesRes {
    images: HashMap<String, Image>,
    draw_queue: Vec<(String, f32, f32)>,
    draw_queue_keep: Vec<(String, f32, f32)>,
}

// figure out how to implement drawing order
impl SpritesRes {
    pub fn new(images: HashMap<String, Image>) -> Self {
        Self {
            images: images,
            draw_queue: vec![],
            draw_queue_keep: vec![],
        }
    }

    pub fn push(&mut self, img_info: (String, f32, f32)) {
        self.draw_queue.push(img_info)
    }

    pub fn push_keep(&mut self, img_info: (String, f32, f32)) {
        self.draw_queue_keep.push(img_info)
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for (img_name, x, y) in &self.draw_queue_keep {
            let img = self.images
                .get(img_name)
                .expect(&format!("{} image not found", img_name));
            draw_ex(
                ctx,
                img,
                DrawParam {
                    dest: Point2::new(*x, *y),
                    ..Default::default()
                },
            ).expect("error with drawing");
        }
        for (img_name, x, y) in &self.draw_queue {
            let img = self.images
                .get(img_name)
                .expect(&format!("{} image not found", img_name));
            draw_ex(
                ctx,
                img,
                DrawParam {
                    dest: Point2::new(*x, *y),
                    ..Default::default()
                },
            ).expect("error with drawing");
        }
        self.draw_queue.clear();
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
