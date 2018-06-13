use ecs::components::*;
use ecs::resources::*;
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

/// System for updating a position of an entity
pub struct UpdatePosSys;

// Figure out how to add collision check here
impl<'a> System<'a> for UpdatePosSys {
    type SystemData = (
        ReadStorage<'a, VelocityComp>,
        WriteStorage<'a, PositionComp>,
    );

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}

/// System for updating velocity based on player input
pub struct ControlSys;

impl<'a> System<'a> for ControlSys {
    type SystemData = (
        ReadStorage<'a, ControlledComp>,
        Read<'a, InputRes>,
        WriteStorage<'a, VelocityComp>,
    );

    fn run(&mut self, (con, inp, mut vel): Self::SystemData) {
        for (_, vel) in (&con, &mut vel).join() {
            if inp.up {
                vel.y = -5.0;
            } else if inp.down {
                vel.y = 5.0;
            } else {
                vel.y = 0.0;
            }
            if inp.left {
                vel.x = -5.0;
            } else if inp.right {
                vel.x = 5.0;
            } else {
                vel.x = 0.0;
            }
        }
    }
}

/// System for loading sprites into drawing queue
pub struct LoadDrawSys;

impl<'a> System<'a> for LoadDrawSys {
    type SystemData = (
        ReadStorage<'a, PositionComp>,
        ReadStorage<'a, SpriteComp>,
        Write<'a, SpritesRes>,
    );

    fn run(&mut self, (pos, spr, mut sprites): Self::SystemData) {
        for (pos, spr) in (&pos, &spr).join() {
            sprites.push((spr.image_name.clone(), pos.x, pos.y));
        }
    }
}
