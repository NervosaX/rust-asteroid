pub mod components;
pub mod resources;
pub mod systems;

use ggez;
use ggez::event::{EventHandler, Keycode, Mod};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use specs::prelude::{Dispatcher, DispatcherBuilder, World};
use game::components::{register_components, Controlled, Position, Renderable, RenderableType,
                       Rotation, Shapes, Velocity};
use asteroid::components::Asteroid;
use player::components::Player;
use game::systems::{AssetSystem, MovementSystem, RenderingSystem, LevelsSystem};
use game::resources::{DeltaTime, PlayerInput, Window, GameWorld};
use assets::components::Asset;

use player::systems::PlayerMovementSystem;

const DESIRED_FPS: u32 = 60;

pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();

        let coords = ggez::graphics::get_screen_coordinates(ctx);

        register_components(&mut world);
        world.add_resource(GameWorld::new());
        world.add_resource(DeltaTime::default());
        world.add_resource(PlayerInput::default());
        world.add_resource(Window::new(coords.w, coords.h));

        world
            .create_entity()
            .with(Controlled)
            .with(Position::new(coords.w / 2.0, coords.h / 2.0))
            .with(Rotation::new(0.0))
            .with(Velocity::new(0.0, 0.0))
            .with(Asset::new(None))
            .with(Renderable {
                renderable_type: RenderableType::Shape(Shapes::Player(Player::new(20.0, 40.0))),
            })
            .build();

        // let asteroid = Asteroid::new(30.0);

        // world
        //     .create_entity()
        //     .with(Position::new(100.0, 100.0))
        //     .with(Velocity::new(0.0, 0.0))
        //     .with(Asset::new(None))
        //     .with(Renderable {
        //         renderable_type: RenderableType::Shape(Shapes::Asteroid(asteroid)),
        //     })
        //     .build();

        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .with(LevelsSystem, "level_system", &[])
            .with(MovementSystem, "movement_system", &[])
            .with(PlayerMovementSystem, "player_movement_system", &[])
            .build();

        Ok(Game { world, dispatcher })
    }
}

impl<'a, 'b> EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        {
            use specs::prelude::RunNow;
            let mut rs = AssetSystem::new(ctx);
            rs.run_now(&self.world.res);
        }

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / f64::from(DESIRED_FPS);
            self.world.write_resource::<DeltaTime>().delta = dt;
        }

        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            use specs::prelude::RunNow;
            let mut rs = RenderingSystem::new(ctx);
            rs.run_now(&self.world.res);
        }

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();

        if !repeat {
            match keycode {
                Keycode::Left => input.left = true,
                Keycode::Right => input.right = true,
                Keycode::Up => input.up = true,
                _ => {}
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();

        if !repeat {
            match keycode {
                Keycode::Left => input.left = false,
                Keycode::Right => input.right = false,
                Keycode::Up => input.up = false,
                _ => {}
            }
        }
    }
}
