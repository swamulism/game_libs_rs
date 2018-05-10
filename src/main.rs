//! test for noises

extern crate ggez;
extern crate noise;
extern crate rand;

use ggez::{event, graphics, Context, GameResult};

struct MainState {}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState {};
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let ctx = &mut ggez::ContextBuilder::new("noise", "Me")
        .window_setup(ggez::conf::WindowSetup::default().title("GAME"))
        .build()
        .expect("Failed to build ggez context");

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    }
}
