use ggez::filesystem::resources_dir;
use glam::*;

use ggez::event;
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {}

impl MainState {
    fn new(_ctx: &Context) -> GameResult<Self> {
        Ok(Self {})
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let resources_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("drawing", "Jack").add_resource_path(resources_dir);
    let (mut ctx, events_loop) = cb.build()?;

    println!("{}", graphics::renderer_info(&ctx)?);
    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
