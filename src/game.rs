use ecs::components::Position;
use ecs::updatepos::UpdatePos;
use ggez::graphics::{DrawMode, Point2};
use ggez::{event, graphics, timer, Context, GameResult};
use specs::{RunNow, World};

pub struct Systems {
    pub update_pos: UpdatePos,
}

pub struct MainState {
    world: World,
    systems: Systems,
}

impl MainState {
    pub fn new(_ctx: &mut Context, world: World, systems: Systems) -> GameResult<MainState> {
        let s = MainState { world, systems };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_UPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_UPS) {
            self.systems.update_pos.run_now(&self.world.res);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;
        graphics::clear(ctx);
        let positions = self.world.read::<Position>();
        for entity in self.world.entities().join() {
            if let Some(pos) = positions.get(entity) {
                graphics::circle(ctx, DrawMode::Fill, Point2::new(pos.x, pos.y), 100.0, 2.0)?;
            }
        }
        graphics::present(ctx);
        Ok(())
    }
}
