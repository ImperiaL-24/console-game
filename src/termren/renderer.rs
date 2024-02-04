use core::time::Duration;
use crate::termren::world::Scene;
use crate::termren::ticker::Ticker;
use crate::termren::ticker::Tickable;
use crate::termren::ticker::TickCode::{self, Success};

use std::sync::Arc;
use std::sync::Mutex;

trait Renderalbe {

}

pub struct Renderer {
    world: Arc<Mutex<Scene>>,
}

impl Renderer {
    pub fn new(world: &Arc<Mutex<Scene>>) -> Renderer {
        Renderer { world: world.clone() }
    }
    //TODO: add renderalbe
}

impl Tickable for Renderer {
    fn tick(&mut self, ticker: &Ticker, delta_time: &Duration) -> TickCode {

        println!(
            "{}{}{}\r",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            self.world.lock().unwrap().get_value()
        );
        Success
    }
}
