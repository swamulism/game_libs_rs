//! Roguetima

extern crate ggez;
extern crate noise;
extern crate rand;
extern crate specs;

mod ecs;
mod game;

use game::MainState;
use ggez::{event, graphics};
use std::{env, path};

pub fn main() {
    let ctx = &mut ggez::ContextBuilder::new("game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Roguetima"))
        .build()
        .expect("Failed to build ggez context");

    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

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
