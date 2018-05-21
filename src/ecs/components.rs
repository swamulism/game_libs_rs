use specs::{Component, NullStorage, VecStorage};

/// Position of entity
#[derive(Debug)]
pub struct PositionComp {
    pub x: f32,
    pub y: f32,
}

impl Component for PositionComp {
    type Storage = VecStorage<Self>;
}

/// Velocity of entity
#[derive(Debug)]
pub struct VelocityComp {
    pub x: f32,
    pub y: f32,
}

impl Component for VelocityComp {
    type Storage = VecStorage<Self>;
}

/// Whether or not entity is controlled by player 
#[derive(Debug, Default)]
pub struct ControlledComp;

impl Component for ControlledComp {
    type Storage = NullStorage<Self>;
}

/// Info about sprite that entity has
#[derive(Debug)]
pub struct SpriteComp {
    pub image_name: String,
}

// Not sure how this component should be structured
// or what it should store
impl SpriteComp {
    pub fn new(name: String) -> Self {
        Self {
            image_name: name,
        }
    }
}

impl Component for SpriteComp {
    type Storage = VecStorage<Self>;
}


// #[derive(Debug)]
// pub struct Collision {
//     pub x0: f32,
//     pub y0: f32,
//     pub x1: f32,
//     pub x2: f32,
// }

// impl Component for Collision {
//     type Storage = VecStorage<Self>;
// }