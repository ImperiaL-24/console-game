use core::time::Duration;
use crate::termren::ticker::Ticker;
use crate::termren::ticker::Tickable;
use crate::termren::ticker::TickCode::{self, Success};
pub struct Scene {
    value: i32,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { value: 1 }
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl Tickable for Scene {
    fn tick(&mut self, ticker: &Ticker, delta_time: &Duration) -> TickCode {
        self.value += 1;
        Success
    }
}
