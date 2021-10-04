use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyMods};

use specs::prelude::*;

use crate::systems::{InputSystem, RenderingSystem};
use crate::resources::InputQueue;

pub struct Game {
    pub world: World,
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // todo: update draw here
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        // println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}
