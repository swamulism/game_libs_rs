use ggez::{event, graphics, Context, GameResult};

pub struct MainState {}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
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
