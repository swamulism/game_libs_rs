//! Roguetima

extern crate ggez;
extern crate noise;
extern crate rand;
extern crate specs;

mod ecs;
mod game;

use game::MainState;
use ggez::event;
use std::{env, path};

pub fn main() {
    let ctx = &mut ggez::ContextBuilder::new("game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("GAME"))
        .build()
        .expect("Failed to build ggez context");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    }
}

// let mut image2_nearest = graphics::Image::new(ctx, "/shot.png")?;
// image2_nearest.set_filter(graphics::FilterMode::Nearest);
// graphics::draw_ex(
//     ctx,
//     &self.image2_nearest,
//     graphics::DrawParam {
//         dest: Point2::new(10.0, 10.0),
//         ..Default::default()
//     },
// )?;

// struct EnemySpawner;

// impl<'a> System<'a> for EnemySpawner {
//     type SystemData = Entities<'a>;

//     fn run(&mut self, entities: Entities<'a>) {
//         let enemy = entities.create();
//     }
// }
