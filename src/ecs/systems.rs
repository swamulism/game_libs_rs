use ecs::components::*;
use ecs::resources::*;
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}

// struct EnemySpawner;

// impl<'a> System<'a> for EnemySpawner {
//     type SystemData = Entities<'a>;

//     fn run(&mut self, entities: Entities<'a>) {
//         let enemy = entities.create();
//     }
// }

pub struct Control;
impl<'a> System<'a> for Control {
    type SystemData = (
        ReadStorage<'a, Controlled>,
        WriteStorage<'a, Velocity>,
        Read<'a, PlayerInput>,
    );

    fn run(&mut self, (con, mut vel, inp): Self::SystemData) {
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

// figure out way to put into buffer
// perhaps use resource to do this
#[derive(Default)]
pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        Read<'a, Sprites>,
        Write<'a, ToBeDrawn>,
    );

    fn run(&mut self, (pos, spr, sprites, mut tbdrawn): Self::SystemData) {
        for (pos, spr) in (&pos, &spr).join() {
            let img = sprites
                .images
                .get(&spr.image_name)
                .expect(&format!("{} not found", spr.image_name));
            tbdrawn.images.push((img.clone(), pos.x, pos.y));
        }
    }
}

// pub struct DrawSys;
// impl<'a> System<'a> for DrawSys {
//     type SystemData = (Write<'a, ToBeDrawn>);

//     fn run(&mut self, mut tbdrawn: Self::SystemData) {
//         tbdrawn.draw();
//         tbdrawn.clear();
//     }
// }
