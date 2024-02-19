use crate::termren::renderer::Renderer;
use crate::termren::ticker::TickCode::{self, StopTicker, Success};
use crate::termren::ticker::Tickable;

use termion::input::TermRead;

use core::time::Duration;
use termion::input::Keys;
use termion::AsyncReader;

use std::sync::Arc;
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

impl Tickable<(Arc<Mutex<Renderer>>, Arc<Mutex<InputReader>>)> for InputReader {
    fn tick(&mut self, _origin: Option<&mut (Arc<Mutex<Renderer>>, Arc<Mutex<InputReader>>)>, _delta_time: Duration) -> TickCode {
        let key_opt = self.keys.lock().unwrap().next();
        if let Some(Ok(key)) = key_opt {
            match key {
                termion::event::Key::Esc => return StopTicker,
                _ => println!("KEY: {:#?}", key),
            }
        };
        Success
    }
}
