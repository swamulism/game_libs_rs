use ecs::components::*;
use ecs::resources::*;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        use specs::Join;
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
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Controlled>,
        Read<'a, PlayerInput>,
    );

    fn run(&mut self, (mut vel, con, inp): Self::SystemData) {
        use specs::Join;
        for (vel, _) in (&mut vel, &con).join() {
            if inp.up {
                vel.y = -5.0;
            }
            else if inp.down {
                vel.y = 5.0;
            }
            else {
                vel.y = 0.0;
            }
            if inp.left {
                vel.x = -5.0;
            }
            else if inp.right {
                vel.x = 5.0;
            }
            else {
                vel.x = 0.0;
            }

        }
    }
}
