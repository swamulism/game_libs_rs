use ecs::components::*;
use ecs::resources::*;
use ecs::systems::*;
use ggez::event::{Keycode, Mod};
use ggez::{event, graphics, timer, Context, GameResult};
use specs::{Dispatcher, DispatcherBuilder, World};
use std::collections::HashMap;

pub struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        let imgs = get_images(ctx);
        world.register::<PositionComp>();
        world.register::<VelocityComp>();
        world.register::<ControlledComp>();
        world.register::<SpriteComp>();
        world.add_resource(InputRes::new());
        world.add_resource(SpritesRes::new(imgs));
        world.add_resource(DrawQueueRes::new());

        // Creating charactor
        world
            .create_entity()
            .with(PositionComp { x: 0.0, y: 0.0 })
            .with(VelocityComp { x: 0.0, y: 0.0 })
            .with(ControlledComp)
            .with(SpriteComp::new("/Platino1.png".to_string()))
            .build();

        let dispatcher = DispatcherBuilder::new()
            .with(ControlSys, "control", &[])
            .with(UpdatePosSys, "update_pos", &[])
            .build();
        let s = Self { world, dispatcher };
        Ok(s)
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_UPS: u32 = 60;

        // FPS counter
        if timer::get_ticks(ctx) % 10000 == 0 {
            println!("FPS: {}", timer::get_fps(ctx));
        }

        // Catch up on updates if fps too low
        while timer::check_update_time(ctx, DESIRED_UPS) {
            self.dispatcher.dispatch(&mut self.world.res);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::RunNow;

        // Clear screen
        graphics::clear(ctx);

        // Load data into DrawQueueRes
        LoadDrawSys.run_now(&self.world.res);

        // Draw stuff in queue
        let mut drawq = self.world.write_resource::<DrawQueueRes>();
        drawq.draw(ctx);

        // Put things on screen
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<InputRes>();
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
        let mut input = self.world.write_resource::<InputRes>();
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

/// Generate Hashmap for all images used in game
fn get_images(ctx: &mut Context) -> HashMap<String, graphics::Image> {
    let mut imgs = HashMap::new();
    let img_name = "/Platino1.png";
    let img = graphics::Image::new(ctx, img_name).expect(&format!("{}, Not found", img_name));
    imgs.insert(img_name.to_string(), img);
    return imgs;
}
