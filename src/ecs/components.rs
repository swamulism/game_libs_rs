use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

// use ggez::graphics;

// #[derive(Debug)]
// pub struct Sprite {
//     image: graphics::Image,
//     sprite_index: i32,
//     frames_since_last_draw: i32,
//     animation_rate: i32,
// }

// impl Component for Sprite {
//     type Storage = VecStorage<Self>;
// }
