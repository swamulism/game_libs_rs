use ecs::components::*;
use ecs::resources::*;
use ecs::systems::*;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{DrawMode, Point2};
use ggez::{event, graphics, timer, Context, GameResult};
use specs::{Dispatcher, DispatcherBuilder, World};

pub struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Controlled>();
        world.add_resource(PlayerInput::new());

        world
            .create_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Controlled)
            .build();

        let dispatcher = DispatcherBuilder::new()
            .with(Control, "control", &[])
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

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();

        if !repeat {
            match keycode {
                Keycode::A => input.left = true,
                Keycode::D => input.right = true,
                Keycode::W => input.up = true,
                Keycode::S => input.down = true,
                Keycode::Escape => {
                    ctx.quit().expect("wat");
                }
                _ => (),
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();
        if !repeat {
            match keycode {
                Keycode::A => input.left = false,
                Keycode::D => input.right = false,
                Keycode::W => input.up = false,
                Keycode::S => input.down = false,
                _ => (),
            }
        }
    }
}
