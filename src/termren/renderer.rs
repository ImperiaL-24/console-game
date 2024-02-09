use core::time::Duration;
use crate::component::component::Value;
use crate::component::world::Scene;
use crate::termren::ticker::Ticker;
use crate::termren::ticker::Tickable;
use crate::termren::ticker::TickCode::{self, Success};

use std::sync::Arc;
use std::sync::Mutex;

pub struct Renderer {
    world: Arc<Mutex<Scene>>,
    buffer: String
}

impl Renderer {
    pub fn new(world: &Arc<Mutex<Scene>>) -> Renderer {
        Renderer { world: world.clone(), buffer: String::new() }
    }
}

impl Tickable for Renderer {
    fn tick(&mut self, ticker: &Ticker, delta_time: &Duration) -> TickCode {

        println!(
            "{}{}{}\r",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            self.world.lock().unwrap().get_entity(&1).unwrap().get_val()
        );
        Success
    }
}
