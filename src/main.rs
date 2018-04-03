extern crate specs;
extern crate rand;
extern crate ggez;

#[macro_use]
extern crate specs_derive;

pub mod player;
pub mod asteroid;
pub mod game;

use ggez::event;
use ggez::Context;
use ggez::conf::{Conf, WindowMode, WindowSetup};

use game::Game;

fn main() {
    let config = Conf {
        window_mode: WindowMode::default().dimensions(1024, 768).vsync(true),
        window_setup: WindowSetup::default().title("Asteroids"),
        ..Default::default()
    };

    let ctx = &mut Context::load_from_conf("gg-asteroids", "ggez", config).unwrap();

    let mut state = Game::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, &mut state) {
        println!("Error: {}", e);
        std::process::exit(1);
    };
}
