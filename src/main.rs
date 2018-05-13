//! Roguetima

extern crate ggez;
extern crate noise;
extern crate rand;
extern crate specs;

mod ecs;
mod game;

use ecs::components::{Position, Velocity};
use ecs::updatepos::UpdatePos;
use game::{MainState, Systems};
use ggez::event;
use specs::World;

pub fn main() {
    let ctx = &mut ggez::ContextBuilder::new("game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("GAME"))
        .build()
        .expect("Failed to build ggez context");

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();

    world
        .create_entity()
        .with(Position { x: 0.0, y: 380.0 })
        .with(Velocity { x: 5.0, y: 0.1 })
        .build();

    let systems = Systems {
        update_pos: UpdatePos,
    };
    let state = &mut MainState::new(ctx, world, systems).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    }
}
