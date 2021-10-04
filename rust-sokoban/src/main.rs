use std::path;
use specs::prelude::*;
use ggez;
use ggez::{conf, event, GameResult};

pub mod systems;
pub mod components;
pub mod constants;
pub mod entities;
pub mod map;
pub mod resources;

use crate::components::register_components;
use crate::entities::Game;
use crate::map::initialize_level;
use crate::resources::register_resources;

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;

    let game = &mut Game { world };
    event::run(context, event_loop, game)
}
