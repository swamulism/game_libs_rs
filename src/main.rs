//! Roguetima

extern crate ggez;
extern crate noise;
extern crate rand;
extern crate specs;

mod game;

use game::MainState;
use ggez::event;

pub fn main() {
    let ctx = &mut ggez::ContextBuilder::new("game", "Me")
        .window_setup(ggez::conf::WindowSetup::default().title("GAME"))
        .build()
        .expect("Failed to build ggez context");

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    }
}
