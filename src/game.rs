use ecs::components::{Position, Velocity};
use ecs::updatepos::UpdatePos;
use ggez::graphics::{DrawMode, Point2};
use ggez::{event, graphics, timer, Context, GameResult};
use specs::{Dispatcher, DispatcherBuilder, World};
// pub struct Systems {
//     pub update_pos: UpdatePos,
// }

pub struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
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

        // let systems = Systems {
        //     update_pos: UpdatePos,
        // };

        let dispatcher = DispatcherBuilder::new()
            .with(UpdatePos, "update_pos", &[])
            .build();
        let s = Self { world, dispatcher };
        Ok(s)
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_UPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_UPS) {
            self.dispatcher.dispatch(&mut self.world.res);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;
        graphics::clear(ctx);
        let positions = self.world.read_storage::<Position>();
        for entity in self.world.entities().join() {
            if let Some(pos) = positions.get(entity) {
                graphics::circle(ctx, DrawMode::Fill, Point2::new(pos.x, pos.y), 100.0, 2.0)?;
            }
        }
        graphics::present(ctx);
        Ok(())
    }
}
