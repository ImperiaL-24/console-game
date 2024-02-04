use crate::termren::ticker::Ticker;
use crate::termren::ticker::Tickable;
use crate::termren::ticker::TickCode::{self, Success, StopTicker};
use termion::input::TermRead;

use termion::input::Keys;
use core::time::Duration;
use termion::AsyncReader;
use std::sync::Mutex;

pub struct InputReader {
    keys: Mutex<Keys<AsyncReader>>,
}

impl InputReader {
    pub fn new() -> InputReader {
        let keys = Mutex::new(termion::async_stdin().keys());
        InputReader { keys }
    }
}

impl Tickable for InputReader {
    fn tick(&mut self, ticker: &Ticker, delta_time: &Duration) -> TickCode {
        let key_opt = self.keys.lock().unwrap().next();
        if let Some(Ok(key)) = key_opt {
            match key {
                termion::event::Key::Esc => return StopTicker,
                _ => println!("KEY: {:#?}", key)
            }
        };
        Success
    }
}